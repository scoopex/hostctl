mod parameters;
mod groups_config;
mod utils;
mod execute;

use std::process::exit;
use env_logger::Env;
use clap::Parser;
use clap::CommandFactory;
use crate::groups_config::{dump_batch_list, dump_groups, unified_node_list};
use crate::parameters::CommandLineArgs;
use crate::utils::{dump_recipes, output_str, OutputType};

fn main() {
    unsafe { libc::umask(0o077) };

    ctrlc::set_handler(move || {
        output_str("\n\nReceived Ctrl+C! Immediate stop!", OutputType::Fatal);
        exit(0);
    }).expect("Error setting Ctrl-C handler");

    let mut commandline_args = CommandLineArgs::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(commandline_args.log_level.clone())
    ).format_timestamp_secs().init();

    if commandline_args.show {
        if commandline_args.items.len() == 0 {
            commandline_args.items.push("all".to_string());
        }
        dump_groups(commandline_args.items, commandline_args.json);
        exit(0);
    }
    if commandline_args.array {
        dump_batch_list(commandline_args.items);
        exit(0);
    }

    let nodes: Vec<String>;

    let mut only_nodes = false;
    if commandline_args.nodes {
        nodes = commandline_args.items.clone();
        only_nodes = true;
    } else {
        nodes = unified_node_list(commandline_args.items.clone());
    }

    if (commandline_args.command != "") != (commandline_args.recipe != "") {
        let execution_lines: Vec<String> =
            utils::get_execution_lines(&commandline_args);

        if commandline_args.execute_local {
            exit(execute::execute_nodes(nodes, only_nodes, true, &execution_lines, commandline_args));
        } else {
            exit(execute::execute_nodes(nodes, only_nodes, false, &execution_lines, commandline_args));
        }
    }

    if commandline_args.login {
        let mut execution_lines: Vec<String> = Vec::new();
        execution_lines.push(format!("ssh {} HOST", commandline_args.optssh));
        exit(execute::execute_nodes(nodes, only_nodes, true, &execution_lines, commandline_args));
    }

    let mut cmd = CommandLineArgs::command();
    cmd.print_help().expect("Failed to print help");
    dump_recipes();
}

