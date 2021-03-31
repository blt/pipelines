use async_trait::async_trait;
use futures::channel::mpsc;
use std::iter::Iterator;
use std::slice;
use tokio::io;

#[async_trait]
pub trait Task {
    async fn run(self) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct BlockEvents {
    capacity: usize,
    events: Vec<Event>,
}

#[derive(Debug)]
pub struct BlockIterMut<'a> {
    inner: slice::IterMut<'a, Event>,
}

#[derive(Debug)]
pub struct BlockIter<'a> {
    inner: slice::Iter<'a, Event>,
}

impl<'a> Iterator for BlockIterMut<'a> {
    type Item = &'a mut Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = &'a Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl BlockEvents {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            events: Vec::with_capacity(capacity),
        }
    }

    pub fn enqueue(&mut self) -> Option<&mut Event> {
        if self.events.len() == self.capacity {
            None
        } else {
            self.events.push(Event::default());
            self.events.last_mut()
        }
    }

    pub fn iter_mut(&mut self) -> BlockIterMut {
        BlockIterMut {
            inner: self.events.iter_mut(),
        }
    }

    pub fn iter(&self) -> BlockIter {
        BlockIter {
            inner: self.events.iter(),
        }
    }
}

#[derive(Debug, Default)]
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
