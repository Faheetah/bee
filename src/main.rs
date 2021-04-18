use std::env;
use std::ffi::OsStr;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

use serde::{Deserialize, Serialize};

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

    if command == OsStr::new("bee") {
        let (arg_0, cmd_args) = args.split_at(2);
        run_command(&arg_0[1], cmd_args)
    } else {
        let (arg_0, cmd_args) = args.split_at(1);
        let command = Path::new(&arg_0[0]).file_name().unwrap();
        let s = command.to_str().unwrap().to_owned();
        run_command(&s, cmd_args)
    }
}

fn run_command(cmd: &String, args: &[String]) {
    let mut process = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
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
