use std::process::Command;
use which::which;

pub async fn check_node_version() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::task::spawn_blocking(|| {
        match which("node") {
            Ok(path) => {
                println!("Node found at: {}", path.display());
                match Command::new("node").arg("-v").output() {
                    Ok(output) => {
                        let version_info = String::from_utf8_lossy(&output.stdout);
                        println!("Installed Node version: {}", version_info.trim());
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("Error getting Node version: {}", e);
                        Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                    }
                }
            }
            Err(e) => {
                eprintln!("Node is not installed or not found in PATH. {}", e);
                Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }).await?
}