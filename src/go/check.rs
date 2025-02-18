use std::error::Error;
use std::process::Command as ProcessCommand;
use tokio::task;
use which::which;

pub async fn check_go_version() -> Result<(), Box<dyn Error + Send + Sync>> {
    task::spawn_blocking(|| {
        match which("go") {
            Ok(path) => {
                println!("Go found at: {}", path.display());
                match ProcessCommand::new("go").arg("version").output() {
                    Ok(output) => {
                        let version_info = String::from_utf8_lossy(&output.stdout);
                        println!("Installed Go version: {}", version_info.trim());
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("Error: Unable to retrieve Go version. {}", e);
                        Err(Box::new(e) as Box<dyn Error + Send + Sync>)
                    }
                }
            }
            Err(e) => {
                eprintln!("Go is not installed or not found in PATH. {}", e);
                Err(Box::new(e) as Box<dyn Error + Send + Sync>)
            }
        }
    })
        .await?
}
