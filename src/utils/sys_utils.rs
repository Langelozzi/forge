use std::process::Command;

pub fn get_default_c_compiler() -> String {
    let candidates = ["gcc", "clang", "cc"];

    for compiler in candidates {
        // Run 'which <compiler>' to find the absolute path
        let output = Command::new("which").arg(compiler).output();

        if let Ok(out) = output {
            if out.status.success() {
                // Convert bytes to string and trim the newline character
                let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path.is_empty() {
                    return path;
                }
            }
        }
    }

    // Fallback: If 'which' fails, just return a default string
    "gcc".to_string()
}
