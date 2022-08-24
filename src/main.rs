mod cli;
mod errors;
use log;
mod ipcs;
use std::process::exit;
mod config;
mod container;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            container::start(args);
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
}
