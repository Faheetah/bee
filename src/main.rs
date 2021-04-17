use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Output {
    rc: i32,
    stdout: Vec<String>,
    stderr: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg = args[0].clone();
    let command = Path::new(&first_arg).file_name().unwrap();
    if command == OsStr::new("bee") {
        run_command(args)
    } else {
        run_wrapped_command(args)
    }
}

fn run_command(args: Vec<String>) {
    let (arg_0, cmd_args) = args.split_at(2);
    let output = Command::new(&arg_0[1])
        .args(cmd_args)
        .output()
        .expect("failed to execute process");
    let data = Output {
        rc: output.status.code().unwrap(),
        stdout: parse_std_message(output.stdout),
        stderr: parse_std_message(output.stderr),
    };
    let json_string = serde_json::to_string_pretty(&data);
    println!("{}", json_string.unwrap())
}

fn run_wrapped_command(args: Vec<String>) {
    let (arg_0, cmd_args) = args.split_at(2);
    let command = Path::new(&arg_0[0]).file_name().unwrap();
    let output = Command::new(command)
        .args(cmd_args)
        .output()
        .expect("failed to execute process");
    let data = Output {
        rc: output.status.code().unwrap(),
        stdout: parse_std_message(output.stdout),
        stderr: parse_std_message(output.stderr),
    };
    let json_string = serde_json::to_string_pretty(&data);
    println!("{}", json_string.unwrap())
}

fn parse_std_message(message: Vec<u8>) -> Vec<String> {
    return String::from_utf8(message)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
}
