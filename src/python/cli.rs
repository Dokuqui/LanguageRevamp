use clap::{Arg, ArgMatches, Command};
use crate::python::{check::check_python_version};
use crate::python::update::{install_pip, install_python, update_pip, update_python};

pub fn python_subcommand() -> Command {
    Command::new("python")
        .about("Manage Python installation")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .help("Check the installed Python version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .help("Update Python to the latest version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("install")
                .short('i')
                .long("install")
                .help("Install Python if not installed")
                .action(clap::ArgAction::SetTrue),
        )
}

pub async fn handle_python_commands(matches: &ArgMatches) {
    let check = matches.get_one::<bool>("check").copied().unwrap_or(false);
    let update = matches.get_one::<bool>("update").copied().unwrap_or(false);
    let install = matches.get_one::<bool>("install").copied().unwrap_or(false);

    if check {
        println!("Checking Python version...");
        if let Err(e) = check_python_version().await {
            eprintln!("Error checking Python version: {}", e);
        }
    }else if update {
        println!("Updating Python...");
        if let Err(e) = update_python().await {
            eprintln!("Error updating Python: {}", e);
        }

        println!("Updating pip...");
        if let Err(e) = update_pip().await {
            eprintln!("Error updating pip: {}", e);
        }
    }else if install {
        println!("Installing Python...");
        if let Err(e) = install_python().await {
            eprintln!("Error installing Python: {}", e);
        }

        println!("Installing pip...");
        if let Err(e) = install_pip().await {
            eprintln!("Error updating pip: {}", e);
        }
    } else {
        println!("Run 'language-revamp python --help' for usage instructions.");
    }
}
