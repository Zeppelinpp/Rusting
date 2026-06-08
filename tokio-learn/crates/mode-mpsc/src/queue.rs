use crate::transform::{TransformationType, transform};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{sync::Arc, time::Duration};
use tokio::sync::{Semaphore, mpsc};

pub struct Job {
    pub in_path: String,
    pub out_path: String,
}

pub async fn run_batch(jobs: Vec<Job>, worker_count: usize) {
    let total = jobs.len() as u64;
    let multi = Arc::new(MultiProgress::new());

    // 顶部总进度条：最先 add，因此固定在 index 0
    let total_pb = multi.add(ProgressBar::new(total));
    total_pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("--"),
    );
    total_pb.set_message("Converting...");

    let (tx, mut rx) = mpsc::channel::<Job>(100);
    let semaphore = Arc::new(Semaphore::new(worker_count));

    let dispatcher = tokio::spawn({
        let multi = Arc::clone(&multi);
        let total_pb = total_pb.clone();
        async move {
            let mut handles = Vec::new();

            while let Some(job) = rx.recv().await {
                let permit = semaphore
                    .clone()
                    .acquire_owned()
                    .await
                    .expect("semaphore closed");
                let total_pb = total_pb.clone();
                let multi = Arc::clone(&multi);

                let handle = tokio::spawn(async move {
                    let _permit = permit;

                    // 每个 job 一个 spinner bar，左侧自动转圈
                    let job_pb = multi.add(ProgressBar::new_spinner());
                    job_pb.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap(),
                    );
                    job_pb.enable_steady_tick(Duration::from_millis(80));
                    job_pb.set_message(format!("Processing {}", job.in_path));

                    let result =
                        transform(&job.in_path, &job.out_path, TransformationType::Vidoe2Wav).await;
                    match result {
                        Ok(_) => {
                            job_pb.finish_and_clear();
                        }
                        Err(e) => {
                            job_pb.finish_with_message(format!("✗ {} failed: {}", job.in_path, e));
                        }
                    }

                    total_pb.inc(1);
                });

                handles.push(handle);
            }

            for h in handles {
                let _ = h.await;
            }
        }
    });

    for job in jobs {
        if tx.send(job).await.is_err() {
            break;
        }
    }
    drop(tx);

    let _ = dispatcher.await;
    total_pb.finish_with_message("Done");
}
