mod parameters;
mod groups_config;
mod utils;
mod execute;

use std::process::exit;
use env_logger::Env;
use clap::Parser;
use clap::CommandFactory;
use clap_complete::Generator;
use crate::groups_config::{dump_batch_list, dump_groups, dump_groups_for_completion, unified_node_list};
use crate::parameters::CommandLineArgs;
use crate::utils::{dump_recipes, output_str, OutputType};

fn main() {
    unsafe { libc::umask(0o077) };

    ctrlc::set_handler(move || {
        output_str("\n\nReceived Ctrl+C! Immediate stop!", OutputType::Fatal);
        exit(0);
    }).expect("Error setting Ctrl-C handler");

    let mut cli = CommandLineArgs::parse();

    parameters::shell_completions();
    // dump_groups_for_completion();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(cli.log_level.clone())
    ).format_timestamp_secs().init();

    let mut new_items: Vec<String> = Vec::new();
    for item in cli.items {
        new_items.push(item.replace(",", ""));
    }
    cli.items = new_items;

    if cli.show {
        if cli.items.len() == 0 {
            cli.items.push("all".to_string());
        }
        dump_groups(cli.items, cli.json);
        exit(0);
    }
    if cli.array {
        dump_batch_list(cli.items);
        exit(0);
    }

    let nodes: Vec<String>;

    let mut only_nodes = false;
    if cli.nodes {
        nodes = cli.items.clone();
        only_nodes = true;
    } else {
        nodes = unified_node_list(cli.items.clone());
    }

    if (cli.command != "") != (cli.recipe != "") {
        let execution_lines: Vec<String> =
            utils::get_execution_lines(&cli);

        if cli.execute_local {
            exit(execute::execute_nodes(nodes, only_nodes, true, &execution_lines, cli));
        } else {
            exit(execute::execute_nodes(nodes, only_nodes, false, &execution_lines, cli));
        }
    }

    if cli.login {
        let mut execution_lines: Vec<String> = Vec::new();
        if cli.sudo {
            execution_lines.push("sudo -i".to_string());
        }else {
            execution_lines.push("bash -i".to_string());
        }
        cli.term = true;
        exit(execute::execute_nodes(nodes, only_nodes, false, &execution_lines, cli));
    }

    let mut cmd = CommandLineArgs::command();
    cmd.print_help().expect("Failed to print help");
    dump_recipes();
}

