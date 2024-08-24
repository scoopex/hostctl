mod parameters;
mod groups_config;
mod utils;
mod execute;

use std::process::exit;
use env_logger::Env;
use clap::Parser;
use clap::CommandFactory;
use crate::groups_config::{dump_batch_mode, dump_groups, unified_node_list};
use crate::parameters::CommandLineArgs;

fn main() {
    unsafe { libc::umask(0o077) };

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    }).expect("Error setting Ctrl-C handler");

    let mut args = CommandLineArgs::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(args.log_level)
    ).format_timestamp_secs().init();

    if args.show {
        if args.items.len() == 0 {
            args.items.push("all".to_string());
        }
        dump_groups(args.items, args.json);
        exit(0);
    }
    if args.batchmode {
        dump_batch_mode(args.items);
        exit(0);
    }

    let nodes: Vec<String>;
    let mut only_nodes = false;
    if args.nodes {
        nodes = args.items;
        only_nodes = true;
    } else {
        nodes = unified_node_list(args.items);
    }

    if (args.command != "") != (args.recipe != ""){
        let execution_lines: Vec<String> =
            utils::get_execution_lines(&args.command, &args.recipe);

        if args.execute_local {
            exit(execute::execute_nodes(nodes, only_nodes, true, &execution_lines, args.optssh));
        } else {
            exit(execute::execute_nodes(nodes, only_nodes, false, &execution_lines, args.optssh));
        }
    }

    if args.login {
        let mut execution_lines: Vec<String> = Vec::new();
        execution_lines.push(format!("ssh {} HOST", args.optssh));
        exit(execute::execute_nodes(nodes, only_nodes, true, &execution_lines,args.optssh));
    }

    let mut cmd = CommandLineArgs::command();
    cmd.print_help().expect("Failed to print help");

}

