use crate::core::{self, Event, Task};
use async_trait::async_trait;
use tokio::sync::mpsc;

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
    async fn run(self) -> Result<(), core::Error> {
        let mut ingress = self.ingress;

        while let Some(mut event) = ingress.recv().await {
            let spaces = event
                .line
                .chars()
                .fold(0, |acc, c| if c == ' ' { acc + 1 } else { acc });
            event.spaces = Some(spaces);
            self.egress.send(event).await?
        }
        Ok(())
    }
}
