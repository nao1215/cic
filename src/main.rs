mod calculations;

use calculations::{plot_summary, Investment};
use clap::{Arg, Command};
use serde_json::to_string_pretty;
use std::env;

fn run() {
    let matches = Command::new("Compound Interest Calculator")
        .about("cis - Calculates Compound Interest.\nOutput the results of compound interest calculations as either a line graph image or JSON.")
        .arg(
            Arg::new("principal")
                .short('p')
                .long("principal")
                .value_name("PRINCIPAL")
                .help("The principal at the time you started investing. Defaults to 0"),
        )
        .arg(
            Arg::new("contribution")
                .short('c')
                .long("contribution")
                .value_name("CONTRIBUTION")
                .help("The monthly contribution amount. Defaults to 1"),
        )
        .arg(
            Arg::new("rate")
                .short('r')
                .long("rate")
                .value_name("RATE")
                .help("The annual interest rate (in %). Defaults to 5"),
        )
        .arg(
            Arg::new("years")
                .short('y')
                .long("years")
                .value_name("YEARS")
                .help("The number of years for contributions. Defaults to 5"),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output as JSON. Defaults to false")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("`cls --help` for usage");
        return;
    }

    let investment = Investment::from_matches(&matches);
    // Display the yearly summary
    let summary = investment.yearly_summary();
    if matches.get_flag("json") {
        match to_string_pretty(&summary) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Failed to serialize to JSON: {}", e),
        }
    } else {
        match plot_summary(&summary) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to plot summary: {}", e),
        }
    }
}

fn main() {
    run();
}
