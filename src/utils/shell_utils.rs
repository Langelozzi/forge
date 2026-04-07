use std::{io::Error, process::Command};

pub fn exec(command: &str) -> std::io::Result<String> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let cmd = parts[0];
    let args = &parts[1..];

    let output = Command::new(cmd).args(args).output()?; // This returns an io::Error if the command fails to start

    if output.status.success() {
        // Convert bytes to String, trim whitespace/newlines
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        // Capture stderr to see WHY it failed
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(Error::other::<_>(format!(
            "Command '{}' failed: {}",
            cmd, err_msg
        )))
    }
}
