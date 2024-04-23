use crate::utils::output;
use crate::utils::OutputType;

pub fn execute_node(node: String, iter_information: String, local_execution: bool, execution_lines: &Vec<String>) {
    let mut output_text: String = format!("*** HOST: {node} {iter_information}");
    if local_execution {
        output_text = format!("*** LOCAL: {node} {iter_information}");
    }
    output(output_text, OutputType::Info);
    for line in execution_lines {
        output(line.clone(), OutputType::Debug);
    }
}
