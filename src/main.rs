mod args;
mod calculations;
mod server;

use calculations::{plot_summary, Investment};
use serde_json::to_string_pretty;
use std::env;

async fn run() -> std::io::Result<()> {
    let matches = args::build_cli().get_matches();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("`cls --help` for usage");
        return Ok(());
    }

    if let Some(matches) = matches.subcommand_matches("server") {
        let port = args::get_port(&matches);
        if let Err(e) = server::start_server(port).await {
            eprintln!("Failed to start server: {}", e);
        }
        return Ok(());
    }

    let investment = Investment::from_matches(&matches);
    let summary = investment.yearly_summary();
    if matches.get_flag("json") {
        match to_string_pretty(&summary) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Failed to serialize to JSON: {}", e),
        }
        return Ok(());
    }
    match plot_summary(&summary) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to plot summary: {}", e),
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
