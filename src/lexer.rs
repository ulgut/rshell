#[derive(Debug)]
pub enum Token {
    Literal(String),
    IOIn,           // <
    IOOutOverwrite, // >
    IOOutAppend,    // >>
    Term(char),     // '|', ';'
}

pub fn lex_cmds(input: &String) -> Vec<Token> {
    let mut iter = input.chars().peekable();
    let mut toks = Vec::new();
    let mut quote_stack: Option<char> = None;
    let mut temp_lit = String::new();

    while let Some(&c) = iter.peek() {
        match c {
            '\'' | '\"' => {
                if quote_stack.is_some() {
                    if quote_stack.unwrap() == c {
                        quote_stack = None;
                    } else {
                        temp_lit.push(c);
                    };
                } else {
                    quote_stack = Some(c);
                };
            }
            _ if c.is_whitespace() => {
                if quote_stack.is_some() {
                    temp_lit.push(c);
                } else if !temp_lit.is_empty() {
                    toks.push(Token::Literal(temp_lit.clone()));
                    temp_lit.clear(); // todo: check if faster/kosher to just reallocate instead
                };
            }
            '>' => {
                if quote_stack.is_some() {
                    temp_lit.push(c);
                } else {
                    iter.next();
                    if let Some(next_c) = iter.next() {
                        if next_c.is_whitespace() {
                            toks.push(Token::IOOutOverwrite);
                            continue;
                        } else if next_c == '>' {
                            toks.push(Token::IOOutAppend);
                            continue;
                        };
                    };
                    panic!("expected literal after redirect op");
                };
            }
            '<' => {
                if quote_stack.is_some() {
                    temp_lit.push(c);
                } else {
                    iter.next();
                    if let Some(next_c) = iter.next() {
                        if next_c.is_whitespace() {
                            toks.push(Token::IOIn);
                            continue;
                        };
                    };
                    panic!("expected literal after redirect op");
                };
            }
            '|' | ';' => {
                toks.push(Token::Term(c));
            }
            _ => {
                temp_lit.push(c);
            }
        }
        iter.next();
    }

    if quote_stack.is_some() {
        panic!("invalid quote string");
    };
    return toks;
}
