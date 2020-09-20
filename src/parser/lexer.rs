use std::fmt;
use std::str::CharIndices;
// use phf::phf_map;

#[derive(Clone)]
pub enum Token<'input> {
    String(&'input str),
    // Pi,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPow,
    ParenOpen,
    ParenClose,
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::String(s) => write!(f, "\"{}\"", s),
            // Token::Pi => write!(f, "PI"),
            Token::OpAdd => write!(f, "+"),
            Token::OpSub => write!(f, "-"),
            Token::OpMul => write!(f, "*"),
            Token::OpDiv => write!(f, "/"),
            Token::OpPow => write!(f, "**"),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
        }
    }
}

// static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
//     "pi" => Token::Pi,
// };

#[derive(Debug)]
pub enum LexicalError {
    Error(usize, usize),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::Error(_, _) => write!(f, "lexical error"),
        }
    }
}

pub struct Lexer<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\t')) => continue,
                Some((i, '+')) => return Some(Ok((i, Token::OpAdd, i + 1))),
                Some((i, '-')) => return Some(Ok((i, Token::OpSub, i + 1))),
                Some((i, '*')) => match self.chars.peek() {
                    Some((_, '*')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpPow, i + 2)));
                    }
                    _ => return Some(Ok((i, Token::OpMul, i + 1))),
                },
                Some((i, '/')) => return Some(Ok((i, Token::OpDiv, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Token::ParenClose, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Token::ParenOpen, i + 1))),

                None => return None, // End of file
                Some((i, _)) => loop {
                    match self.chars.peek() {
                        Some((j, ')')) | Some((j, '(')) | Some((j, '+')) | Some((j, '-'))
                        | Some((j, '*')) | Some((j, '/')) | Some((j, ' ')) => {
                            return Some(Ok((i, Token::String(&self.input[i..*j]), *j)))
                        }
                        None => {
                            return Some(Ok((i, Token::String(&self.input[i..]), self.input.len())))
                        }
                        _ => {
                            self.chars.next();
                        }
                    }
                },
            }
        }
    }
}
