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
    let max_wait = 30;
    for num in 1..max_wait{
        match check_screen(args){
            0 => {
                Command::new("screen")
                    .args(["-t", "init", "-S", &*args.inscreen, "-d", "-m", "sleep", "120"])
                    .output()
                    .expect("failed to start base screen");
                output(format!("Wait for screen session for {} seconds", num), OutputType::Info);
                thread::sleep(Duration::from_secs(1));
            },
            1 => {
                if num == 1 {
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
        let mut cmd = Command::new(base_executable);
        if args.knownhostsaccept {
            cmd.args(["-o", "StrictHostKeyChecking=accept-new"]);
        }
        return cmd;
    }

    if COUNTER.fetch_add(1, Ordering::Relaxed) == 0 {
        establish_screen_session(args);
    }

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

    output(format!("Executing local: {:?}", cmd), OutputType::Detail);

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

fn send_screen_stdin(screen: String, node: String, templated_lines: Vec<String>){
    let mut temp_file = NamedTempFile::with_prefix("hostctl_")
        .expect("Unable to create temporary file");

    for (nr, line) in templated_lines.iter().enumerate() {
        writeln!(temp_file, "{}\n", line).expect(format!("Unable to write file in line {}", nr).as_str());
    }
    let temp_file_path = temp_file.path().to_str();

    // read the contents from the temporary file
    Command::new("screen")
        .args(["-S", &*screen,"-p", &*node,"-X", "readbuf", temp_file_path.unwrap()])
        .output()
        .expect(&format!("failed to do a readbuf on file {} for node", temp_file_path.unwrap()).as_str());

    // paste the buffer
    Command::new("screen")
        .args(["-S", &*screen,"-p", &*node,"-X", "paste", "."])
        .output()
        .expect("failed to paste the commands");
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
    if args.inscreen != "" {
        if args.sudo {
            cmd.arg("sudo bash -s");
        }else{
            cmd.arg("bash -s");
        }
    }else{
        if args.sudo{
            cmd.arg(format!("sudo bash -c '{}'", templated_lines.join("\n")));
        }else{
            cmd.arg(templated_lines.join("\n"));
        }
    }

    output(format!("Execute remote : {:?}", cmd), OutputType::Detail);
    if args.term {
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    }else {
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    }

    let mut child = cmd.spawn().expect("Unable to start remote connection");

    if !args.term {
        if args.inscreen == "" {
            if let Some(mut stdin) = child.stdin.take() {
                for line in templated_lines.iter() {
                    writeln!(stdin, "{}\n", line).expect("Failed to write to stdin");
                }
            } else {
                output("Failed to open stdin".to_string(), OutputType::Fatal);
            }
        } else {
            thread::sleep(Duration::from_secs(1));
            send_screen_stdin(args.inscreen.clone(), node.clone(), templated_lines);
        }
    }

    let status = child.wait().expect("Failed to wait for script");
    if !status.success() {
        output(format!("FAILED, EXITCODE WAS : {}\n", status.code().unwrap()), OutputType::Error);
        return false;
    }
    output("\nSUCCESS\n".to_string(), OutputType::Info);
    return true;
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
        // TODO: Use a more unambiguous tag
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
    let mut successful_nodes: Vec<String> = Vec::new();

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
                NodeResult::Ok => {successful_nodes.push(node.clone());},
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

    if args.inscreen != "" {
        output_str("NOTE: Teardown 'init' screen now", OutputType::Detail);
        Command::new("screen")
            .args(["-x", &*args.inscreen,"-p","init", "-X","kill"])
            .output()
            .expect("Failed to teadown 'init' screen");

        output(format!("NOTE: command were executed in a screen session, attach to the screen session by executing 'screen -x {}'", args.inscreen), OutputType::Info);
        output_str("(see 'man screen' or 'STRG + a :help' for getting information about handling screen sessions)", OutputType::Info);

    }

    failed_nodes.sort();
    failed_nodes.dedup();

    if failed_nodes.len() > 0 {
        let failed_nodes_str = failed_nodes.join(", ");
        output(format!("\n\nCOMPLETED  - ONE OR MORE NODES FAILED!\n\nFAILED NODES: {failed_nodes_str}\n"), OutputType::Error);
        if successful_nodes.len() > 0{
            let successful_nodes_str = successful_nodes.join(", ");
            output(format!("SUCCESSFUL NODES: {successful_nodes_str}\n"), OutputType::Info);
        }
    } else {
        output_str("\n\nCOMPLETED - ALL NODES WERE SUCCESSFUL", OutputType::Info);
    }
    failed_nodes.len() as i32
}
