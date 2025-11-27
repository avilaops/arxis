//! Async Runtime Example
//!
//! Demonstrates the custom async runtime with:
//! - I/O reactor (epoll/kqueue/IOCP)
//! - Timer wheel for timeouts
//! - Thread pool for task execution

use avx_http::runtime;
use std::time::Duration;

async fn delayed_print(msg: &str, delay_ms: u64) {
    println!("[START] {}", msg);
    runtime::sleep(Duration::from_millis(delay_ms)).await;
    println!("[DONE]  {} (after {}ms)", msg, delay_ms);
}

async fn compute_task(id: usize) {
    println!("Task {} starting...", id);

    // Simulate some work
    for i in 0..3 {
        println!("Task {} - iteration {}", id, i);
        runtime::sleep(Duration::from_millis(100)).await;
    }

    println!("Task {} completed!", id);
}

async fn parallel_tasks() {
    println!("\n=== Parallel Task Execution ===\n");

    // Spawn multiple tasks
    runtime::spawn(compute_task(1));
    runtime::spawn(compute_task(2));
    runtime::spawn(compute_task(3));

    // Wait a bit for tasks to complete
    runtime::sleep(Duration::from_secs(2)).await;
}

async fn timer_cascade() {
    println!("\n=== Timer Cascade Test ===\n");

    // Schedule timers at different delays
    delayed_print("Quick task (10ms)", 10).await;
    delayed_print("Medium task (100ms)", 100).await;
    delayed_print("Slow task (500ms)", 500).await;
}

async fn main_async() {
    println!("AVX-HTTP Async Runtime Demo");
    println!("============================\n");

    // Timer cascade
    timer_cascade().await;

    // Parallel execution
    parallel_tasks().await;

    println!("\n=== All tasks completed! ===");
}

fn main() {
    // Block on the main async function
    runtime::block_on(main_async());
}
