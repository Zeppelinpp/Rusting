use crate::transform::{transform, TransformationType};
use tokio::sync::{mpsc, Semaphore};
use std::sync::Arc;
use tracing::{error, info};

pub struct Job {
    pub in_path: String,
    pub out_path: String,
}

pub async fn run_batch(jobs: Vec<Job>, worker_count: usize) {
    let (tx, mut rx) = mpsc::channel::<Job>(100);

    // tokio::sync::mpsc is multi-producer, single-consumer.
    // We use one dispatcher task to recv jobs from the channel,
    // then spawn transform tasks limited by a Semaphore.
    let semaphore = Arc::new(Semaphore::new(worker_count));

    let dispatcher = tokio::spawn(async move {
        let mut handles = Vec::new();

        while let Some(job) = rx.recv().await {
            let permit = semaphore
                .clone()
                .acquire_owned()
                .await
                .expect("semaphore closed");

            let handle = tokio::spawn(async move {
                let _permit = permit; // hold permit until transform completes

                let result = transform(
                    &job.in_path,
                    &job.out_path,
                    TransformationType::Vidoe2Wav,
                )
                .await;

                match result {
                    Ok(r) => info!("Done: {} -> {}", r.in_path, r.out_path),
                    Err(e) => error!("Failed {}: {}", job.in_path, e),
                }
            });

            handles.push(handle);
        }

        // Wait for all in-flight transforms to finish
        for h in handles {
            let _ = h.await;
        }
    });

    // Producer: send all jobs
    for job in jobs {
        if tx.send(job).await.is_err() {
            break; // dispatcher dropped
        }
    }
    drop(tx);

    // Wait for dispatcher to finish
    let _ = dispatcher.await;
}
