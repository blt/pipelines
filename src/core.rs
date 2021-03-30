use async_trait::async_trait;
use futures::channel::mpsc;
use tokio::io;

#[async_trait]
pub trait Task {
    async fn run(self) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct Event {
    pub line: Box<str>,
    pub spaces: Option<u32>,
}

pub enum Error {
    Io(io::Error),
    Send(mpsc::SendError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<mpsc::SendError> for Error {
    fn from(err: mpsc::SendError) -> Self {
        Error::Send(err)
    }
}
