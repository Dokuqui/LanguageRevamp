use std::{fs, process::Command, env};
use std::path::Path;
use reqwest;
use serde_json::{Value, from_str};
use which::which;
use crate::utils::system::{get_os, OS};
use crate::utils::version::compare_versions;

pub async fn fetch_latest_node_version() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get("https://nodejs.org/dist/index.json").await?;
    let body = response.text().await?;

    let releases: Vec<Value> = from_str(&body)?;
    if let Some(lts_release) = releases.iter().find(|release| {
        release["lts"].as_bool().unwrap_or(false) ||
            release["lts"].as_str().map(|s| !s.is_empty()).unwrap_or(false)
    }) {
        let version = lts_release["version"]
            .as_str()
            .ok_or("Invalid version format")?
            .to_string();
        Ok(version[1..].to_string())
    } else {
        Err("No LTS version found".into())
    }
}

async fn get_installed_node_version() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    match which("node") {
        Ok(_) => {
            let output = Command::new("node").arg("-v").output()?;
            let version_info = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(version_info[1..].to_string())
        }
        Err(_) => Err("Node is not installed".into()),
    }
}

fn uninstall_node() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let os = get_os();

    let potential_paths = match os {
        OS::Windows => vec![
            "C:\\Program Files\\nodejs".to_string(),
            "C:\\nodejs".to_string(),
            format!("{}\\nodejs", env::var("ProgramFiles").unwrap_or_default()),
            format!("{}\\Node", env::var("ProgramFiles").unwrap_or_default()),
        ],
        OS::Linux => vec![
            "/usr/local/node".to_string(),
            "/usr/local/bin/node".to_string(),
            "/usr/bin/node".to_string(),
            "/opt/node".to_string(),
        ],
        OS::MacOS => vec![
            "/usr/local/node".to_string(),
            "/usr/local/bin/node".to_string(),
            "/opt/node".to_string(),
            "/opt/local/bin/node".to_string(),
        ],
        OS::Unknown => return Err("Unsupported OS for Node uninstallation".into()),
    };

    let mut node_path_found = false;

    match os {
        OS::Windows => {
            let uninstall_result = Command::new("wmic")
                .args(&["product", "where", "name='Node.js'", "call", "uninstall"])
                .status();

            if let Ok(status) = uninstall_result {
                if status.success() {
                    println!("Node.js uninstalled via MSI installer");
                    node_path_found = true;
                }
            }

            if !node_path_found {
                for path in &potential_paths {
                    if fs::metadata(path).is_ok() {
                        println!("Node.js path found: {}", path);
                        match Command::new("cmd")
                            .args(&["/C", "rmdir", "/S", "/Q", path])
                            .status()
                        {
                            Ok(status) if status.success() => {
                                println!("Node.js has been removed from: {}", path);
                                node_path_found = true;
                                break;
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }
        OS::Linux | OS::MacOS => {
            for path in &potential_paths {
                if fs::metadata(path).is_ok() {
                    println!("Node.js path found: {}", path);
                    Command::new("sudo")
                        .args(["rm", "-rf", path])
                        .status()?;
                    println!("Node.js has been removed from: {}", path);
                    node_path_found = true;
                    break;
                }
            }
        }
        OS::Unknown => unreachable!(),
    }

    if let Ok(home) = env::var("HOME").or(env::var("USERPROFILE")) {
        let npm_path = format!("{}\\AppData\\Roaming\\npm", home);
        let npm_cache = format!("{}\\AppData\\Roaming\\npm-cache", home);

        if fs::metadata(&npm_path).is_ok() {
            fs::remove_dir_all(&npm_path)?;
            println!("Cleaned up global npm modules from: {}", npm_path);
        }
        if fs::metadata(&npm_cache).is_ok() {
            fs::remove_dir_all(&npm_cache)?;
            println!("Cleaned up npm cache from: {}", npm_cache);
        }
    }

    if !node_path_found {
        return Err("Node.js installation path not found in any of the checked locations.".into());
    }

    Ok(())
}

pub async fn install_node(version: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let os = get_os();

    let download_url = match os {
        OS::Windows => format!("https://nodejs.org/dist/v{}/node-v{}-x64.msi", version, version),
        OS::Linux => format!("https://nodejs.org/dist/v{}/node-v{}-linux-x64.tar.gz", version, version),
        OS::MacOS => format!("https://nodejs.org/dist/v{}/node-v{}-darwin-x64.tar.gz", version, version),
        OS::Unknown => return Err("Unsupported OS for Node.js installation".into()),
    };

    println!("Downloading Node.js LTS v{} from {}", version, download_url);

    let response = reqwest::get(&download_url).await?;
    let bytes = response.bytes().await?;

    let file_name = if os == OS::Windows {
        "node-installer.msi"
    } else {
        "node.tar.gz"
    };

    fs::write(file_name, &bytes)?;

    println!("Installing Node.js...");

    match os {
        OS::Windows => {
            let status = Command::new("msiexec")
                .args(["/i", file_name, "/quiet", "/norestart"])
                .status()?;
            if !status.success() {
                return Err("Failed to install Node.js MSI".into());
            }
        }
        OS::Linux | OS::MacOS => {
            let status = Command::new("sudo")
                .args(["tar", "-C", "/usr/local", "-xzf", file_name])
                .status()?;
            if !status.success() {
                return Err("Failed to extract Node.js archive".into());
            }

            let extracted_dir = format!(
                "/usr/local/node-v{}-{}",
                version,
                if os == OS::Linux { "linux-x64" } else { "darwin-x64" }
            );
            Command::new("sudo")
                .args(["ln", "-sf", &format!("{}/bin/node", extracted_dir), "/usr/local/bin/node"])
                .status()?;
            Command::new("sudo")
                .args(["ln", "-sf", &format!("{}/bin/npm", extracted_dir), "/usr/local/bin/npm"])
                .status()?;
        }
        OS::Unknown => {
            return Err("Unsupported OS for Node.js installation".into());
        }
    }

    fs::remove_file(file_name)?;

    println!("Node.js v{} installed successfully!", version);
    Ok(())
}

pub async fn update_node() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let installed_version = get_installed_node_version().await.unwrap_or("None".to_string());
    let latest_version = fetch_latest_node_version().await?;

    if installed_version == "None" {
        println!("Node.js is not installed. Installing latest LTS version...");
        install_node(&latest_version).await?;
    } else {
        match compare_versions(&installed_version, &latest_version) {
            std::cmp::Ordering::Equal => {
                println!("Node.js is already up to date (v{})", installed_version);
            }
            std::cmp::Ordering::Less => {
                println!("Updating Node.js from v{} to v{}", installed_version, latest_version);
                uninstall_node()?;
                install_node(&latest_version).await?;
            }
            std::cmp::Ordering::Greater => {
                println!("Installed Node.js (v{}) is newer than latest LTS (v{})",
                         installed_version, latest_version);
            }
        }
    }

    Ok(())
}

pub fn is_nvm_installed() -> bool {
    let home = env::var("HOME").or(env::var("USERPROFILE")).unwrap_or_default();
    let nvm_paths = vec![
        format!("{}/.nvm/nvm.sh", home),
        format!("{}\\AppData\\Roaming\\nvm", home),
    ];

    nvm_paths.iter().any(|path| Path::new(path).exists()) ||
        Command::new("nvm").arg("version").output().is_ok()
}

pub async fn update_with_nvm() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Updating Node.js using NVM...");

    let latest_version = fetch_latest_node_version().await?;

    let status = Command::new("nvm")
        .args(&["install", &latest_version])
        .status()?;

    if !status.success() {
        return Err("Failed to install Node.js with NVM".into());
    }

    let status = Command::new("nvm")
        .args(&["use", &latest_version]).status()?;

    if !status.success() {
        return Err("Failed to switch to new Node.js version with NVM".into());
    }

    println!("Node.js updated to v{} using NVM", latest_version);
    Ok(())
}

pub async fn install_with_nvm() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Installing Node.js using NVM...");

    let latest_version = fetch_latest_node_version().await?;

    let status = Command::new("nvm")
        .args(&["install", &latest_version]).status()?;

    if !status.success() {
        return Err("Failed to install Node.js with NVM".into());
    }

    println!("Node.js v{} installed using NVM", latest_version);
    Ok(())
}