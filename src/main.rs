mod parameters;

use env_logger::Env;
use clap::Parser;
use crate::parameters::CommandLineArgs;

fn execute_node(node: String){
    println!("*** HOST: {} (Member of <Specified hostnames> [1/2])", node)
}

fn main() {
    unsafe { libc::umask(0o077) };
    let args = CommandLineArgs::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(args.log_level)
    ).format_timestamp_secs().init();

    for node in args.items{
        execute_node(node);
    }
}
