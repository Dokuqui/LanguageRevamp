use std::process::Command;
use std::error::Error;
use crate::utils::system::{get_os, OS};

pub async fn check_python_version() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut found_any = false;
    let os = get_os();

    fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
        Command::new(cmd)
            .args(args)
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    None
                }
            })
    }

    if let Some(version) = run_command("python", &["--version"]) {
        found_any = true;
        println!("🐍 System Python: {}", version);
        if os == OS::Windows {
            if let Some(path) = run_command("where", &["python"]) {
                println!("   📍 Path: {}", path);
            }
        } else {
            if let Some(path) = run_command("which", &["python"]) {
                println!("   📍 Path: {}", path);
            }
        }
        if let Some(pip) = run_command("python", &["-m", "pip", "--version"]) {
            println!("   📦 Pip installed: {}", pip);
        }
    }

    if let Some(version) = run_command("python3", &["--version"]) {
        found_any = true;
        println!("🐍 Python3 (Alternative): {}", version);
        if os == OS::Windows {
            if let Some(path) = run_command("where", &["python3"]) {
                println!("   📍 Path: {}", path);
            }
        } else {
            if let Some(path) = run_command("which", &["python3"]) {
                println!("   📍 Path: {}", path);
            }
        }
        if let Some(pip) = run_command("python3", &["-m", "pip", "--version"]) {
            println!("   📦 Pip installed: {}", pip);
        }
    }

    if let Some(conda_version) = run_command("conda", &["--version"]) {
        found_any = true;
        println!("📦 Anaconda detected: {}", conda_version);
        if os == OS::Windows {
            if let Some(path) = run_command("where", &["conda"]) {
                println!("   📍 Path: {}", path);
            }
        } else {
            if let Some(path) = run_command("which", &["conda"]) {
                println!("   📍 Path: {}", path);
            }
        }
        if let Some(pip) = run_command("python", &["-m", "pip", "--version"]) {
            println!("   📦 Pip installed: {}", pip);
        }
    }

    if !found_any {
        println!("❌ No Python installation found.");
    }

    Ok(())
}
