use std::process::Command;
use std::error::Error;
use crate::utils::system::{get_os, OS};
pub async fn update_python() -> Result<(), Box<dyn Error + Send + Sync>> {
    let os = get_os();

    match os {
        OS::Windows => {
            if let Ok(status) = Command::new("conda")
                .arg("update")
                .arg("python")
                .arg("-y")
                .status()
            {
                if status.success() {
                    println!("Python successfully updated via Anaconda.");
                    return Ok(());
                }
            }

            let status = Command::new("winget")
                .arg("install")
                .arg("--id")
                .arg("Python.Python.3")
                .arg("--source")
                .arg("winget")
                .status()?;

            if status.success() {
                println!("Python updated via winget.");
                return Ok(());
            }
        }
        OS::Linux => {
            let status = Command::new("sudo")
                .arg("apt")
                .arg("update")
                .arg("python3")
                .status()?;

            if status.success() {
                println!("Python updated on Linux.");
                return Ok(());
            }
        }
        OS::MacOS => {
            let status = Command::new("brew")
                .arg("upgrade")
                .arg("python")
                .status()?;

            if status.success() {
                println!("Python updated on macOS.");
                return Ok(());
            }
        }
        OS::Unknown => {
            println!("Unknown OS. Python update is not supported.");
        }
    }

    Err("Python update failed.".into())
}

pub async fn install_python() -> Result<(), Box<dyn Error + Send + Sync>> {
    let os = get_os();

    match os {
        OS::Windows => {
            if let Ok(status) = Command::new("conda")
                .arg("install")
                .arg("python")
                .arg("-y")
                .status()
            {
                if status.success() {
                    println!("Python successfully installed via Anaconda.");
                    return Ok(());
                }
            }

            let status = Command::new("winget")
                .arg("install")
                .arg("--id")
                .arg("Python.Python.3")
                .arg("--source")
                .arg("winget")
                .status()?;

            if status.success() {
                println!("Python installed via winget.");
                return Ok(());
            }
        }
        OS::Linux => {
            let status = Command::new("sudo")
                .arg("apt")
                .arg("install")
                .arg("python3")
                .arg("-y")
                .status()?;

            if status.success() {
                println!("Python installed on Linux.");
                return Ok(());
            }
        }
        OS::MacOS => {
            let status = Command::new("brew")
                .arg("install")
                .arg("python")
                .status()?;

            if status.success() {
                println!("Python installed on macOS.");
                return Ok(());
            }
        }
        OS::Unknown => {
            println!("Unknown OS. Python installation is not supported.");
        }
    }

    Err("Python installation failed.".into())
}

pub async fn update_pip() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut pip_update_cmd: Command = if is_conda_available() {
        let mut cmd = Command::new("conda");
        cmd.arg("run")
            .arg("-n")
            .arg("base")
            .arg("python")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("--upgrade")
            .arg("pip");
        cmd
    } else {
        let mut cmd = Command::new("python");
        cmd.arg("-m")
            .arg("pip")
            .arg("install")
            .arg("--upgrade")
            .arg("pip");
        cmd
    };

    let status = pip_update_cmd.status()?;

    if status.success() {
        println!("pip successfully updated.");
        return Ok(());
    }

    Err("pip update failed.".into())
}

pub async fn install_pip() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut pip_install_cmd: Command = if is_conda_available() {
        let mut cmd = Command::new("conda");
        cmd.arg("run")
            .arg("-n")
            .arg("base")
            .arg("python")
            .arg("-m")
            .arg("ensurepip")
            .arg("--upgrade");
        cmd
    } else {
        let mut cmd = Command::new("python");
        cmd.arg("-m")
            .arg("ensurepip")
            .arg("--upgrade");
        cmd
    };

    let status = pip_install_cmd.status()?;

    if status.success() {
        println!("pip successfully installed.");
        return Ok(());
    }

    Err("pip installation failed.".into())
}

fn is_conda_available() -> bool {
    Command::new("conda")
        .arg("--version")
        .output()
        .is_ok()
}
