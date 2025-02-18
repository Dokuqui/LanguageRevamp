use clap::{Arg, Command};
use std::process::Command as ProcessCommand;
use which::which;

fn main() {
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
                        .num_args(0),
                )
                .arg(
                    Arg::new("update")
                        .short('u')
                        .long("update")
                        .help("Update Go to the latest version")
                        .num_args(0),
                ),
        )
        .get_matches();

    if let Some(go_matches) = matches.subcommand_matches("go") {
        if go_matches.contains_id("check") {
            check_go_version();
        } else if go_matches.contains_id("update") {
            println!("Updating Go to the latest version...");
        } else {
            println!("Run 'language-revamp go --help' for usage instructions.");
        }
    } else {
        println!("Run 'language-revamp --help' for usage instructions.")
    }
}

fn check_go_version() {
    match which("go") {
        Ok(path) => {
            println!("Go found at: {}", path.display());
            match ProcessCommand::new("go").arg("version").output() {
                Ok(output) => {
                    let version_info = String::from_utf8_lossy(&output.stdout);
                    println!("Installed Go version: {}", version_info.trim());
                }
                Err(_) => println!("Error: Unable to retrieve Go version."),
            }
        }
        Err(_) => println!("Go is not installed or not found in PATH."),
    }
}
