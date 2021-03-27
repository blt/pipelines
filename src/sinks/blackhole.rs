use crate::core::{self, Event, Task};
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct Blackhole {
    ingress: mpsc::Receiver<Event>,
}

impl Blackhole {
    pub fn new(ingress: mpsc::Receiver<Event>) -> Self {
        Self { ingress }
    }
}

#[async_trait]
impl Task for Blackhole {
    async fn run(self) -> Result<(), core::Error> {
        let mut ingress = self.ingress;
        while let Some(_event) = ingress.recv().await {}
        Ok(())
    }
}
