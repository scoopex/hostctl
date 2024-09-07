use crate::utils::{output, output_str};
use crate::utils::OutputType;


use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use inquire::{InquireError, Select};
use tempfile::NamedTempFile;
use crate::parameters::CommandLineArgs;

fn execute_local(templated_lines: Vec<String>) -> bool {
    let mut temp_file = NamedTempFile::with_prefix("hostctl_")
        .expect("Unable to create temporary file");

    for (nr, line) in templated_lines.iter().enumerate() {
        writeln!(temp_file, "{}", line).expect(format!("Unable to write file in line {}", nr).as_str());
    }
    let temp_file_path = temp_file.path().to_str();

    let mut cmd = Command::new("bash");
    cmd.arg(temp_file_path.unwrap().to_string());

    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = cmd.spawn().expect("Unable to start script");

    let status = child.wait().expect("Failed to wait for script");
    if !status.success() {
        output(format!("FAILED, EXITCODE WAS : {}\n", status.code().unwrap()), OutputType::Error);
        return false;
    }
    output("\nSUCCESS\n".to_string(), OutputType::Info);
    true
}

fn execute_remote(node: String, templated_lines: Vec<String>, ssh_options: String) -> bool {

    let mut cmd = Command::new("ssh");
    if ssh_options != "" {
        output(format!("Adding extra ssh options >>>{}<<<", ssh_options), OutputType::Detail);
        for ssh_opt in ssh_options.split_whitespace() {
            cmd.arg(ssh_opt.to_string());
        }
    }
    cmd.arg(node);
    cmd.arg("bash");
    cmd.arg("-s");

    //cmd.arg(temp_file_path.unwrap().to_string());

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = cmd.spawn().expect("Unable to start remote connection");

    if let Some(mut stdin) = child.stdin.take() {
        for line in templated_lines.iter() {
            writeln!(stdin, "{}", line).expect("Failed to write to stdin");
        }
    } else {
        eprintln!("Failed to open stdin");
    }

    let status = child.wait().expect("Failed to wait for script");
    if !status.success() {
        output(format!("FAILED, EXITCODE WAS : {}\n", status.code().unwrap()), OutputType::Error);
        return false;
    }
    output("\nSUCCESS\n".to_string(), OutputType::Info);
    true
}

#[derive(Debug, PartialEq)]
pub enum NodeResult {
    Ok,
    Failed,
    Skipped,
    Quit,
}

pub fn execute_node(node: String, iter_information: String, local_execution: bool, execution_lines: &Vec<String>, args: &CommandLineArgs) -> NodeResult {
    let mut output_text: String = format!("*** HOST: {node} {iter_information}\n");
    if local_execution {
        output_text = format!("*** LOCAL: {node} {iter_information}\n");
    }
    output(output_text, OutputType::Info);

    let templated_lines = template_lines(execution_lines, &node);

    for (nr, line) in templated_lines.iter().enumerate() {
        output(format!("#{nr}: {line}"), OutputType::Detail);
    }

    if args.makeselection {
        match prompt_before() {
            "Execute" => {},
            "Skip" => { return NodeResult::Skipped },
            "Quit" => { return NodeResult::Quit },
            _ => {
                output_str("Interrupted from choice", OutputType::Fatal);
            }
        }
    }
    println!();

    let res: bool;
    if local_execution {
        res = execute_local(templated_lines);
    } else {
        res = execute_remote(node, templated_lines, args.optssh.clone());
    }

    if res {
        NodeResult::Ok
    }else{
        NodeResult::Failed
    }
}

fn template_lines(execution_lines: &Vec<String>, node: &String) -> Vec<String> {
    let mut templated_commands: Vec<String> = Vec::new();
    for line in execution_lines {
        templated_commands.push(line.clone().replace("HOST", &node));
    }
    templated_commands
}

pub fn prompt_before() -> &'static str {
    let options = vec!["Execute", "Skip", "Quit"];
    let ans: Result<&str, InquireError> = Select::new("What to you want to do?", options).prompt();
    ans.unwrap_or_else(|_| "ERROR")
}

pub fn prompt_after() -> &'static str {
    let options = vec!["Continue", "Retry", "Shell", "Quit"];
    let ans: Result<&str, InquireError> = Select::new("What to you want to do?", options).prompt();
    ans.unwrap_or_else(|_| "ERROR")
}

pub fn get_shell(node: String, args: &CommandLineArgs) {
    let mut execution_lines: Vec<String> = Vec::new();
    execution_lines.push(format!("ssh HOST"));
    execute_node(node, "Shell".to_string(), true, &execution_lines, args);
}

pub fn execute_nodes(nodes: Vec<String>, only_nodes: bool, execute_local: bool, execution_lines: &Vec<String>, args: CommandLineArgs) -> i32 {
    let number_of_nodes = nodes.len();
    let mut number_of_current = 0;
    let mut failed_nodes: Vec<String> = Vec::new();

    if nodes.len() == 0 {
        output_str("EXIT: No nodes were specified", OutputType::Fatal);
    }

    'node_loop: for node in nodes {
        number_of_current += 1;
        let iter_info: String;
        if only_nodes {
            iter_info = format!(" [{number_of_current}/{number_of_nodes}]");
        } else {
            let membership_info = "Nodes";
            iter_info = format!("({membership_info} [{number_of_current}/{number_of_nodes}])");
        }

        'outer: loop {
            let res = execute_node(node.clone(), iter_info.to_string(), execute_local, &execution_lines, &args);

            match res {
                NodeResult::Failed => { failed_nodes.push(node.clone());},
                NodeResult::Quit => {failed_nodes.push(node.clone()); break 'node_loop},
                _ => {}
            }
            if args.wait > 0 {
                output(format!("\nWaiting for {} seconds before continue\n", args.wait), OutputType::Info);
                thread::sleep(Duration::from_secs(args.wait));
            }
            'inner: loop {
                if args.prompt {
                    // TODO: implement edit
                    match prompt_after() {
                        "Continue" => { break 'outer; }
                        "Shell" => { get_shell(node.clone(), &args) }
                        "Retry" => { break 'inner; }
                        "Quit" => { break 'node_loop; }
                        _ => {
                            output_str("Interrupted from choice", OutputType::Fatal);
                        }
                    }
                } else {
                    break 'outer;
                }
            }
        }
    }
    if failed_nodes.len() > 0 {
        let failed_nodes_str = failed_nodes.join(", ");
        output(format!("\n\nCOMPLETED  - ONE OR MORE NODES FAILED!\n\nFAILED NODES: {failed_nodes_str}"), OutputType::Error);
    } else {
        output_str("\n\nCOMPLETED - ALL NODES WERE SUCCESSFUL", OutputType::Info);
    }
    failed_nodes.len() as i32
}
