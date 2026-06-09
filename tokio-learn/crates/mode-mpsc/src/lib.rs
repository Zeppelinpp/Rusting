use crate::{
    queue::{Job, ServiceHandle, spawn_service},
    transform::TransformationType,
};
use std::sync::OnceLock;

pub mod progress;
pub mod queue;
pub mod transform;

static SERVICE: OnceLock<ServiceHandle> = OnceLock::new();

pub fn init_service(worker_count: usize) {
    SERVICE.get_or_init(|| spawn_service(worker_count));
}

// Public API
pub async fn process_batch(
    input_paths: Vec<impl Into<String>>,
    transformation_type: TransformationType,
) {
    let jobs = input_paths
        .into_iter()
        .map(|path| Job::new(path, None, Some(transformation_type.clone())))
        .collect::<Vec<_>>();
    if let Some(svc) = SERVICE.get() {
        svc.submit(jobs).await;
    }
}

pub fn shutdown_service() {
    if let Some(svc) = SERVICE.get() {
        svc.shutdown();
    }
}
