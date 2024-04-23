use std::vec;
use crate::utils::output;
use crate::utils::OutputType;

pub fn execute_node(node: String, iter_information: String, local_execution: bool, execution_lines: &Vec<String>) {
    let mut output_text: String = format!("*** HOST: {node} {iter_information}");
    if local_execution {
        output_text = format!("*** LOCAL: {node} {iter_information}");
    }
    output(output_text, OutputType::Info);
    let templated_lines = template_lines(execution_lines);
    for line in templated_lines {
        output(line.clone(), OutputType::Debug);
    }
}

fn template_lines(execution_lines: &Vec<String>) -> Vec<String> {
    let mut templated_commands: Vec<String> = Vec::new();
    for line in execution_lines {
        // replace
        templated_commands.push(line.clone());
    }
    templated_commands
}
