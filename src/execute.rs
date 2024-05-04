use crate::utils::output;
use crate::utils::OutputType;


use std::io::{self, Write};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

fn execute_local(templated_lines: Vec<String>) -> io::Result<()> {

    let mut temp_file = NamedTempFile::with_prefix("hostctl_")?;

    for (nr, line) in templated_lines.iter().enumerate() {
        output(format!("#{nr}: {line}"), OutputType::Debug);
        writeln!(temp_file, "{}", line)?;
    }
    let temp_file_path = temp_file.path().to_str();

    let mut cmd = Command::new("bash");
    cmd.arg(temp_file_path.unwrap().to_string());

    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = cmd.spawn()?;

    let status = child.wait()?;
    if !status.success() {
        output(format!("FAILED, EXITCODE WAS : {}", status.code().unwrap()),OutputType::Error);
    }
    output("\nSUCCESS".to_string(), OutputType::Info);
    Ok(())
}


pub fn execute_node(node: String, iter_information: String, local_execution: bool, execution_lines: &Vec<String>) {
    let mut output_text: String = format!("*** HOST: {node} {iter_information}\n");
    if local_execution {
        output_text = format!("*** LOCAL: {node} {iter_information}\n");
    }
    output(output_text, OutputType::Info);
    let templated_lines = template_lines(execution_lines, &node);

    if local_execution {
        let _res = execute_local(templated_lines);
    }else{
        output("NOT IMPLEMENTED".to_string(), OutputType::Error);
    }
}

fn template_lines(execution_lines: &Vec<String>, node: &String) -> Vec<String> {
    let mut templated_commands: Vec<String> = Vec::new();
    for line in execution_lines {
        templated_commands.push(line.clone().replace("HOST", &node));
    }
    templated_commands
}
