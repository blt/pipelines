use crate::core::{self, Event, Task};
use async_trait::async_trait;
use futures::future;
use futures::StreamExt;
use tokio::io::{self, AsyncBufReadExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::LinesStream;

pub struct Stdin {
    inner: LinesStream<io::BufReader<io::Stdin>>,
    egress: mpsc::Sender<Event>,
}

impl Stdin {
    pub fn new(egress: mpsc::Sender<Event>) -> Self {
        let stdin = io::BufReader::new(io::stdin());
        let lines = LinesStream::new(stdin.lines());

        Self {
            inner: lines,
            egress,
        }
    }
}

#[async_trait]
impl Task for Stdin {
    async fn run(self) -> Result<(), core::Error> {
        let mut events = self
            .inner
            .filter_map(|l| future::ready(l.ok()))
            .map(|l| Event {
                line: l.into_boxed_str(),
                spaces: None,
            });

        while let Some(event) = events.next().await {
            self.egress.send(event).await?;
        }
        Ok(())
    }
}
