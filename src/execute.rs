use crate::utils::{output, output_str};
use crate::utils::OutputType;


use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use inquire::{InquireError, Select};
use regex::Regex;
use tempfile::NamedTempFile;
use crate::parameters::CommandLineArgs;


fn check_screen(args: &CommandLineArgs) -> usize{
    let cmd = Command::new("screen").args(["-ls", &*args.inscreen]).output().expect("Unable to execute screen");
    let stdout = String::from_utf8_lossy(&cmd.stdout);

    let re = Regex::new(r"\s*\d+\..*\(.*\)\s*\(.*\)").unwrap();
    stdout.lines()
        .filter(|line| re.is_match(line))
        .count()
}

fn establish_screen_session(args: &CommandLineArgs){
    for _num in 1..30{
        match check_screen(args){
            0 => {
                Command::new("screen")
                    .args(["-t", "init", "-S", &*args.inscreen, "-d", "-m", "sleep", "600"])
                    .output()
                    .expect("failed to start base screen");
                println!("Wait for screen session");
                thread::sleep(Duration::from_secs(1));
            },
            1 => {
                if _num == 1 {
                    output(format!("FAILED: There already a screen with the name >>>{}<<<", args.inscreen), OutputType::Fatal)
                }
                thread::sleep(Duration::from_secs(1));
                break;
            },
            _ => {
                output(format!("FAILED: There more screen with the name >>>{}<<<", args.inscreen), OutputType::Fatal)
            }
        }
    }

    if check_screen(args) != 1{
        output(format!("FAILED: Failed to initialize a screen with name >>>{}<<<", args.inscreen), OutputType::Fatal)
    }
    Command::new("screen")
        .args(["-x", &*args.inscreen, "-m","-X","defscrollback", "10000"])
        .output()
        .expect("failed to configure base screen");

    Command::new("screen")
        .args(["-x", &*args.inscreen, "-m","-X","caption", "always", "%3n %t%? @%u%?%? [%h]%?"])
        .output()
        .expect("failed to configure base screen");

    Command::new("screen")
        .args(["-x", &*args.inscreen, "-m","-X","caption", "string", "%{.ck} %n %t %{.gk}"])
        .output()
        .expect("failed to configure base screen");

    Command::new("screen")
        .args(["-x", &*args.inscreen, "-m","-X","hardstatus", "alwayslastline"])
        .output()
        .expect("failed to configure base screen");

    Command::new("screen")
        .args(["-x", &*args.inscreen, "-m","-X","hardstatus", "string", "%{.rw}%c:%s [%l] %{.bw} %n %t %{.wk} %W %{.wk}"])
        .output()
        .expect("failed to configure base screen");

    output(format!("Base screen established >>>{}<<<", args.inscreen), OutputType::Detail)
}
fn establish_base_command(args: &CommandLineArgs, base_executable: &str, node: &str) -> Command {
    let mut cmd;
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    if args.inscreen == "" {
        return Command::new(base_executable);
    }

    if COUNTER.fetch_add(1, Ordering::Relaxed) == 0 {
        establish_screen_session(args);
    }

    // TODO: Close the window with the title "init"
    // Command::new("screen")
    //     .args(["-x", &*args.inscreen, "-m","-X","hardstatus", "string", "%{.rw}%c:%s [%l] %{.bw} %n %t %{.wk} %W %{.wk}"])
    //     .output()
    //     .expect("failed to configure base screen");

    output(format!("NOTE: command were execute in a screen session, attach by executing 'screen -x {}'", args.inscreen), OutputType::Info);
    output_str("(see 'man screen' or 'STRG + a :help' for getting information about handling screen sessions)", OutputType::Info);

    cmd = Command::new("screen");
    cmd.args(["-x", &*args.inscreen, "-m", "-X", "screen", "-t", node]);
    cmd.arg(base_executable);
    cmd
}

