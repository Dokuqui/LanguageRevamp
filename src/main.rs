mod go;
mod rust;
mod cli;
mod utils;
mod python;

use cli::{build_cli, handle_cli};

#[tokio::main]
async fn main() {
    let matches = build_cli().get_matches();
    handle_cli(matches).await;
}
