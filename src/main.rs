mod constants;
mod lexer;
mod parser;

use lexer::Token;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::panic;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::str;

fn execute_and_fetch(cmd: &str) -> String {
    let output = Command::new(cmd).output().unwrap();
    let result = String::from_utf8(output.stdout).unwrap().trim().to_string();
    return result;
}

fn handler(user: &String, host: &String) {
    print!("{user}@{host}$>");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let toks = lexer::lex_cmds(&input);

    let cmds = parser::parse_cmds(toks);
    if cmds.len() == 0 {
        print!(" unknown command: `{}`", input);
        return;
    }
    let mut prev_cmd_opt: Option<Child> = Option::None;
    for i in 0..cmds.len() {
        let cmd = &cmds[i];
        let mut new_cmd: Command = Command::new(&cmd.cmd);
        new_cmd.args(&cmd.args);

        match cmd.cmd.as_str() {
            constants::CMD_CD => {
                // todo: add more relative cmds here
                let mut tgt_path: PathBuf = "~".into();
                if !cmd.args.is_empty() {
                    let curr_path = std::env::current_dir().unwrap();
                    let joined_path = curr_path.join(cmd.args[0].as_str());
                    tgt_path = fs::canonicalize(joined_path).unwrap();
                };
                std::env::set_current_dir(tgt_path).unwrap();
            }
            _ => {
                // io-based cmds
                match &cmd.input {
                    Some(fname) => {
                        let in_file = std::fs::File::open(fname).unwrap();
                        new_cmd.stdin(Stdio::from(in_file));
                    }
                    None => match prev_cmd_opt {
                        Some(mut prev_cmd) => {
                            let child_stdout = prev_cmd.stdout.take().unwrap();
                            new_cmd.stdin(child_stdout);
                        }
                        None => {
                            new_cmd.stdin(Stdio::inherit());
                        }
                    },
                };

                match &cmd.output {
                    Some(out_cfg) => {
                        let out_file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(out_cfg.append)
                            .truncate(!out_cfg.append)
                            .open(&out_cfg.output)
                            .unwrap();
                        new_cmd.stdout(Stdio::from(out_file));
                    }
                    None => {
                        if i < cmds.len() - 1 {
                            new_cmd.stdout(Stdio::piped());
                        } else {
                            new_cmd.stdout(Stdio::inherit());
                        }
                    }
                };
                prev_cmd_opt = Some(new_cmd.spawn().unwrap());
            }
        }
    }

    if prev_cmd_opt.is_some() {
        let _ = prev_cmd_opt.unwrap().wait(); // hack to wait on last process
    };
}

fn main() {
    let user = execute_and_fetch("whoami");
    let host = execute_and_fetch("hostname");
    loop {
        let res = panic::catch_unwind(|| handler(&user, &host));
        match res {
            Ok(_) => {}
            Err(err) => println!("{:?}", err),
        }
    }
}
