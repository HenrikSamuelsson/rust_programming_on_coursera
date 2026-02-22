use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Args {
    /// Optional disk number
    #[arg(long)]
    disk: Option<u32>,
}

fn main() {
    let args = Args::parse();

    let ps_command = match args.disk {
        Some(n) => format!("Get-Disk -Number {n} | ConvertTo-Json"),
        None => "Get-Disk | ConvertTo-Json".to_string(),
    };

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_command])
        .output()
        .expect("Failed to execute PowerShell");

    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("{}", stdout);
}
