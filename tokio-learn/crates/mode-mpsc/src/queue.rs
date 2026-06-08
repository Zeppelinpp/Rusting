use crate::progress::{ProgressContext, ProgressExt};
use crate::transform::{TransformationType, transform};
use mode_mpsc_macros::with_progress;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};

pub struct Job {
    pub in_path: String,
    pub out_path: String,
}

#[with_progress(jobs.len(), ProgressContext)]
pub async fn run_batch(jobs: Vec<Job>, worker_count: usize) {
    let (tx, mut rx) = mpsc::channel::<Job>(100);
    let semaphore = Arc::new(Semaphore::new(worker_count));

    let dispatcher = tokio::spawn({
        let ctx: ProgressContext = __ctx.clone();
        async move {
            let mut handles = Vec::new();

            while let Some(job) = rx.recv().await {
                let permit = semaphore
                    .clone()
                    .acquire_owned()
                    .await
                    .expect("semaphore closed");
                let ctx = ctx.clone();

                let handle = tokio::spawn(async move {
                    let _permit = permit;

                    let result = ctx
                        .task(format!("Processing {}", job.in_path), async {
                            transform(&job.in_path, &job.out_path, TransformationType::Vidoe2Wav)
                                .await
                        })
                        .await;

                    if let Err(e) = &result {
                        ctx.println(format!("✗ {} failed: {}", job.in_path, e));
                    }
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
    __ctx.finish();
}
