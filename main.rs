use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use tokio::time::{sleep, Duration};

const WAIT_FOR_DATA_MILLIS: u64 = 200;

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
    #[arg(short, long, default_value_t = 2, value_parser = clap::value_parser!(u32).range(1..))]
    rate: u32,

    /// Exit if the input stops (optional, continues to wait for data if not provided)
    #[arg(short, long, default_value_t = false)]
    exit: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let delay = Duration::from_secs_f64(1.0 / cli.rate as f64);

    let mut reader = create_reader(&cli.file)?;
    let mut buf = String::new();

    loop {
        buf.clear();
        let bytes_read = reader.read_line(&mut buf)?;

        if bytes_read > 0 {
            print!("{}", buf);
            io::stdout().flush()?;

            sleep_between_lines(delay).await;
        } else if cli.exit {
            break;
        } else {
            wait_for_data().await;
        }
    }

    Ok(())
}

/// Return a buffered reader for the specified file or stdin if no file is provided.
fn create_reader(file: &Option<String>) -> io::Result<Box<dyn BufRead>> {
    match file {
        Some(path) => {
            let file = File::open(path)?;
            Ok(Box::new(BufReader::new(file)))
        }
        None => Ok(Box::new(io::stdin().lock())),
    }
}

async fn sleep_between_lines(delay: Duration) {
    sleep(delay).await;
}

async fn wait_for_data() {
    sleep(Duration::from_millis(WAIT_FOR_DATA_MILLIS)).await;
}
