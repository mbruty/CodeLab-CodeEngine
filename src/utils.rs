use std::fs;
use std::process::{Command, Output};

pub fn write_to_file(content: &str, file_name: &str) {
    fs::write("./unsafe/".to_owned() + file_name, content)
        .expect("Unable to write file");
}

pub fn exec_command<'a>(command: &str, args: Vec<&str>) -> String {
    let child = Command::new(command)
        .current_dir("./unsafe")
        .args(args)
        .output()
        .expect(&*format!("Could not spawn child process: {}", command));
    String::from_utf8(child.stdout).expect("Stdout was not a string")
}

pub fn exec_command_output<'a>(command: &str, args: Vec<&str>) -> Output {
    Command::new(command)
        .current_dir("./unsafe")
        .args(args)
        .output()
        .expect(&*format!("Could not spawn child process: {}", command))
}

pub fn get_stats(pid: &str) -> String {
    exec_command("pidstat", Vec::from(["-u", "-r", "-p", pid]))
}

pub fn process_stats(stats: &str) -> Vec<&str> {
    let lines = stats.split("\n").collect::<Vec<&str>>();
    if lines.len() < 8 {
        return Vec::new();
    }
    Vec::from([
        lines[3].split_whitespace().collect::<Vec<&str>>()[7],
        lines[6].split_whitespace().collect::<Vec<&str>>()[6]
    ])
}