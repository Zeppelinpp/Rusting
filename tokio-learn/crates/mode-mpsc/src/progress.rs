use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::future::Future;
use std::time::Duration;

pub trait ProgressExt: Clone + Send + Sync + 'static {
    fn new(total: usize) -> Self;

    fn task<Fut, N>(
        &self,
        name: N,
        fut: Fut,
    ) -> impl Future<Output = Fut::Output> + Send
    where
        Fut: Future + Send,
        N: Into<String> + Send;

    fn println<M>(&self, msg: M)
    where
        M: AsRef<str>;

    fn finish(&self);
}

#[derive(Clone)]
pub struct ProgressContext {
    multi: MultiProgress,
    total: ProgressBar,
}

impl ProgressExt for ProgressContext {
    fn new(total: usize) -> Self {
        let multi = MultiProgress::new();
        let total_pb = multi.add(ProgressBar::new(total as u64));
        total_pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
                .unwrap()
                .progress_chars("=>-"),
        );
        total_pb.set_message("Processing...");

        Self {
            multi,
            total: total_pb,
        }
    }

    fn task<Fut, N>(
        &self,
        name: N,
        fut: Fut,
    ) -> impl Future<Output = Fut::Output> + Send
    where
        Fut: Future + Send,
        N: Into<String> + Send,
    {
        let name = name.into();
        let multi = self.multi.clone();
        let total = self.total.clone();

        async move {
            let pb = multi.add(ProgressBar::new_spinner());
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_message(name);

            let result = fut.await;
            pb.finish_and_clear();
            total.inc(1);
            result
        }
    }

    fn println<M>(&self, msg: M)
    where
        M: AsRef<str>,
    {
        self.multi.println(msg.as_ref()).ok();
    }

    fn finish(&self) {
        self.total.finish_with_message("Done");
    }
}
