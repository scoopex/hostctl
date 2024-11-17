mod parameters;
mod groups_config;
mod utils;
mod execute;

use std::process::exit;
use env_logger::Env;
use clap::Parser;
use clap::CommandFactory;
use clap_complete::{generate, Generator, Shell};
use crate::groups_config::{dump_batch_list, dump_groups, unified_node_list};
use crate::parameters::CommandLineArgs;
use crate::utils::{dump_recipes, output_str, OutputType};

use std::env;
use std::io;

fn generate_completions<G: Generator>(shell: G) {
    let mut cmd = CommandLineArgs::command();
    let bin_name = env!("CARGO_PKG_NAME");
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}

fn main() {
    unsafe { libc::umask(0o077) };

    ctrlc::set_handler(move || {
        output_str("\n\nReceived Ctrl+C! Immediate stop!", OutputType::Fatal);
        exit(0);
    }).expect("Error setting Ctrl-C handler");

    let mut cli = CommandLineArgs::parse();

    if let Some(shell) = env::args().nth(1) {
        if shell == "generate-completions" {
            if let Some(shell_name) = env::args().nth(2) {
                let shell_enum = shell_name.parse::<Shell>();
                if let Ok(shell_enum) = shell_enum {
                    generate_completions(shell_enum);
                    return;
                }
            }
            eprintln!("Usage: {} generate-completions <shell>", env!("CARGO_PKG_NAME"));
            return;
        }
    }

    env_logger::Builder::from_env(
        Env::default().default_filter_or(cli.log_level.clone())
    ).format_timestamp_secs().init();

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
        execution_lines.push(format!("ssh {} HOST", cli.optssh));
        exit(execute::execute_nodes(nodes, only_nodes, true, &execution_lines, cli));
    }

    let mut cmd = CommandLineArgs::command();
    cmd.print_help().expect("Failed to print help");
    dump_recipes();
}

