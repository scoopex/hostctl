use crate::utils::output;
use crate::utils::OutputType;


use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

fn execute_local(templated_lines: Vec<String>) -> bool {
    let mut temp_file = NamedTempFile::with_prefix("hostctl_").expect("Unable to create temporary file");

    for (nr, line) in templated_lines.iter().enumerate() {
        output(format!("#{nr}: {line}"), OutputType::Debug);
        writeln!(temp_file, "{}", line).expect("Unable to write file");
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
        output(format!("FAILED, EXITCODE WAS : {}", status.code().unwrap()),OutputType::Error);
        return false;
    }
    output("\nSUCCESS".to_string(), OutputType::Info);
    return true;
}


pub fn execute_node(node: String, iter_information: String, local_execution: bool, execution_lines: &Vec<String>) -> bool {
    let mut output_text: String = format!("*** HOST: {node} {iter_information}\n");
    if local_execution {
        output_text = format!("*** LOCAL: {node} {iter_information}\n");
    }
    output(output_text, OutputType::Info);
    let templated_lines = template_lines(execution_lines, &node);

    if local_execution {
        return execute_local(templated_lines);
    }else{
        output("NOT IMPLEMENTED".to_string(), OutputType::Error);
        return false;
    }
}

fn template_lines(execution_lines: &Vec<String>, node: &String) -> Vec<String> {
    let mut templated_commands: Vec<String> = Vec::new();
    for line in execution_lines {
        templated_commands.push(line.clone().replace("HOST", &node));
    }
    templated_commands
}

pub fn execute_nodes(nodes: Vec<String>, only_nodes: bool, execute_local: bool, execution_lines: &Vec<String>) -> i32 {
    let number_of_nodes = nodes.len();
    let mut number_of_current = 0;
    let mut failed_host = 0;
    for node in nodes {
        number_of_current += 1;
        let iter_info: String;
        if only_nodes {
            iter_info = format!(" [{number_of_current}/{number_of_nodes}]");
        } else {
            let membership_info = "Nodes";
            iter_info = format!("({membership_info} [{number_of_current}/{number_of_nodes}])");
        }
        if execute_node(node, iter_info.to_string(), execute_local, &execution_lines){
            failed_host += 1;
        }
    }
    return failed_host;
}