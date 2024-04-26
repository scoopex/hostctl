use crate::utils::output;
use crate::utils::OutputType;

pub fn execute_node(node: String, iter_information: String, local_execution: bool, execution_lines: &Vec<String>) {
    let mut output_text: String = format!("*** HOST: {node} {iter_information}");
    if local_execution {
        output_text = format!("*** LOCAL: {node} {iter_information}");
    }
    output(output_text, OutputType::Info);
    let mut templated_lines = template_lines(execution_lines, &node);
    for line in templated_lines {
        output(line, OutputType::Debug);
    }
}

fn template_lines(execution_lines: &Vec<String>, node: &String) -> Vec<String> {
    let mut templated_commands: Vec<String> = Vec::new();
    for line in execution_lines {
        templated_commands.push(line.clone().replace("HOST", &node));
    }
    templated_commands
}
