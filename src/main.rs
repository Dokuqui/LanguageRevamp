use clap::{Arg, Command};
use go::check::check_go_version;
use go::update::update_go;
use crate::go::update::{fetch_latest_go_version, install_go};

mod go;
mod utils;

#[tokio::main]
async fn main() {
    let matches = Command::new("language-revamp")
        .version("0.1.0")
        .author("Ddokubi")
        .about("A CLI tool to update programming languages")
        .subcommand(
            Command::new("go")
                .about("Manage Go installation")
                .arg(
                    Arg::new("check")
                        .short('c')
                        .long("check")
                        .help("Check the current Go version")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("update")
                        .short('u')
                        .long("update")
                        .help("Update Go to the latest version")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("download")
                        .short('d')
                        .long("download")
                        .help("Download the latest version")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .get_matches();

    if let Some(go_matches) = matches.subcommand_matches("go") {
        let check = go_matches.get_one::<bool>("check").copied().unwrap_or(false);
        let update = go_matches.get_one::<bool>("update").copied().unwrap_or(false);
        let download = go_matches.get_one::<bool>("download").copied().unwrap_or(false);

        if check && update && download {
            eprintln!("Error: You can't use --check and --update together.");
            std::process::exit(1);
        } else if check {
            println!("Running check command...");
            if let Err(e) = check_go_version().await {
                eprintln!("Error checking Go version: {}", e);
            }
        } else if update {
            println!("Running update command...");
            if let Err(e) = update_go().await {
                eprintln!("Error updating Go: {}", e);
            }
        } else if download {
            let version = fetch_latest_go_version().await.unwrap();
            println!("Downloading and installing Go version: {}", version);
            if let Err(e) = install_go(&version).await {
                eprintln!("Error installing Go: {}", e);
            }
        } else {
            println!("Run 'language-revamp go --help' for usage instructions.");
        }
    } else {
        println!("Run 'language-revamp --help' for usage instructions.");
    }
}
