use phf::phf_map;
use std::fmt;
use std::str::CharIndices;

#[derive(Clone, Copy, Debug)]
pub enum Token<'input> {
    Variable(&'input str),
    Number(&'input str),
    Pi,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPow,
    OpenRoundBracket,
    CloseRoundBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    Semicolon,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Variable(s) => write!(f, "\"{}\"", s),
            Token::Number(n) => write!(f, "\"{}\"", n),
            Token::Pi => write!(f, "PI"),
            Token::OpAdd => write!(f, "+"),
            Token::OpSub => write!(f, "-"),
            Token::OpMul => write!(f, "*"),
            Token::OpDiv => write!(f, "/"),
            Token::OpPow => write!(f, "**"),
            Token::OpenRoundBracket => write!(f, "("),
            Token::CloseRoundBracket => write!(f, ")"),
            Token::OpenSquareBracket => write!(f, "["),
            Token::CloseSquareBracket => write!(f, "]"),
            Token::Semicolon => write!(f, ";"),
        }
    }
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "PI" => Token::Pi,
};

#[derive(Debug)]
pub enum LexicalError {
    UnrecognizedSymbol(usize, char),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::UnrecognizedSymbol(i, ch) => {
                write!(f, "lexical error: unrecognized symbol '{}' at {}", ch, i)
            },
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

    fn get_number(&mut self, start: usize) -> usize {
        let mut end = start;
        while let Some((i, ch)) = self.chars.peek() {
            if !ch.is_ascii_digit() {
                break;
            }
            end = *i;
            self.chars.next();
        }

        end + 1
    }

    fn get_variable(&mut self, start: usize) -> usize {
        let mut end = start;
        while let Some((i, ch)) = self.chars.peek() {
            if !ch.is_ascii_alphabetic() && !ch.is_ascii_digit() && *ch != '_' {
                break;
            }
            end = *i;
            self.chars.next();
        }

        end + 1
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                None => return None, // end of file

                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\r')) | Some((_, '\t')) => continue,
                Some((i, '+')) => return Some(Ok((i, Token::OpAdd, i + 1))),
                Some((i, '-')) => return Some(Ok((i, Token::OpSub, i + 1))),
                Some((i, '/')) => return Some(Ok((i, Token::OpDiv, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Token::OpenRoundBracket, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Token::CloseRoundBracket, i + 1))),
                Some((i, '[')) => return Some(Ok((i, Token::OpenSquareBracket, i + 1))),
                Some((i, ']')) => return Some(Ok((i, Token::CloseSquareBracket, i + 1))),
                Some((i, ';')) => return Some(Ok((i, Token::Semicolon, i + 1))),

                Some((i, '*')) => match self.chars.peek() {
                    Some((_, '*')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpPow, i + 2)));
                    }
                    _ => return Some(Ok((i, Token::OpMul, i + 1))),
                },

                Some((i, ch)) if ch.is_ascii_digit() => {
                    let end = self.get_number(i);
                    return Some(Ok((i, Token::Number(&self.input[i..end]), end)));
                }

                Some((i, ch)) if ch.is_ascii_alphabetic() => {
                    let end = self.get_variable(i);
                    let variable = &self.input[i..end];

                    match KEYWORDS.get(variable) {
                        Some(w) => return Some(Ok((i, *w, end))),
                        _ => return Some(Ok((i, Token::Variable(&self.input[i..end]), end))),
                    };
                }

                Some((i, ch)) => return Some(Err(LexicalError::UnrecognizedSymbol(i, ch))),
            }
        }
    }
}
