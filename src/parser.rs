use crate::Token;
pub type Args = Vec<String>;

#[derive(Debug)]
pub struct Cmd {
    pub cmd: String,
    pub args: Args,
    pub input: Option<String>, // None => stdin
    pub output: Option<IOOut>, // None => stdout
}

impl Cmd {
    fn new() -> Cmd {
        Cmd {
            cmd: String::new(),
            args: Vec::new(),
            input: None,
            output: None,
        }
    }
}

#[derive(Debug)]
pub struct IOOut {
    pub output: String,
    pub append: bool,
}

pub fn parse_cmds(input: Vec<Token>) -> Vec<Cmd> {
    let mut iter = input.iter().peekable();
    let mut cmds = Vec::new();
    let mut curr_cmd: Cmd = Cmd::new();

    while let Some(&tok) = iter.peek() {
        match tok {
            Token::Literal(lit) => {
                if curr_cmd.cmd.is_empty() {
                    curr_cmd.cmd = lit.to_owned();
                } else {
                    curr_cmd.args.push(lit.to_owned());
                };
            }
            Token::Term(lit) => {
                assert!(!curr_cmd.cmd.is_empty());
                match lit {
                    '|' | ';' => {
                        cmds.push(curr_cmd);
                        curr_cmd = Cmd::new();
                    }
                    _ => panic!("invalid termination character"),
                }
            }
            Token::IOIn => {
                assert!(!curr_cmd.cmd.is_empty());
                iter.next();
                if let Some(Token::Literal(io_src)) = iter.peek() {
                    curr_cmd.input = Some(io_src.to_owned());
                } else {
                    panic!("Expected a valid input src after '<'"); // this doesn't support recursive $(`..`) commands
                };
            }
            Token::IOOutAppend | Token::IOOutOverwrite => {
                assert!(!curr_cmd.cmd.is_empty());
                iter.next();
                if let Some(Token::Literal(io_dst)) = iter.peek() {
                    curr_cmd.output = Some(IOOut {
                        output: io_dst.to_owned(),
                        append: match tok {
                            Token::IOOutAppend => true,
                            Token::IOOutOverwrite => false,
                            _ => panic!("cannot happen"),
                        },
                    });
                } else {
                    panic!("Expected a valid output src after '>'"); // this doesn't support recursive $(`..`) commands
                }
            }
        }
        iter.next();
    }
    cmds.push(curr_cmd);
    return cmds;
}