fn execute_local(node: String, templated_lines: Vec<String>, args: &CommandLineArgs) -> bool {
    let mut temp_file = NamedTempFile::with_prefix("hostctl_")
        .expect("Unable to create temporary file");

    for (nr, line) in templated_lines.iter().enumerate() {
        writeln!(temp_file, "{}\n", line).expect(format!("Unable to write file in line {}", nr).as_str());
    }
    let temp_file_path = temp_file.path().to_str();

    let mut cmd = establish_base_command(&args, "bash", &node);

    cmd.arg(temp_file_path.unwrap().to_string());

    println!("Executing local: {:?}", cmd);

    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = cmd.spawn().expect("Unable to start script");

    let status = child.wait().expect("Failed to wait for script");
    if !status.success() {
        output(format!("FAILED, EXITCODE WAS : {}\n", status.code().unwrap()), OutputType::Error);
        return false;
    }
    output("\nSUCCESS\n".to_string(), OutputType::Info);
    true
}

fn send_screen_stdin(screen: String, node: String, line: String, line_nr: usize) -> bool{
    let the_line = format!("{}^M", line);
    let cmd_args = ["-S", &*screen,"-p", &*node,"-X", "stuff", the_line.as_str()];
    //println!("send: {:?}", cmd_args);
    let result = Command::new("screen")
        .args( cmd_args)
        .stderr(Stdio::inherit()) // Print any error messages to the console
        .output(); // Run the command and capture the output

    match result {
        Ok(cmd_output) => {
            if cmd_output.status.success() {
                true
            } else {
                output(format!("FAILED IN LINE  {} : {}\n", line_nr, cmd_output.status), OutputType::Error);
                false
            }
        }
        Err(e) => {
            output(format!("FAILED IN LINE {} : {}\n", line_nr, e), OutputType::Error);
            false
        }
    }
}

fn execute_remote(node: String, templated_lines: Vec<String>, args: &CommandLineArgs) -> bool {

    let mut cmd = establish_base_command(&args, "ssh", &node);

    if args.batchmode {
        cmd.arg("-o");
        cmd.arg("BatchMode=yes");
    }

    if args.term || args.inscreen != "" {
        cmd.arg("-t");
    }

    if args.optssh != "" {
        output(format!("Adding extra ssh options >>>{}<<<", args.optssh), OutputType::Detail);
        for ssh_opt in args.optssh.split_whitespace() {
            cmd.arg(ssh_opt.to_string());
        }
    }
    cmd.arg(node.clone());
    cmd.arg("bash -s");

    println!("Executing remote: {:?}", cmd);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());


    let mut child = cmd.spawn().expect("Unable to start remote connection");

    if args.inscreen == "" {
        if let Some(mut stdin) = child.stdin.take() {
            for line in templated_lines.iter() {
                writeln!(stdin, "{}\n", line).expect("Failed to write to stdin");
            }
        } else {
            eprintln!("Failed to open stdin");
        }
    }else{
        thread::sleep(Duration::from_secs(1));
        for (nr, line) in templated_lines.iter().enumerate() {
            let res = send_screen_stdin(args.inscreen.clone(), node.clone(), line.clone(), nr);
            if !res{
                false;
            }
        }
    }

    let status = child.wait().expect("Failed to wait for script");
    if !status.success() {
        output(format!("FAILED, EXITCODE WAS : {}\n", status.code().unwrap()), OutputType::Error);
        false;
    }
    output("\nSUCCESS\n".to_string(), OutputType::Info);
    true
}

#[derive(Debug, PartialEq)]
pub enum NodeResult {
    Ok,
    Failed,
    Skipped,
    Quit,
}

