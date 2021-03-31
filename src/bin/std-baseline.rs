use pipeline::str::{get_header, total_spaces};
use std::io::{self, Write};

fn inner<R, W>(stdin: R, mut stdout: io::BufWriter<W>) -> Result<(), io::Error>
where
    R: io::BufRead,
    W: io::Write,
{
    let mut lines = stdin.lines();
    while let Some(Ok(line)) = lines.next() {
        let spaces = total_spaces(&line);
        let header = get_header(spaces as usize);
        stdout.write_all(header)?;
        stdout.write_all(&line.as_bytes())?;
        stdout.write_all(b"\n")?;
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    inner(stdin.lock(), io::BufWriter::new(stdout.lock()))
}
