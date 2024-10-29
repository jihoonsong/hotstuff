use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::{sleep, Duration, Instant, Sleep};

pub struct Timeout {
    timer: Pin<Box<Sleep>>,
    duration: Duration,
}

impl Timeout {
    pub fn new(timeout: u64) -> Self {
        let duration = Duration::from_millis(timeout);
        let timer = Box::pin(sleep(duration));

        Self { timer, duration }
    }

    pub fn reset(&mut self) {
        self.timer.as_mut().reset(Instant::now() + self.duration);
    }
}

impl Future for Timeout {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.timer.as_mut().poll(cx)
    }
}
