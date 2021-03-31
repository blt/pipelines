use pipeline::str::{get_header, total_spaces};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio::task;

async fn inner(
    stdin: io::BufReader<io::Stdin>,
    mut stdout: io::BufWriter<io::Stdout>,
) -> Result<(), io::Error> {
    let mut lines = stdin.lines();
    while let Some(l) = lines.next_line().await? {
        let spaces = total_spaces(&l);
        let header = get_header(spaces as usize);
        stdout.write_all(header).await?;
        stdout.write_all(l.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
    }
    Ok(())
}

async fn run() {
    let stdin = io::BufReader::new(io::stdin());
    let stdout = io::BufWriter::new(io::stdout());

    let inner = task::spawn(inner(stdin, stdout));
    let _ = inner.await.expect("should not fail");
}

fn main() {
    let runtime: Runtime = runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .expect("Unable to create async runtime");

    runtime.block_on(run());
}
