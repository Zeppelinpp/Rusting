use mode_mpsc::queue::{Job, run_batch};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Test 80 jobs
    let jobs_count = 80;
    let mut jobs = Vec::with_capacity(jobs_count);
    for i in 0..jobs_count {
        let job = Job {
            in_path: format!("tests/test{}.mp4", i),
            out_path: format!("tests/test{}.wav", i),
        };
        jobs.push(job);
    }

    run_batch(jobs, 10).await;
}
