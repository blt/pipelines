use async_trait::async_trait;
use tokio::io;
use tokio::sync::mpsc;

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
    Send(mpsc::error::SendError<Event>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<mpsc::error::SendError<Event>> for Error {
    fn from(err: mpsc::error::SendError<Event>) -> Self {
        Error::Send(err)
    }
}
