mod parameters;
mod groups_config;
mod utils;

use std::process::exit;
use env_logger::Env;
use clap::Parser;
use inline_colorization::*;
use crate::groups_config::{dump_batch_mode, dump_groups, unified_node_list};
use crate::parameters::CommandLineArgs;

fn execute_node(node: String, iter_information: String) {
    println!("{color_green}*** HOST: {} (Member of <Specified hostnames> {iter_information}){color_reset}", node)
}

fn main() {
    unsafe { libc::umask(0o077) };

    let args = CommandLineArgs::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(args.log_level)
    ).format_timestamp_secs().init();

    if args.show {
        dump_groups(args.items, args.json);
        exit(0);
    }
    if args.batchmode {
        dump_batch_mode(args.items);
        exit(0);
    }

    let nodes: Vec<String>;
    if args.nodes {
        nodes = args.items;
    }else{
        nodes =  unified_node_list(args.items);
    }

    let number_of_nodes = nodes.len();
    let mut number_of_current = 0;

    for node in nodes{
        number_of_current += 1;
        let iter_info: String;
        if args.nodes {
            iter_info = format!(" [{number_of_current}/{number_of_nodes}]");
        }else {
            let membership_info = "Nodes";
            iter_info = format!("({membership_info} [{number_of_current}/{number_of_nodes}])");
        }
        execute_node(node, iter_info);
    }
}