use futures::stream::FuturesUnordered;
use futures::StreamExt;
use pipeline::core::{Event, Task};
use pipeline::sinks::Blackhole;
use pipeline::sources::Stdin;
use pipeline::transforms::SpaceCounter;
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::task;

async fn run() {
    let (a_snd, a_rcv): (mpsc::Sender<Event>, mpsc::Receiver<Event>) = mpsc::channel(100);
    let (b_snd, b_rcv) = mpsc::channel(100);

    let stdin = Stdin::new(a_snd);
    let counter = SpaceCounter::new(a_rcv, b_snd);
    let blackhole = Blackhole::new(b_rcv);

    let reader = task::spawn(async { stdin.run().await });
    let counter = task::spawn(async { counter.run().await });
    let blackhole = task::spawn(async { blackhole.run().await });

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
