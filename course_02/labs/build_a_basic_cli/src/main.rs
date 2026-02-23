use build_a_basic_cli::run_get_disk;
use clap::{Arg, ColorChoice, Command};

fn main() {
    let matches = Command::new("basic-rust-cli")
        .bin_name("basic-rust-cli")
        .version("0.0.1")
        .author("Henrik")
        .about("Get-Disk in Rust (Windows PowerShell JSON output)")
        .color(ColorChoice::Always)
        .arg(
            Arg::new("disk")
                .help("Disk number to query (e.g., 0)")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(disk) = matches.get_one::<String>("disk") {
        let output = serde_json::to_string(&run_get_disk(disk)).unwrap();
        println!("{output}");
    } else {
        println!("No disk provided");
    }
}
