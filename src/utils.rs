use std::{fs, io};
use std::io::BufRead;

use inline_colorization::color_green;
use inline_colorization::color_yellow;
use inline_colorization::color_red;
use inline_colorization::color_reset;

/*
use inline_colorization::color_green;
use inline_colorization::color_yellow;
use inline_colorization::color_red;
use inline_colorization::color_reset;
*/


#[derive(Debug, PartialEq)]
pub enum OutputType {
    Info,
    Warn,
    Error,
    Debug,
}


pub fn output(msg: String, level: OutputType){
    if level == OutputType::Info {
        println!("{color_green}{msg}{color_reset}")
    }else if level == OutputType::Debug {
        println!("{color_yellow}{msg}{color_reset}")
    }else{
        println!("{color_red}{msg}{color_reset}")
    }
}

pub fn get_execution_lines(command: &String, recipe: &String, execute_local: &String) -> String {
    let mut execution_lines: Vec<String> = Vec::new();
    if command != "" {
        execution_lines.push(command.clone());
    }
    else if execute_local != "" {
        execution_lines.push(execute_local.clone());
    }
    else if recipe != "" {
        let recipe_files = [
            format!("{}/.hostctl/{}", env!("HOME"),recipe),
            format!("{}/recipe/{}", env!("PWD"),recipe),
        ];
        for recipe_file in &recipe_files {
            println!("----> {}", recipe_file);
            if let Ok(lines) = read_lines(recipe_file) {
                println!("YEAH {}", recipe_file);
                if let Ok(lines) = read_lines(recipe_file) {
                    for line in lines {
                        execution_lines.push(line.unwrap().clone());
                    }
                    break;
                }
            }
        }
    }
    return execution_lines.join("\n");
}

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where
        P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
