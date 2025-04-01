use clap::{Command, ArgMatches};
use crate::go::cli::{go_subcommand, handle_go_commands};
use crate::python::cli::{handle_python_commands, python_subcommand};
use crate::rust::cli::{rust_subcommand, handle_rust_commands};

pub fn build_cli() -> Command {
    Command::new("language-revamp")
        .version("1.0.0")
        .author("Ddokubi")
        .about("A CLI tool to manage programming languages")
        .subcommand(go_subcommand())
        .subcommand(rust_subcommand())
        .subcommand(python_subcommand())
}

pub async fn handle_cli(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("go", sub_matches)) => handle_go_commands(sub_matches).await,
        Some(("rust", sub_matches)) => handle_rust_commands(sub_matches).await,
        Some(("python", sub_matches)) => handle_python_commands(sub_matches).await,
        _ => println!("Run 'language-revamp --help' for usage instructions."),
    }
}
