//! 验证 tokio 的多线程行为

use std::thread;
use std::time::Duration;

/// 获取当前线程的 ID
fn thread_id() -> String {
    format!("{:?}", thread::current().id())
}

/// 模拟一个耗时的异步操作（非阻塞）
async fn async_task(name: &str, secs: u64) -> String {
    println!(
        "[{}] 任务 {} 开始，线程: {}",
        chrono::Local::now().format("%H:%M:%S.%3f"),
        name,
        thread_id()
    );

    // tokio::time::sleep 是非阻塞的，让出线程给其他任务
    tokio::time::sleep(Duration::from_secs(secs)).await;

    println!(
        "[{}] 任务 {} 结束，线程: {}",
        chrono::Local::now().format("%H:%M:%S.%3f"),
        name,
        thread_id()
    );

    format!("任务 {} 完成", name)
}

/// 模拟阻塞操作（会卡住线程！）
fn blocking_operation(name: &str) {
    println!(
        "[{}] 阻塞操作 {} 开始，线程: {}",
        chrono::Local::now().format("%H:%M:%S.%3f"),
        name,
        thread_id()
    );

    // std::thread::sleep 会阻塞整个线程！
    thread::sleep(Duration::from_secs(2));

    println!(
        "[{}] 阻塞操作 {} 结束，线程: {}",
        chrono::Local::now().format("%H:%M:%S.%3f"),
        name,
        thread_id()
    );
}

#[tokio::main]
async fn main() {
    println!("=== Tokio 多线程测试 ===\n");

    // 测试1: 单线程内的多个异步任务（协作式调度）
    println!("--- 测试1: 顺序 await（单线程）---");
    let start = std::time::Instant::now();

    // 顺序执行，不会并行
    let r1 = async_task("A", 1).await;
    let r2 = async_task("B", 1).await;

    println!("耗时: {:?}\n", start.elapsed()); // 约 2秒

    // 测试2: 使用 tokio::spawn 创建并行任务（多线程！）
    println!("--- 测试2: tokio::spawn（真正的并行）---");
    let start = std::time::Instant::now();

    // spawn 会在工作线程池中分配任务
    let handle1 = tokio::spawn(async_task("C", 1));
    let handle2 = tokio::spawn(async_task("D", 1));

    // 等待两个任务都完成
    let (r1, r2) = tokio::join!(handle1, handle2);
    println!("结果: {:?}, {:?}", r1.unwrap(), r2.unwrap());
    println!("耗时: {:?}\n", start.elapsed()); // 约 1秒（并行！）

    // 测试3: 大量任务看线程分配
    println!("--- 测试3: 大量任务的线程分配 ---");
    let mut handles = vec![];

    for i in 0..10 {
        handles.push(tokio::spawn(async move {
            println!("任务 {} 运行在线程: {}", i, thread_id());
            tokio::time::sleep(Duration::from_millis(100)).await;
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    // 测试4: spawn_blocking - 专门处理阻塞操作
    println!("\n--- 测试4: spawn_blocking（独立的阻塞线程池）---");
    let start = std::time::Instant::now();

    let blocking_handle = tokio::task::spawn_blocking(|| {
        blocking_operation("文件IO");
        "阻塞操作完成"
    });

    // 同时执行异步任务
    let async_handle = tokio::spawn(async_task("E", 1));

    let (r1, r2) = tokio::join!(blocking_handle, async_handle);
    println!("结果: {:?}, {:?}", r1.unwrap(), r2.unwrap());
    println!("耗时: {:?}", start.elapsed());

    println!("\n=== 总结 ===");
    println!("当前运行时使用的线程数: 默认是 CPU 核心数（通常是 {}）", num_cpus::get());
}

/*
运行结果示例：

=== Tokio 多线程测试 ===

--- 测试1: 顺序 await（单线程）---
[14:30:00.123] 任务 A 开始，线程: ThreadId(1)
[14:30:01.123] 任务 A 结束，线程: ThreadId(1)
[14:30:01.123] 任务 B 开始，线程: ThreadId(1)
[14:30:02.123] 任务 B 结束，线程: ThreadId(1)
耗时: 2.00s

--- 测试2: tokio::spawn（真正的并行）---
[14:30:02.124] 任务 C 开始，线程: ThreadId(2)  <-- 不同线程！
[14:30:02.124] 任务 D 开始，线程: ThreadId(3)  <-- 不同线程！
[14:30:03.124] 任务 C 结束，线程: ThreadId(2)
[14:30:03.124] 任务 D 结束，线程: ThreadId(3)
耗时: 1.00s  <-- 并行所以快一倍！

--- 测试3: 大量任务的线程分配 ---
任务 0 运行在线程: ThreadId(2)
任务 1 运行在线程: ThreadId(3)
任务 2 运行在线程: ThreadId(4)
...

关键结论：
1. tokio::main 默认使用多线程运行时（线程数 = CPU 核心数）
2. 不加 spawn 的顺序 await 不会并行（协作式调度）
3. tokio::spawn 才会把工作分配到不同线程真正并行
4. spawn_blocking 使用独立的线程池处理阻塞操作
*/
