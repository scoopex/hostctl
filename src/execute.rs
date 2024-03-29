use crate::utils::output;
use crate::utils::OutputType;

pub fn execute_node(node: String, iter_information: String) {
    output(
        format!("*** HOST: {node} {iter_information}").to_string(),
        OutputType::Info
    );
}
