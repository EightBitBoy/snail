use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Lines per second
    #[arg(short, long, default_value_t = 1)]
    rate: u32,

    /// File to tail (optional)
    #[arg()]
    file: Option<String>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let delay = Duration::from_secs_f64(1.0 / cli.rate as f64);

    // Determine input source: stdin or file
    let reader: Box<dyn BufRead> = match &cli.file {
        Some(path) => {
            let file = File::open(path)?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(io::stdin().lock()),
    };

    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        match line {
            Ok(l) => {
                println!("{}", l);
                sleep(delay).await;
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(())
}
