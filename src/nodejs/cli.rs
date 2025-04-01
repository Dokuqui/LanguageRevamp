use clap::{Command, Arg, ArgMatches};
use crate::nodejs::check::check_node_version;
use crate::nodejs::update::{update_node, fetch_latest_node_version, install_node, is_nvm_installed, install_with_nvm, update_with_nvm};

pub fn node_subcommand() -> Command {
    Command::new("node")
        .about("Manage Node.js installation")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .help("Check the current Node.js version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("update-manual")
                .short('u')
                .long("update-manual")
                .help("Update Node.js to the latest version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("install")
                .short('i')
                .long("install")
                .help("Install the latest Node.js version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("nvm")
                .short('n')
                .long("nvm")
                .help("Use NVM (Node Version Manager) for updates/installation")
                .action(clap::ArgAction::SetTrue),
        )
}

pub async fn handle_node_commands(matches: &ArgMatches) {
    let check = matches.get_one::<bool>("check").copied().unwrap_or(false);
    let update = matches.get_one::<bool>("update-manual").copied().unwrap_or(false);
    let install = matches.get_one::<bool>("install").copied().unwrap_or(false);
    let use_nvm = matches.get_one::<bool>("nvm").copied().unwrap_or(false);

    let nvm_available = is_nvm_installed();

    if check {
        if let Err(e) = check_node_version().await {
            eprintln!("Error checking Node.js version: {}", e);
        }
    } else if update {
        if use_nvm || nvm_available {
            if !nvm_available {
                eprintln!("NVM is not installed. Please install NVM or remove --nvm flag.");
                return;
            }
            if let Err(e) = update_with_nvm().await {
                eprintln!("Error updating Node.js with NVM: {}", e);
            }
        } else {
            if let Err(e) = update_node().await {
                eprintln!("Error updating Node.js: {}", e);
            }
        }
    } else if install {
        if use_nvm || nvm_available {
            if !nvm_available {
                eprintln!("NVM is not installed. Please install NVM or remove --nvm flag.");
                return;
            }
            if let Err(e) = install_with_nvm().await {
                eprintln!("Error installing Node.js with NVM: {}", e);
            }
        } else {
            let version = match fetch_latest_node_version().await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error fetching latest version: {}", e);
                    return;
                }
            };
            if let Err(e) = install_node(&version).await {
                eprintln!("Error installing Node.js: {}", e);
            }
        }
    } else {
        println!("Run 'language-revamp node --help' for usage instructions.");
    }
}