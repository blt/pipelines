use futures::pin_mut;
use futures_util::StreamExt;
use pipeline::core::Event;
use pipeline::str::{get_header, total_spaces};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio_stream::wrappers::LinesStream;

async fn run() -> Result<(), std::io::Error> {
    let stdin = io::BufReader::new(io::stdin());
    let mut stdout = io::BufWriter::new(io::stdout());

    let stream = LinesStream::new(stdin.lines())
        .filter_map(|l| async {
            match l {
                Ok(line) => Some(Event {
                    line: line.into_boxed_str(),
                    spaces: None,
                }),
                Err(_) => None,
            }
        })
        .map(|mut event| async {
            let spaces = total_spaces(&event.line);
            event.spaces = Some(spaces);
            event
        })
        .buffer_unordered(128);
    pin_mut!(stream);

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
