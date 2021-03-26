use futures::stream::FuturesUnordered;
use futures::StreamExt;
use tokio::io::{self, AsyncBufReadExt};
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug)]
struct Event {
    line: Box<str>,
    spaces: Option<usize>,
}

enum Error {
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

async fn reader<R>(rdr: R, egress: mpsc::Sender<Event>) -> Result<(), Error>
where
    R: io::AsyncBufRead + Unpin,
{
    let mut lines = rdr.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let event = Event {
            line: line.into_boxed_str(),
            spaces: None,
        };

        egress.send(event).await?
    }
    Ok(())
}

async fn space_counter(
    mut ingress: mpsc::Receiver<Event>,
    egress: mpsc::Sender<Event>,
) -> Result<(), Error> {
    while let Some(mut event) = ingress.recv().await {
        let spaces = event
            .line
            .chars()
            .fold(0, |acc, c| if c == ' ' { acc + 1 } else { acc });

        event.spaces = Some(spaces);

        egress.send(event).await?
    }
    Ok(())
}

async fn blackhole(mut ingress: mpsc::Receiver<Event>) -> Result<(), Error> {
    while let Some(_event) = ingress.recv().await {}
    Ok(())
}

async fn run() {
    let stdin = io::BufReader::new(io::stdin());

    let (a_snd, a_rcv) = mpsc::channel(100);
    let (b_snd, b_rcv) = mpsc::channel(100);

    let reader = task::spawn(reader(stdin, a_snd));
    let counter = task::spawn(space_counter(a_rcv, b_snd));
    let blackhole = task::spawn(blackhole(b_rcv));

    let mut workers = FuturesUnordered::new();
    workers.push(reader);
    workers.push(counter);
    workers.push(blackhole);

    while workers.next().await.is_some() {}
}

fn main() {
    let runtime: Runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to create async runtime");

    runtime.block_on(run());
}
