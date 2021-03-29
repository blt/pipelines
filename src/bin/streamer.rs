use pipeline::core::Event;
use pipeline::str::{get_header, total_spaces};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

async fn run() -> Result<(), std::io::Error> {
    let stdin = io::BufReader::new(io::stdin());
    let mut stdout = io::BufWriter::new(io::stdout());

    let mut stream = LinesStream::new(stdin.lines())
        .filter_map(|l| l.ok())
        .filter_map(|line| {
            Some(Event {
                line: line.into_boxed_str(),
                spaces: None,
            })
        })
        .map(|mut event| {
            let spaces = total_spaces(&event.line);
            event.spaces = Some(spaces);
            event
        });

    while let Some(event) = stream.next().await {
        let spaces = total_spaces(&event.line);
        let header = get_header(spaces as usize);
        stdout.write_all(header).await?;
        stdout.write_all(event.line.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let runtime: Runtime = runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .expect("Unable to create async runtime");

    runtime.block_on(run())
}
