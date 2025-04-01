use std::process::Command;
use which::which;
use crate::utils::system::{get_os, OS};

pub async fn check_java_version() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::task::spawn_blocking(|| {
        match which("java") {
            Ok(path) => {
                println!("Java found at: {}", path.display());
                let output = match get_os() {
                    OS::Windows => Command::new("cmd").args(&["/C", "java", "-version"]).output(),
                    OS::Linux | OS::MacOS => Command::new("java").arg("-version").output(),
                    OS::Unknown => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported OS")),
                }?;

                if output.status.success() {
                    let version_info = String::from_utf8_lossy(&output.stderr);
                    let version_line = version_info.lines().next().unwrap_or("Unknown version");
                    println!("Installed Java version: {}", version_line);
                    Ok(())
                } else {
                    let error_info = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Java execution failed: {}", error_info);
                    Err("Java execution failed".into())
                }
            }
            Err(e) => {
                let suggestion = match get_os() {
                    OS::Windows => "Ensure Java is installed and added to PATH (e.g., C:\\Program Files\\Java).",
                    OS::Linux => "Ensure Java is installed (e.g., via package manager or /usr/bin).",
                    OS::MacOS => "Ensure Java is installed (e.g., via Homebrew or /usr/bin).",
                    OS::Unknown => "Ensure Java is installed on your system.",
                };
                eprintln!("Java is not installed or not found in PATH: {}. {}", e, suggestion);
                Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }).await?
}