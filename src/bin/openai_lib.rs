//! OpenAI Rust Client - 逐步构建教程
//!
//! 核心概念：
//! 1. struct - 定义数据结构
//! 2. trait - 定义接口/行为
//! 3. async/await - 异步编程
//! 4. Result<T, E> - 错误处理
//! 5. serde - JSON 序列化/反序列化

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// 第一步：定义请求/响应的数据结构 (struct)
// =============================================================================

/// 聊天消息结构体
/// 使用 derive 宏自动实现一些 trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// role: system/user/assistant/tool
    pub role: String,
    pub content: String,
}

impl Message {
    /// 关联函数 (类似 Python 的 @classmethod)
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

/// 聊天请求体
/// Serialize 表示可以序列化为 JSON
#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    /// 模型名称，如 "gpt-4o", "gpt-3.5-turbo"
    pub model: String,

    /// 消息列表
    pub messages: Vec<Message>,

    /// 可选参数：temperature (默认 None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// 是否流式输出
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl ChatRequest {
    /// 构建器模式 (Builder Pattern) - Rust 中非常常见
    pub fn new(model: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            model: model.into(),
            messages,
            temperature: None,
            max_tokens: None,
            stream: None,
        }
    }

    /// 链式调用设置 temperature
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    pub fn stream(mut self, enable: bool) -> Self {
        self.stream = Some(enable);
        self
    }
}

// =============================================================================
// 第二步：定义响应数据结构
// =============================================================================

/// 聊天响应
/// Deserialize 表示可以从 JSON 反序列化
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: u32,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: u32,
    #[serde(rename = "total_tokens")]
    pub total_tokens: u32,
}

// =============================================================================
// 第三步：自定义错误类型 (Rust 的错误处理哲学)
// =============================================================================

/// 定义我们自己的错误类型
/// 使用 thiserror 可以简化错误定义，这里先展示手动实现
#[derive(Debug)]
pub enum OpenAIError {
    /// HTTP 请求错误
    HttpError(String),
    /// API 返回的错误
    ApiError { code: String, message: String },
    /// JSON 解析错误
    SerializationError(String),
    /// 配置错误（如缺少 API Key）
    ConfigError(String),
}

/// 实现 std::error::Error trait
impl std::error::Error for OpenAIError {}

/// 实现 Display trait，定义错误信息的显示格式
impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenAIError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            OpenAIError::ApiError { code, message } => {
                write!(f, "API error [{}]: {}", code, message)
            }
            OpenAIError::SerializationError(msg) => write!(f, "JSON error: {}", msg),
            OpenAIError::ConfigError(msg) => write!(f, "Config error: {}", msg),
        }
    }
}

/// 从 reqwest::Error 转换到 OpenAIError
/// 这样可以用 ? 操作符自动转换错误
impl From<reqwest::Error> for OpenAIError {
    fn from(err: reqwest::Error) -> Self {
        OpenAIError::HttpError(err.to_string())
    }
}

/// 从 serde_json::Error 转换
impl From<serde_json::Error> for OpenAIError {
    fn from(err: serde_json::Error) -> Self {
        OpenAIError::SerializationError(err.to_string())
    }
}

// =============================================================================
// 第四步：定义客户端 trait（接口抽象）
// =============================================================================

/// 定义 LLM 客户端的行为接口
/// trait 类似于 Python 的抽象基类或接口
#[async_trait::async_trait]
pub trait LLMClient {
    /// 异步聊天完成
    /// async fn 表示异步函数
    /// Result<T, E> 表示可能失败，成功返回 T，失败返回 E
    async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse, OpenAIError>;

    /// 流式聊天（返回 SSE 流）
    /// Box<dyn Stream> 是 trait object，类似 Python 的生成器
    async fn chat_completion_stream(
        &self,
        request: ChatRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<String, OpenAIError>> + Send>, OpenAIError>;
}

// =============================================================================
// 第五步：实现 OpenAI 客户端
// =============================================================================

pub struct OpenAIClient {
    /// HTTP 客户端
    client: reqwest::Client,
    /// API 基础 URL
    base_url: String,
    /// API 密钥
    api_key: String,
}

impl OpenAIClient {
    /// 构造函数
    /// 关联函数，没有 &self 参数
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: api_key.into(),
        }
    }

    /// 带自定义 base_url 的构造函数（用于兼容其他 API）
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }
}

#[async_trait::async_trait]
impl LLMClient for OpenAIClient {
    async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse, OpenAIError> {
        // 构建请求 URL
        let url = format!("{}/chat/completions", self.base_url);

        // 发送 HTTP POST 请求
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request) // serde 自动序列化
            .send()
            .await?; // ? 自动转换 reqwest::Error -> OpenAIError

        // 检查 HTTP 状态码
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(OpenAIError::ApiError {
                code: status.to_string(),
                message: text,
            });
        }

        // 解析 JSON 响应
        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    }

    async fn chat_completion_stream(
        &self,
        request: ChatRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<String, OpenAIError>> + Send>, OpenAIError>
    {
        // 流式处理涉及更复杂的 SSE 解析
        // 这里先返回一个未实现的占位
        todo!("流式响应将在下一步实现")
    }
}

// =============================================================================
// 第六步：使用示例（main 函数）
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取 API key
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| "请设置 OPENAI_API_KEY 环境变量")?;

    // 创建客户端
    let client =
        OpenAIClient::with_base_url(api_key, "https://dashscope.aliyuncs.com/compatible-mode/v1");

    // 构建请求
    let request = ChatRequest::new(
        "qwen3-max",
        vec![
            Message::system("你是一个有帮助的助手。"),
            Message::user("你好，请用 Rust 写一个 Hello World"),
        ],
    )
    .temperature(0.7);

    // 发送请求并处理结果
    println!("正在发送请求...\n");

    match client.chat_completion(request).await {
        Ok(response) => {
            println!("=== 响应成功 ===");
            println!("模型: {}", response.model);
            println!("使用 Token: {:?}", response.usage);

            for choice in &response.choices {
                println!("\n[{}] {}", choice.message.role, choice.message.content);
            }
        }
        Err(e) => {
            eprintln!("错误: {}", e);
        }
    }

    Ok(())
}

// =============================================================================
// Rust 语言特性总结
// =============================================================================

/*
1. **Ownership & Borrowing**
   - String vs &str: String 拥有数据，&str 是借用
   - impl Into<String> 接受任何可转换为 String 的类型

2. **Option<T> & Result<T, E>**
   - 显式处理可能为空/失败的情况
   - ? 操作符简化错误传播

3. **trait**
   - 定义共享行为，类似接口
   - derive 宏自动实现常用 trait

4. **泛型 (Generics)**
   - impl Into<String> 中的 Into 是一个 trait
   - Vec<T>, Option<T> 中的 T 是类型参数

5. **Builder Pattern**
   - 链式调用构造复杂对象
   - method(self) -> Self 实现流畅接口

6. **async/await**
   - 异步编程不阻塞线程
   - 需要运行时如 tokio

7. **宏 (Macros)**
   - #[derive(...)] 自动生成代码
   - #[serde(rename = "...")] 自定义序列化名称
*/
