use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    after_help =
"EXAMPLES:

snl /var/log/application.log

snl debug.log --rate 5

tail -f /var/log/nginx/error.log | grep 'crit' | snail -r 1"
)]
struct Cli {
    /// File to output with a limited rate (optional, reads from stdin if not provided)
    #[arg()]
    file: Option<String>,

    /// Rate of the output in lines per second
    #[arg(short, long, default_value_t = 2)]
    rate: u32,
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

    let mut reader = reader; // make mutable for read_line
    let mut buf = String::new();

    loop {
        buf.clear();
        let bytes_read = reader.read_line(&mut buf)?;

        if bytes_read > 0 {
            print!("{}", buf); // buf already contains the newline
            sleep(delay).await;
        } else {
            // No new data, wait a bit before retrying
            sleep(Duration::from_millis(200)).await;
        }
    }
}
