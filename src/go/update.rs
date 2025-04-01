use std::error::Error;
use std::fs;
use std::process::Command as ProcessCommand;
use reqwest;
use serde_json::Value;
use which::which;
use crate::utils::system::{get_os, OS};
use crate::utils::version::compare_versions;

pub async fn fetch_latest_go_version() -> Result<String, Box<dyn Error + Send + Sync>> {
    let response = reqwest::get("https://go.dev/dl/?mode=json").await?;
    let body = response.text().await?;

    let releases: Vec<Value> = serde_json::from_str(&body)?;
    if let Some(latest_version) = releases.iter().find(|release| {
        release["stable"].as_bool().unwrap_or(false)
    }) {
        let version = latest_version["version"]
            .as_str()
            .ok_or_else(|| -> Box<dyn Error + Send + Sync> { "Invalid version format".into() })?
            .to_string();
        Ok(version)
    } else {
        Err("Version not found".into())
    }
}

async fn get_installed_go_version() -> Result<String, Box<dyn Error + Send + Sync>> {
    match which("go") {
        Ok(_) => {
            let output = ProcessCommand::new("go").arg("version").output()?;
            let version_info = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let parts: Vec<&str> = version_info.split_whitespace().collect();
            if parts.len() >= 3 {
                Ok(parts[2].to_string())
            } else {
                Err("Could not determine installed Go version".into())
            }
        }
        Err(_) => Err("Go is not installed".into()),
    }
}

fn uninstall_go() -> Result<(), Box<dyn Error + Send + Sync>> {
    let os = get_os();

    let potential_paths = match os {
        OS::Windows => vec![
            "C:\\Program Files\\Go".to_string(),
            "C:\\Go".to_string(),
        ],
        OS::Linux => vec![
            "/usr/local/go".to_string(),
            "$HOME/go".to_string(),
        ],
        OS::MacOS => vec![
            "/usr/local/go".to_string(),
            "/opt/go".to_string(),
            "$HOME/go".to_string(),
        ],
        OS::Unknown => return Err("Unsupported OS for Go uninstallation".into()),
    };

    let mut go_path_found = false;

    for path in potential_paths {
        if fs::metadata(&path).is_ok() {
            println!("Go path found: {}", path);
            fs::remove_dir_all(&path)?;
            println!("Go has been removed from: {}", path);
            go_path_found = true;
            break;
        }
    }

    if !go_path_found {
        return Err("Go installation path not found in any of the checked locations.".into());
    }

    Ok(())
}

pub async fn install_go(version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let os = get_os();

    let download_url = match os {
        OS::Windows => format!("https://go.dev/dl/{}.windows-amd64.msi", version),
        OS::Linux => format!("https://go.dev/dl/{}.linux-amd64.tar.gz", version),
        OS::MacOS => format!("https://go.dev/dl/{}.darwin-amd64.tar.gz", version),
        OS::Unknown => return Err("Unsupported OS for Go installation".into()),
    };

    println!("Downloading Go from {}", download_url);

    let response = reqwest::get(&download_url).await?;
    let bytes = response.bytes().await?;

    let file_name = if os == OS::Windows {
        "go-installer.msi"
    } else {
        "go.tar.gz"
    };

    fs::write(file_name, &bytes)?;

    println!("Installing Go...");

    match os {
        OS::Windows => {
            ProcessCommand::new("msiexec")
                .args(["/i", file_name, "/quiet", "/norestart"])
                .status()?;
        }
        OS::Linux | OS::MacOS => {
            ProcessCommand::new("sudo")
                .args(["tar", "-C", "/usr/local", "-xzf", file_name])
                .status()?;
        }
        OS::Unknown => {
            return Err("Unsupported OS for Go installation".into());
        }
    }

    fs::remove_file(file_name)?;
    println!("Go {} installed successfully!", version);
    Ok(())
}

pub async fn update_go() -> Result<(), Box<dyn Error + Send + Sync>> {
    let installed_version = get_installed_go_version().await.unwrap_or_else(|_| "None".to_string());

    let latest_version = fetch_latest_go_version().await?;

    if installed_version == "None" {
        println!("Go is not installed. Installing the latest version...");
        install_go(&latest_version).await?;
    } else {
        match compare_versions(&installed_version, &latest_version) {
            std::cmp::Ordering::Equal => {
                println!("Go is already up to date ({}).", installed_version);
            }
            std::cmp::Ordering::Less => {
                println!(
                    "Updating Go from {} to {}...",
                    installed_version, latest_version
                );
                uninstall_go()?;
                install_go(&latest_version).await?;
            }
            std::cmp::Ordering::Greater => {
                println!(
                    "Your installed Go version ({}) is newer than the latest available ({}).",
                    installed_version, latest_version
                );
            }
        }
    }

    Ok(())
}
