#![allow(unreachable_code)]
use std::io::Write;
use std::process;

use types::Tokens;
use shell;
use tools::clog;

pub fn run(sh: &shell::Shell, tokens: &Tokens) -> i32 {
    for (_i, job) in sh.jobs.iter() {
        if !job.cmd.starts_with("nohup ") {
            println_stderr!("There are background jobs. Use command `jobs` to see details");
            return 0;
        }
    }

    if tokens.len() > 2 {
        println_stderr!("cicada: exit: too many arguments");
        return 1;
    }

    let mut code = 0;
    if tokens.len() == 2 {
        let _code = &tokens[1].1;
        match _code.parse::<i32>() {
            Ok(x) => {
                code = x;
            }
            Err(_) => {
                println_stderr!("cicada: exit: {}: numeric argument required", _code);
                code = 255;
            }
        }
    }
    process::exit(code);
    0
}
