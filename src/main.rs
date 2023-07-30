mod cli;

extern crate log;
use clap::Parser;
// use config::Config;
use log::debug;

use crate::cli::Commands;

fn main() {
    // RUST_LOG=debug
    env_logger::init();
    let cli = cli::Cli::parse();
    debug!("lobo startup");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Test { config: _} => {
            // create the handlebars registry
            print!("Hello there");
        }
    }
}
