use std::{fs, io};
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};
use std::io::BufRead;
use std::path::Path;
use std::process::exit;
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
    Error,
    Debug,
    Fatal,
}

pub fn output(msg: String, level: OutputType) {
    if level == OutputType::Fatal {
        println!("{color_red}{msg}{color_reset}");
        exit(1);
    }else if level == OutputType::Info {
        println!("{color_green}{msg}{color_reset}")
    }else if level == OutputType::Debug {
        println!("{color_yellow}{msg}{color_reset}")
    }else{
        println!("{color_red}{msg}{color_reset}")
    }
}

pub fn output_str(msg: &str, level: OutputType) {
    output(msg.to_string(), level);
}

pub fn dump_recipe_dir(recipe_dir: String){
    let mut header = false;
    let path = Path::new(recipe_dir.as_str());

    if !path.is_dir() {
        return;
    }
    let mut table_data: Vec<Vec<CellStruct>> = vec![];

    for entry_result in fs::read_dir(recipe_dir.clone()).expect("Can't read recipe dir") {
        if !header {
            println!("Directory: {}", recipe_dir);
            println!();
            header = true;
        }
        let entry = entry_result.unwrap();
        let path = entry.path();

        if path.is_file() {
            let full_name = path.to_str().unwrap();
            let base_name = path.file_name().unwrap().to_str().unwrap();
            table_data.push(vec![
                base_name.cell().justify(Justify::Left),
                full_name.cell().justify(Justify::Left)]);
        }
    }

    print_stdout(table_data.table().title(vec![
        "Shortcut".cell().bold(true),
        "Full Path".cell().bold(true),
    ]).bold(true)).expect("Unable to print table to stdout");
    println!();
}

pub fn dump_recipes() {
    output("\nAvailable recipes:\n".to_string(), OutputType::Info);

    let recipe_dirs = [
        format!("{}/.hostctl/recipe/", env!("HOME")),
        format!("{}/recipe/", env!("PWD")),
    ];
    for recipe_dir in &recipe_dirs {
        dump_recipe_dir(recipe_dir.to_string());
    }
}

pub fn get_execution_lines(command: &String, recipe: &String) -> Vec<String> {
    let mut raw_execution_lines: Vec<String> = Vec::new();
    if command != "" {
        raw_execution_lines.push(format!("{command}\n"));
    }
    else if recipe != "" {
        let recipe_files = [
            format!("{}/.hostctl/{}", env!("HOME"), recipe),
            format!("{}/recipe/{}", env!("PWD"), recipe),
            format!("{}", recipe),
        ];
        for recipe_file in &recipe_files {
            if let Ok(lines) = read_lines(recipe_file) {
                    for line in lines {
                        raw_execution_lines.push(line.unwrap().clone());
                    }
                    break;
                }
        }
        if raw_execution_lines.len() == 0 {
            output("Did not found a recipe or recipe was empty".to_string(),OutputType::Fatal);
        }
    }

    raw_execution_lines
}

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where
        P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
