use std::process::Command;
use std::error::Error;

pub async fn check_rust_version() -> Result<String, Box<dyn Error + Send + Sync>> {
   let output = Command::new("rustc")
        .arg("--version")
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let version_info = String::from_utf8_lossy(&out.stdout).trim().to_string();
                println!("Installed Rust version: {}", version_info);
                Ok(version_info)
            } else {
                let error_info = String::from_utf8_lossy(&out.stderr);
                eprintln!("Error executing rustc: {}", error_info);
                Err("Failed to get Rust version".into())
            }
        }
        Err(e) => {
            eprintln!("Failed to execute rustc: {}", e);
            Err(Box::new(e))
        }
    }
}
