# Tokio - MPSC(Multiple Producer, Single Consumer)

## 通信模式
多个异步任务往一个队列中发送消息，另一个异步任务负责消费消息
```text
worker1 -> tx
worker2 -> tx
worker3 -> tx

collector <- rx
```
## Core API
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<T>(capacity);
```
- `tx`: Sender 发送端, 可以 `clone`
- `rx`: Receiver 接收端, 只能有一个

```rust
// Send
tx.send(value).await?;

// Recv
while let Some(value) = rx.recv().await {
    // 处理 value
}
```
