use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Args {
    /// Optional disk number
    #[arg(long)]
    disk: Option<u32>,

    /// Enable debug prints (can also be set via DISKRS_DEBUG=true)
    #[arg(long, env = "DISKRS_DEBUG", default_value_t = false)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let ps_command = match args.disk {
        Some(n) => format!("Get-Disk -Number {n} | ConvertTo-Json"),
        None => "Get-Disk | ConvertTo-Json".to_string(),
    };

    if args.debug {
        eprintln!("DEBUG: disk argument = {:?}", args.disk);
        eprintln!("DEBUG: PowerShell command = {}", ps_command);
    }

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_command])
        .output()
        .expect("Failed to execute PowerShell");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}
