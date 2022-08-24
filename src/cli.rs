use crate::errors::Errcode;
use env_logger;
use log::{self, LevelFilter};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "crabcan", about = "A simple container in Rust.")]
pub struct Args {
    // Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    // Command to execute in the container
    #[structopt(short, long)]
    pub command: String,

    // User ID that will be used inside the container
    #[structopt(short, long)]
    pub uid: i32,

    // Mount point for the root of the container
    #[structopt(parse(from_os_str), short = "m", long = "mount")]
    pub mount_point: PathBuf,
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args = Args::from_args();

    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(
            None,
            match args.debug {
                true => LevelFilter::Debug,
                false => LevelFilter::Info,
            },
        )
        .init();

    if !args.mount_point.exists() && !args.mount_point.is_dir() {
        return Err(Errcode::InvalidArgument("mount"));
    }

    Ok(args)
}
