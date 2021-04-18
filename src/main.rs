use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Output {
    rc: i32,
    stdout: Vec<String>,
    stderr: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Input {
    stdin: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Path::new(&args[0]).file_name().unwrap();

    let mut buffer = vec![];
    let mut stdin = io::stdin();
    stdin.read_to_end(&mut buffer).unwrap();
    let stdin_str = String::from_utf8(buffer).unwrap();

    let v: Value;
    let stdin_data = if stdin_str == "" {
        ""
    } else {
        v = serde_json::from_str(&stdin_str).unwrap();
        &v["stdin"].as_str().unwrap()
    };

    if command == OsStr::new("bee") {
        let (arg_0, cmd_args) = args.split_at(2);
        run_command(&arg_0[1], cmd_args, stdin_data)
    } else {
        let (arg_0, cmd_args) = args.split_at(1);
        let command = Path::new(&arg_0[0]).file_name().unwrap();
        let s = command.to_str().unwrap().to_owned();
        run_command(&s, cmd_args, stdin_data)
    }
}

fn run_command(cmd: &String, args: &[String], stdin: &str) {
    let mut process = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    process
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    let mut out = process.stdout.take().unwrap();
    let mut err = process.stderr.take().unwrap();

    let mut outbuff = vec![];
    out.read_to_end(&mut outbuff).unwrap();
    let outs = String::from_utf8(outbuff).unwrap();

    let mut errbuff = vec![];
    err.read_to_end(&mut errbuff).unwrap();
    let errs = String::from_utf8(errbuff).unwrap();

    let output = process.wait_with_output().unwrap();

    let data = Output {
        rc: output.status.code().unwrap(),
        stdout: parse_std_message(outs),
        stderr: parse_std_message(errs),
    };
    let json_string = serde_json::to_string_pretty(&data);
    println!("{}", json_string.unwrap())
}

fn parse_std_message(message: String) -> Vec<String> {
    return message.lines().map(|s| s.to_string()).collect();
}
