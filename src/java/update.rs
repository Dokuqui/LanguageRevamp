use std::{fs, process::Command, env};
use reqwest;
use serde_json::{Value, from_str};
use which::which;
use crate::utils::system::{get_os, OS};
use crate::utils::version::compare_versions;

pub async fn fetch_latest_java_version() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get("https://api.adoptium.net/v3/info/available_releases").await?;
    let body = response.text().await?;

    let release_info: Value = from_str(&body)?;
    let available_lts = release_info["available_lts_releases"]
        .as_array()
        .ok_or("No LTS releases found")?;

    if let Some(latest_lts) = available_lts.iter().max_by_key(|v| v.as_i64().unwrap_or(0)) {
        let version = latest_lts.as_i64().unwrap().to_string();
        Ok(version)
    } else {
        Err("No LTS version found".into())
    }
}

pub async fn install_java(version: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let os = get_os();

    let download_url = match os {
        OS::Windows => format!(
            "https://github.com/adoptium/temurin{}-binaries/releases/download/jdk-{}%2B37/OpenJDK{}U-jdk_x64_windows_hotspot_{}u37.msi",
            version, version, version, version
        ),
        OS::Linux => format!(
            "https://github.com/adoptium/temurin{}-binaries/releases/download/jdk-{}%2B37/OpenJDK{}U-jdk_x64_linux_hotspot_{}u37.tar.gz",
            version, version, version, version
        ),
        OS::MacOS => format!(
            "https://github.com/adoptium/temurin{}-binaries/releases/download/jdk-{}%2B37/OpenJDK{}U-jdk_x64_mac_hotspot_{}u37.tar.gz",
            version, version, version, version
        ),
        OS::Unknown => return Err("Unsupported OS for Java installation".into()),
    };

    println!("Downloading Java LTS v{} from {}", version, download_url);

    let response = reqwest::get(&download_url).await?;
    let bytes = response.bytes().await?;

    let file_name = if os == OS::Windows {
        "java-installer.msi"
    } else {
        "java.tar.gz"
    };

    fs::write(file_name, &bytes)?;

    println!("Installing Java...");

    match os {
        OS::Windows => {
            let status = Command::new("msiexec")
                .args(["/i", file_name, "/quiet", "/norestart"])
                .status()?;
            if !status.success() {
                return Err("Failed to install Java MSI".into());
            }
        }
        OS::Linux | OS::MacOS => {
            let status = Command::new("sudo")
                .args(["tar", "-C", "/usr/local", "-xzf", file_name])
                .status()?;
            if !status.success() {
                return Err("Failed to extract Java archive".into());
            }

            let extracted_dir = format!(
                "/usr/local/jdk-{}+37",
                version
            );
            Command::new("sudo")
                .args(["ln", "-sf", &format!("{}/bin/java", extracted_dir), "/usr/local/bin/java"])
                .status()?;
        }
        OS::Unknown => {
            return Err("Unsupported OS for Java installation".into());
        }
    }

    fs::remove_file(file_name)?;

    println!("Java v{} installed successfully!", version);
    Ok(())
}

pub async fn get_installed_java_version() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    tokio::task::spawn_blocking(|| {
        match which("java") {
            Ok(_) => {
                let output = match get_os() {
                    OS::Windows => Command::new("cmd").args(&["/C", "java", "-version"]).output(),
                    OS::Linux | OS::MacOS => Command::new("java").arg("-version").output(),
                    OS::Unknown => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported OS")),
                }?;

                if output.status.success() {
                    let version_info = String::from_utf8_lossy(&output.stderr);
                    let version_line = version_info.lines().next().unwrap_or("Unknown version");
                    let version = version_line.split('"').nth(1).unwrap_or("0").split('.').next().unwrap_or("0");
                    Ok(version.to_string())
                } else {
                    Err("Failed to get Java version".into())
                }
            }
            Err(_) => Err("Java is not installed".into()),
        }
    }).await?
}

pub async fn update_java() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let installed_version = get_installed_java_version().await.unwrap_or("0".to_string());
    let latest_version = fetch_latest_java_version().await?;

    if installed_version == "0" {
        println!("Java is not installed. Installing the latest LTS version...");
        install_java(&latest_version).await?;
    } else {
        match compare_versions(&installed_version, &latest_version) {
            std::cmp::Ordering::Equal => {
                println!("Java is already up to date (v{})", installed_version);
            }
            std::cmp::Ordering::Less => {
                println!("Updating Java from v{} to v{}", installed_version, latest_version);
                uninstall_java()?;
                install_java(&latest_version).await?;
            }
            std::cmp::Ordering::Greater => {
                println!(
                    "Installed Java version (v{}) is newer than the latest available (v{})",
                    installed_version, latest_version
                );
            }
        }
    }

    Ok(())
}

fn uninstall_java() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let os = get_os();

    match os {
        OS::Windows => {
            let status = Command::new("wmic")
                .args(&["product", "where", "name like 'Java%'", "call", "uninstall"])
                .status()?;
            if !status.success() {
                println!("Could not find Java installation to uninstall via MSI. You may need to uninstall manually.");
            } else {
                println!("Java uninstalled successfully via MSI.");
            }
            let java_paths = vec![
                "C:\\Program Files\\Java".to_string(),
                format!("{}\\Java", env::var("ProgramFiles").unwrap_or_default()),
            ];
            for path in java_paths {
                if fs::metadata(&path).is_ok() {
                    fs::remove_dir_all(&path)?;
                    println!("Cleaned up Java directory: {}", path);
                }
            }
            Ok(())
        }
        OS::Linux | OS::MacOS => {
            let java_paths = vec![
                "/usr/local/jdk-*".to_string(),
                "/usr/local/bin/java".to_string(),
            ];
            for path in java_paths {
                if fs::metadata(&path).is_ok() {
                    Command::new("sudo")
                        .args(["rm", "-rf", &path])
                        .status()?;
                    println!("Removed Java from: {}", path);
                }
            }
            Ok(())
        }
        OS::Unknown => Err("Unsupported OS for Java uninstallation".into()),
    }
}