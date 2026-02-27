use chrono::{DateTime, Duration, Local};
use clap::Parser;

/// Generate a timestamp for "now + seconds".
#[derive(Parser, Debug)]
#[command(name = "timestamp-cli")]
#[command(about = "Print a timestamp for now + N seconds", long_about = None)]
struct Args {
    /// Seconds to add to the current time (default: 0)
    #[arg(value_name = "SECONDS", default_value_t = 0)]
    seconds: i64,

    /// Output format:
    /// - "rfc3339" (built-in)
    /// - or a chrono format string like "%Y-%m-%d %H:%M:%S"
    #[arg(long, value_name = "FORMAT")]
    format: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Current local time + offset
    let now: DateTime<Local> = Local::now();
    let ts = now + Duration::seconds(args.seconds);

    // Default format if none is provided
    let out = match args.format.as_deref() {
        None => ts.format("%Y-%m-%d %H:%M:%S").to_string(),

        Some("rfc3339") => ts.to_rfc3339(),

        Some(fmt) => {
            // chrono will accept many strftime-like directives.
            // If the user gives a weird string, it typically prints literals,
            // so we don't need to crash here.
            ts.format(fmt).to_string()
        }
    };

    println!("{out}");
}
