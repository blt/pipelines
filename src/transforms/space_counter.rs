use crate::core::{self, Event, Task};
use crate::str::total_spaces;
use async_trait::async_trait;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use futures::SinkExt;

pub struct SpaceCounter {
    ingress: mpsc::Receiver<Event>,
    egress: mpsc::Sender<Event>,
}

impl SpaceCounter {
    pub fn new(ingress: mpsc::Receiver<Event>, egress: mpsc::Sender<Event>) -> Self {
        Self { egress, ingress }
    }
}

#[async_trait]
impl Task for SpaceCounter {
    async fn run(mut self) -> Result<(), core::Error> {
        let mut ingress = self.ingress;
        let mut egress = self.egress;

        while let Some(mut event) = ingress.next().await {
            let spaces = total_spaces(&event.line);
            event.spaces = Some(spaces);
            egress.send(event).await?
        }
        Ok(())
    }
}
