use clap::{Command, Arg, ArgMatches};
use crate::java::check::check_java_version;
use crate::java::update::{install_java, update_java, fetch_latest_java_version};

pub fn java_subcommand() -> Command {
    Command::new("java")
        .about("Manage Java installation")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .help("Check the current Java version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("update-manual")
                .short('u')
                .long("update-manual")
                .help("Update Java to the latest LTS version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("install")
                .short('i')
                .long("install")
                .help("Install the latest Java LTS version")
                .action(clap::ArgAction::SetTrue),
        )
}

pub async fn handle_java_commands(matches: &ArgMatches) {
    let check = matches.get_one::<bool>("check").copied().unwrap_or(false);
    let update = matches.get_one::<bool>("update-manual").copied().unwrap_or(false);
    let install = matches.get_one::<bool>("install").copied().unwrap_or(false);

    if check {
        if let Err(e) = check_java_version().await {
            eprintln!("Error checking Java version: {}", e);
        }
    } else if update {
        if let Err(e) = update_java().await {
            eprintln!("Error updating Java: {}", e);
        }
    } else if install {
        let version = match fetch_latest_java_version().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error fetching latest Java version: {}", e);
                return;
            }
        };
        if let Err(e) = install_java(&version).await {
            eprintln!("Error installing Java: {}", e);
        }
    } else {
        println!("Run 'language-revamp java --help' for usage instructions.");
    }
}