use clap::{Arg, ArgMatches, Command};

/// Builds the CLI command structure for the Compound Interest Calculator.
///
/// This function defines the main command and its arguments, as well as a subcommand for server mode.
///
/// # Returns
///
/// A `Command` instance configured with the necessary arguments and subcommands.
pub fn build_cli() -> Command {
    Command::new("Compound Interest Calculator")
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
        .subcommand(
            Command::new("server")
                .about("Starts the server mode")
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("PORT")
                        .help("The port to run the server on. Defaults to 8080"),
                ),
        )
}

/// Retrieves the port number from the CLI matches.
///
/// This function extracts and parses the port number from the subcommand matches. If no port is
/// specified, it returns the default port 8080.
///
/// # Arguments
///
/// * `matches` - The `ArgMatches` instance containing the parsed CLI arguments and subcommands.
///
/// # Returns
///
/// The port number as a `u16`.
pub fn get_port(matches: &ArgMatches) -> u16 {
    matches
        .get_one::<String>("port")
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}
