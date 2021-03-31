use futures::channel::mpsc;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use pipeline::core::{BlockEvents, Task};
use pipeline::sinks::Stdout;
use pipeline::sources::Stdin;
use pipeline::transforms::SpaceCounter;
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio::task;

async fn run() {
    let (a_snd, a_rcv): (mpsc::Sender<BlockEvents>, mpsc::Receiver<BlockEvents>) =
        mpsc::channel(128);
    let (b_snd, b_rcv) = mpsc::channel(128);

    let stdin = Stdin::new(a_snd, 4096);
    let counter = SpaceCounter::new(a_rcv, b_snd);
    let stdout = Stdout::new(b_rcv);

    let reader = task::spawn(async { stdin.run().await });
    let counter = task::spawn(async { counter.run().await });
    let writer = task::spawn(async { stdout.run().await });

    let mut workers = FuturesUnordered::new();
    workers.push(reader);
    workers.push(counter);
    workers.push(writer);

    while workers.next().await.is_some() {}
}

fn main() {
    let runtime: Runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to create async runtime");

    runtime.block_on(run());
}
