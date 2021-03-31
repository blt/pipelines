use crate::core::{self, BlockEvents, Task};
use crate::str::total_spaces;
use async_trait::async_trait;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use futures::SinkExt;
use tokio::task;

pub struct SpaceCounter {
    ingress: mpsc::Receiver<BlockEvents>,
    egress: mpsc::Sender<BlockEvents>,
}

impl SpaceCounter {
    pub fn new(ingress: mpsc::Receiver<BlockEvents>, egress: mpsc::Sender<BlockEvents>) -> Self {
        Self { ingress, egress }
    }
}

#[async_trait]
impl Task for SpaceCounter {
    async fn run(mut self) -> Result<(), core::Error> {
        let mut ingress = self.ingress;
        let egress = self.egress;

        while let Some(mut events) = ingress.next().await {
            let mut task_egress = egress.clone();
            task::spawn(async move {
                events.iter_mut().for_each(|event| {
                    event.spaces = Some(total_spaces(&event.line));
                });
                task_egress
                    .send(events)
                    .await
                    .expect("unsure what to do with error")
            });
        }
        Ok(())
    }
}
