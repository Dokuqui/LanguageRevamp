use std::process::Command;
use std::error::Error;
use crate::rust::check::check_rust_version;

pub async fn fetch_latest_version_rust() -> Result<String, Box<dyn Error + Send + Sync>> {
    let output = Command::new("rustup")
        .arg("show")
        .arg("active-toolchain")
        .output()?;

    if output.status.success() {
        let version_info = String::from_utf8_lossy(&output.stdout).trim().to_string();

        let version_parts: Vec<&str> = version_info.split_whitespace().collect();
        if !version_parts.is_empty() {
            let version = version_parts[0].split('-').next().unwrap_or("None");
            Ok(version.to_string())
        } else {
            Err("Couldn't extract Rust version".into())
        }
    } else {
        Err("Rustup is not installed or failed to execute".into())
    }
}

pub async fn update_rust() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Checking installed Rust version...");

    let installed_version = check_rust_version().await.unwrap_or_else(|_| "None".to_string());

    let installed_version_cleaned = installed_version
        .split_whitespace()
        .nth(1)
        .unwrap_or("None")
        .to_string();

    println!("Fetching latest Rust version...");
    let latest_version = fetch_latest_version_rust().await?;

    if installed_version_cleaned == latest_version {
        println!("✅ Rust is already up to date ({}).", installed_version_cleaned);
    } else {
        println!("🔄 Updating Rust from {} to {}...", installed_version_cleaned, latest_version);
        let status = Command::new("rustup")
            .args(["update", "stable"])
            .status()?;

        if status.success() {
            println!("🎉 Rust successfully updated to {}!", latest_version);
        } else {
            eprintln!("❌ Error: Rust update failed.");
        }
    }
    Ok(())
}

pub async fn install_rust() -> Result<(), Box<dyn Error + Send + Sync>> {
    let status = if cfg!(target_os = "windows") {
        Command::new("winget")
            .args(["install", "--id", "Rustlang.Rustup"])
            .status()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
            .status()
    }?;

    if status.success() {
        println!("Rust successfully installed!");
        Ok(())
    } else {
        Err("Failed to install Rust".into())
    }
}
