use clap::Parser;
use std::io::{self, BufRead};
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Lines per second
    #[arg(short, long, default_value_t = 1)]
    rate: u32,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let delay = Duration::from_secs_f64(1.0 / cli.rate as f64);

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
