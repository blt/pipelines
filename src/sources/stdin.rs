use crate::core::{self, BlockEvents, Task};
use async_trait::async_trait;
use futures::channel::mpsc;
use futures::SinkExt;
use std::mem;
use tokio::io::{self, AsyncBufReadExt};

pub struct Stdin {
    inner: io::BufReader<io::Stdin>,
    egress: mpsc::Sender<BlockEvents>,
    max_buffer: usize,
    buffer: BlockEvents,
}

impl Stdin {
    pub fn new(egress: mpsc::Sender<BlockEvents>, max_buffer: usize) -> Self {
        let inner = io::BufReader::new(io::stdin());
        let buffer = BlockEvents::with_capacity(max_buffer);
        Self {
            inner,
            egress,
            max_buffer,
            buffer,
        }
    }
}

#[async_trait]
impl Task for Stdin {
    async fn run(mut self) -> Result<(), core::Error> {
        let inner = self.inner;
        let mut lines = inner.lines();
        let mut buffer = self.buffer;

        // loop over incoming lines, buffering until we hit max_buffer and then
        // sending forward
        while let Some(l) = lines.next_line().await? {
            loop {
                match buffer.enqueue() {
                    Some(mut event) => {
                        event.line = l.into_boxed_str();
                        break;
                    }
                    None => {
                        let mut egress_buffer = BlockEvents::with_capacity(self.max_buffer);
                        mem::swap(&mut egress_buffer, &mut buffer);
                        self.egress.send(egress_buffer).await?;
                    }
                }
            }
        }
        // send any buffer that remains
        self.egress.send(buffer).await?;
        Ok(())
    }
}
