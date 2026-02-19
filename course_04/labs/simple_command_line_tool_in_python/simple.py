import subprocess
import json
import sys


def run_command(command: list[str]) -> str:
    """
    Runs a command without a shell and returns stdout as text.
    Raises CalledProcessError on non-zero exit.
    """
    return subprocess.check_output(command, text=True, stderr=subprocess.STDOUT)


def run_windows_disks() -> list[dict]:
    """
    Uses PowerShell to return disk inventory as JSON.
    Similar intent to `lsblk -J` on Linux.
    """
    ps_script = r"""
    Get-Disk |
      Select-Object Number, FriendlyName, SerialNumber, BusType, MediaType, Size, PartitionStyle, OperationalStatus, HealthStatus |
      ConvertTo-Json -Depth 3
    """.strip()

    output = run_command(["powershell", "-NoProfile", "-Command", ps_script])

    # ConvertTo-Json returns either an object or an array depending on count.
    data = json.loads(output) if output.strip() else []
    if isinstance(data, dict):
        return [data]
    return data


def find_windows_disk(disk_arg: str) -> dict | None:
    """
    disk_arg: disk number as a string (e.g., '0', '1').
    """
    try:
        disk_num = int(disk_arg)
    except ValueError:
        return None

    disks = run_windows_disks()
    for d in disks:
        if d.get("Number") == disk_num:
            return d
    return None


def print_help() -> None:
    print(
        "Usage: python simple.py <disk-number>\n"
        "Example: python simple.py 0\n\n"
        "This Windows-native version queries disks via PowerShell Get-Disk."
    )


def main(arg: str) -> None:
    disk = find_windows_disk(arg)
    if disk is None:
        print(f"Disk '{arg}' not found (or invalid). Use --help for usage.")
        raise SystemExit(2)

    print(json.dumps(disk, indent=2))


if __name__ == "__main__":
    if "--help" in sys.argv or "-h" in sys.argv:
        print_help()
        raise SystemExit(0)

    if len(sys.argv) < 2:
        print("No disk specified. Use --help for usage.")
        raise SystemExit(1)

    main(sys.argv[1])
