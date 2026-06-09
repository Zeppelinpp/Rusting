use crate::progress::{ProgressContext, ProgressExt};
use crate::transform::{TransformationType, transform};
use mode_mpsc_macros::with_progress;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc, watch};

pub struct Job {
    pub in_path: String,
    pub out_path: String,
    pub transformation_type: TransformationType,
}

impl Job {
    pub fn new(
        in_path: impl Into<String>,
        out_path: Option<String>,
        transformation_type: Option<TransformationType>,
    ) -> Self {
        let in_path = in_path.into();
        let out_path = out_path.unwrap_or_else(|| {
            in_path
                .rsplit_once('.')
                .map(|(stem, _)| format!("{stem}.wav"))
                .unwrap_or_else(|| format!("{in_path}.wav"))
        });
        Self {
            in_path,
            out_path,
            transformation_type: transformation_type.unwrap_or_default(),
        }
    }
}

pub struct ServiceHandle {
    pub tx: mpsc::Sender<Job>,
    shutdown_tx: watch::Sender<bool>,
}

impl ServiceHandle {
    pub async fn submit(&self, jobs: Vec<Job>) {
        for job in jobs {
            if self.tx.send(job).await.is_err() {
                break;
            }
        }
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(true);
    }
}

pub fn spawn_service(worker_count: usize) -> ServiceHandle {
    let (tx, mut rx) = mpsc::channel::<Job>(100);
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    tokio::spawn(async move {
        let semaphore = Arc::new(Semaphore::new(worker_count));
        let mut handles = vec![];
        loop {
            tokio::select! {
                biased;
                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        break;
                    }
                }
                Some(job) = rx.recv() => {
                    let permit = semaphore.clone().acquire_owned().await.expect("semaphore closed");
                    let handle = tokio::spawn(async move {
                        let _permit = permit;
                        let _ = transform(&job.in_path, &job.out_path, TransformationType::Video2Wav).await;
                    });
                    handles.push(handle);
                }
            }
        }
        while let Ok(job) = rx.try_recv() {
            let permit = semaphore
                .clone()
                .acquire_owned()
                .await
                .expect("semaphore closed");
            let handle = tokio::spawn(async move {
                let _permit = permit;
                let _ = transform(&job.in_path, &job.out_path, TransformationType::Video2Wav).await;
            });
            handles.push(handle);
        }
        for h in handles {
            let _ = h.await;
        }
    });
    ServiceHandle { tx, shutdown_tx }
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
                            transform(&job.in_path, &job.out_path, TransformationType::Video2Wav)
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