pub fn execute_node(node: String, iter_information: String, local_execution: bool, execution_lines: &Vec<String>, args: &CommandLineArgs) -> NodeResult {
    let mut output_text: String = format!("*** HOST: {node} {iter_information}\n");
    if local_execution {
        output_text = format!("*** LOCAL: {node} {iter_information}\n");
    }
    output(output_text, OutputType::Info);

    let templated_lines = template_lines(execution_lines, &node);

    for (nr, line) in templated_lines.iter().enumerate() {
        output(format!("#{nr}: {line}"), OutputType::Detail);
    }

    if args.makeselection {
        match prompt_before() {
            "Execute" => {},
            "Skip" => { return NodeResult::Skipped },
            "Quit" => { return NodeResult::Quit },
            _ => {
                output_str("Interrupted from choice", OutputType::Fatal);
            }
        }
    }
    println!();

    let res: bool;
    if local_execution {
        res = execute_local(node, templated_lines, args);
    } else {
        res = execute_remote(node, templated_lines, args);
    }

    if res {
        NodeResult::Ok
    }else{
        NodeResult::Failed
    }
}

fn template_lines(execution_lines: &Vec<String>, node: &String) -> Vec<String> {
    let mut templated_commands: Vec<String> = Vec::new();
    for line in execution_lines {
        templated_commands.push(line.clone().replace("HOST", &node));
    }
    templated_commands
}

pub fn prompt_before() -> &'static str {
    let options = vec!["Execute", "Skip", "Quit"];
    let ans: Result<&str, InquireError> = Select::new("What to you want to do?", options).prompt();
    ans.unwrap_or_else(|_| "ERROR")
}

pub fn prompt_after() -> &'static str {
    let options = vec!["Continue", "Retry", "Shell", "Quit"];
    let ans: Result<&str, InquireError> = Select::new("What to you want to do?", options).prompt();
    ans.unwrap_or_else(|_| "ERROR")
}

pub fn get_shell(node: String, args: &CommandLineArgs) {
    let mut execution_lines: Vec<String> = Vec::new();
    execution_lines.push(format!("ssh HOST"));
    execute_node(node, "Shell".to_string(), true, &execution_lines, args);
}

pub fn execute_nodes(nodes: Vec<String>, only_nodes: bool, execute_local: bool, execution_lines: &Vec<String>, args: CommandLineArgs) -> i32 {
    let number_of_nodes = nodes.len();
    let mut number_of_current = 0;
    let mut failed_nodes: Vec<String> = Vec::new();

    if nodes.len() == 0 {
        output_str("EXIT: No nodes were specified", OutputType::Fatal);
    }

    'node_loop: for node in nodes {
        number_of_current += 1;
        let iter_info: String;
        if only_nodes {
            iter_info = format!(" [{number_of_current}/{number_of_nodes}]");
        } else {
            let membership_info = "Nodes";
            iter_info = format!("({membership_info} [{number_of_current}/{number_of_nodes}])");
        }

        'outer: loop {
            let res = execute_node(node.clone(), iter_info.to_string(), execute_local, &execution_lines, &args);

            match res {
                NodeResult::Failed => { failed_nodes.push(node.clone());},
                NodeResult::Quit => {failed_nodes.push(node.clone()); break 'node_loop},
                _ => {}
            }
            if args.wait > 0 {
                output(format!("\nWaiting for {} seconds before continue\n", args.wait), OutputType::Info);
                thread::sleep(Duration::from_secs(args.wait));
            }
            'inner: loop {
                if args.prompt {
                    // TODO: implement edit, tranche, all
                    match prompt_after() {
                        "Continue" => { break 'outer; }
                        "Shell" => { get_shell(node.clone(), &args) }
                        "Retry" => { break 'inner; }
                        "Quit" => { break 'node_loop; }
                        _ => {
                            output_str("Interrupted from choice", OutputType::Fatal);
                        }
                    }
                } else {
                    break 'outer;
                }
            }
        }
    }

    failed_nodes.sort();
    failed_nodes.dedup();

    if failed_nodes.len() > 0 {
        let failed_nodes_str = failed_nodes.join(", ");
        output(format!("\n\nCOMPLETED  - ONE OR MORE NODES FAILED!\n\nFAILED NODES: {failed_nodes_str}"), OutputType::Error);
    } else {
        output_str("\n\nCOMPLETED - ALL NODES WERE SUCCESSFUL", OutputType::Info);
    }
    failed_nodes.len() as i32
}
