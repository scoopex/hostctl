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

fn execute_remote(node: String, templated_lines: Vec<String>) -> bool {

    for (nr, line) in templated_lines.iter().enumerate() {
        output(format!("#{nr}: {line}"), OutputType::Debug);
    }

    let mut cmd = Command::new("ssh");
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
            write!(stdin, "{}", line).expect("Failed to write to stdin");
        }
    } else {
        eprintln!("Failed to open stdin");
    }

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
        return execute_remote(node, templated_lines);
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
    let mut failed_nodes: Vec<String> = Vec::new();
    for node in nodes {
        number_of_current += 1;
        let iter_info: String;
        if only_nodes {
            iter_info = format!(" [{number_of_current}/{number_of_nodes}]");
        } else {
            let membership_info = "Nodes";
            iter_info = format!("({membership_info} [{number_of_current}/{number_of_nodes}])");
        }
        if !execute_node(node.clone(), iter_info.to_string(), execute_local, &execution_lines){
            failed_nodes.push(node)
        }
    }
    if failed_nodes.len() > 0 {
        let failed_nodes_str = failed_nodes.join(", ");
        output(format!("\n\nCOMPLETED  - ONE OR MORE NODES FAILED!\n\nFAILED NODES: {failed_nodes_str}"), OutputType::Error);
    }else{
        output(format!("\n\nCOMPLETED - ALL NODES WERE SUCCESSFUL"), OutputType::Info);
    }
    failed_nodes.len() as i32
}