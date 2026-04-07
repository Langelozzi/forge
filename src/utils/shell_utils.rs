use std::io::{self, Error, Read, Write};
use std::process::{Command, Stdio};

/// exec: Streams output to terminal AND captures it for Forge to use.
pub fn exec(command: &str) -> io::Result<String> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(Error::other("Empty command"));
    }

    let mut child = Command::new(parts[0])
        .args(&parts[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut captured = String::new();

    // We use a simple loop to read stdout while it's running
    if let Some(mut stdout) = child.stdout.take() {
        let mut buffer = [0; 1024];
        loop {
            let n = stdout.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            let chunk = &buffer[..n];

            // Stream to terminal
            io::stdout().write_all(chunk)?;
            io::stdout().flush()?;

            // Capture for return
            captured.push_str(&String::from_utf8_lossy(chunk));
        }
    }

    let status = child.wait()?;
    if status.success() {
        Ok(captured.trim().to_string())
    } else {
        Err(Error::other(format!("Command '{}' failed", parts[0])))
    }
}

/// exec_raw: Runs command purely in the terminal (no capture).
pub fn exec_raw(command: &str) -> io::Result<()> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(Error::other("Empty command"));
    }

    let status = Command::new(parts[0]).args(&parts[1..]).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::other(format!("Command '{}' failed", parts[0])))
    }
}
