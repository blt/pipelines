use crate::core::{self, Event, Task};
use async_trait::async_trait;
use futures::channel::mpsc;
use futures::SinkExt;
use tokio::io::{self, AsyncBufReadExt};

pub struct Stdin {
    inner: io::BufReader<io::Stdin>,
    egress: mpsc::Sender<Event>,
}

impl Stdin {
    pub fn new(egress: mpsc::Sender<Event>) -> Self {
        let inner = io::BufReader::new(io::stdin());

        Self { inner, egress }
    }
}

#[async_trait]
impl Task for Stdin {
    async fn run(mut self) -> Result<(), core::Error> {
        let inner = self.inner;
        let mut lines = inner.lines();

        while let Some(l) = lines.next_line().await? {
            let event = Event {
                line: l.into_boxed_str(),
                spaces: None,
            };
            self.egress.send(event).await?;
        }
        Ok(())
    }
}
