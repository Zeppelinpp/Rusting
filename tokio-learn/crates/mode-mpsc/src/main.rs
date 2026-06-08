use mode_mpsc::queue::{Job, run_batch};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let jobs_count = 200;
    let mut jobs = Vec::with_capacity(jobs_count);
    for i in 0..jobs_count {
        let job = Job {
            in_path: "tests/test.mp4".to_string(),
            out_path: format!("tests/output_{}.wav", i),
        };
        jobs.push(job);
    }

    run_batch(jobs, 10).await;
}
