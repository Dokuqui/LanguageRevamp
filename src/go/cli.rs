use clap::{Arg, Command, ArgMatches};
use crate::go::check::check_go_version;
use crate::go::update::{update_go, fetch_latest_go_version, install_go};

pub fn go_subcommand() -> Command {
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
                .help("Download and install the latest Go version")
                .action(clap::ArgAction::SetTrue),
        )
}

pub async fn handle_go_commands(matches: &ArgMatches) {
    let check = matches.get_one::<bool>("check").copied().unwrap_or(false);
    let update = matches.get_one::<bool>("update").copied().unwrap_or(false);
    let download = matches.get_one::<bool>("download").copied().unwrap_or(false);

    if check {
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
}
