

use inline_colorization::*;

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