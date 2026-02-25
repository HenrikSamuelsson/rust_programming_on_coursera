use serde_json::Value;
use std::process::Command;

pub fn run_get_disk(disk: &str) -> Value {
    let ps_command = format!(
        "Get-Disk -Number {disk} | Select-Object Number,FriendlyName,SerialNumber,BusType,Size | ConvertTo-Json"
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_command])
        .output()
        .expect("Failed to execute PowerShell");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    serde_json::from_str(&stdout).expect("PowerShell did not return valid JSON")
}
