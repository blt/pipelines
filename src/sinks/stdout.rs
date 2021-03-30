use crate::core::{self, Event, Task};
use crate::str::get_header;
use async_trait::async_trait;
use futures::channel::mpsc;
use futures::StreamExt;
use tokio::io::{self, AsyncWriteExt};

pub struct Stdout {
    ingress: mpsc::Receiver<Event>,
    egress: io::BufWriter<io::Stdout>,
}

impl Stdout {
    pub fn new(ingress: mpsc::Receiver<Event>) -> Self {
        let egress = io::BufWriter::new(io::stdout());
        Self { ingress, egress }
    }
}

#[async_trait]
impl Task for Stdout {
    async fn run(self) -> Result<(), core::Error> {
        let mut ingress = self.ingress;
        let mut egress = self.egress;

        while let Some(event) = ingress.next().await {
            let header = get_header(event.spaces.unwrap_or(0) as usize);
            egress.write_all(header).await?;
            egress.write_all(event.line.as_ref().as_bytes()).await?;
            egress.write_all(b"\n").await?
        }
        Ok(())
    }
}
