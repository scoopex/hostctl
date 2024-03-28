mod parameters;

use env_logger::Env;
use clap::Parser;
use crate::parameters::CommandLineArgs;


fn main() {
    unsafe { libc::umask(0o077) };
    let args = CommandLineArgs::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(args.log_level)
    ).format_timestamp_secs().init();
}
