use clap::{Arg, Command, ArgMatches};
use crate::rust::check::check_rust_version;
use crate::rust::update::{install_rust, update_rust};

pub fn rust_subcommand() -> Command {
    Command::new("rust")
        .about("Manage Rust installation")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .help("Check the installed Rust version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .help("Update Rust to the latest version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("install")
                .short('i')
                .long("install")
                .help("Install Rust if not installed")
                .action(clap::ArgAction::SetTrue),
        )
}

pub async fn handle_rust_commands(matches: &ArgMatches) {
    let check = matches.get_one::<bool>("check").copied().unwrap_or(false);
    let update = matches.get_one::<bool>("update").copied().unwrap_or(false);
    let install = matches.get_one::<bool>("install").copied().unwrap_or(false);


    if check {
        println!("Checking Rust version...");
        if let Err(e) = check_rust_version().await {
            eprintln!("Error checking Rust version: {}", e);
        }
    } else if update {
        println!("Updating Rust...");
        if let Err(e) = update_rust().await {
            eprintln!("Error updating Rust: {}", e);
        }
    } else if install {
        println!("Installing Rust...");
        if let Err(e) = install_rust().await {
            eprintln!("Error installing Rust: {}", e);
        }
    } else {
        println!("Run 'language-revamp rust --help' for usage instructions.");
    }
}
