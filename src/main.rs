mod parameters;

use env_logger::Env;
use clap::Parser;
use inline_colorization::*;
use crate::parameters::CommandLineArgs;

fn execute_node(node: String, iter_information: String){
    println!("{color_green}*** HOST: {} (Member of <Specified hostnames> {iter_information}){color_reset}", node)
}

fn main() {
    unsafe { libc::umask(0o077) };
    let args = CommandLineArgs::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(args.log_level)
    ).format_timestamp_secs().init();

    let nodes = args.items;
    let number_of_nodes = nodes.len();
    let mut number_of_current = 0;

    for node in nodes{
        number_of_current += 1;
        let iter_information= format!("[{number_of_current}/{number_of_nodes}]");
        execute_node(node, iter_information);
    }
}
