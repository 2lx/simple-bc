#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod syntax;
mod lexer;
mod terms;

use lalrpop_util::ParseError;

pub fn parse(
    src: &str,
) -> Result<terms::Block, ParseError<usize, lexer::Token, lexer::LexicalError>> {
    let lexer = lexer::Lexer::new(src);
    syntax::SourceParser::new().parse(src, lexer)
}

#[test]
fn test_expressions_ok() {
    let result = parse("3 + 22 * 11 + 65");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{:?}", result.unwrap()),
        "[((3 + (22 * 11)) + 65)]"
    );

    let result = parse("(3 + 22) * 11 + 65");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{:?}", result.unwrap()),
        "[(((3 + 22) * 11) + 65)]"
    );

    let result = parse("3 + 22 * (11 + 65)");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{:?}", result.unwrap()),
        "[(3 + (22 * (11 + 65)))]"
    );

    let result = parse("(3 + 22) * (11 + 65)");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{:?}", result.unwrap()),
        "[((3 + 22) * (11 + 65))]"
    );

    let result = parse("-(3 + - 22) * (-11 + 65)");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{:?}", result.unwrap()),
        "[(-(3 + -22) * (-11 + 65))]"
    );

    let result = parse("-3+-22*-11**3*1+ 65");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{:?}", result.unwrap()),
        "[((-3 + ((-22 * (-11 ** 3)) * 1)) + 65)]"
    );

    let result = parse("-3+-22--11**3*1+ 65");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{:?}", result.unwrap()),
        "[(((-3 + -22) - ((-11 ** 3) * 1)) + 65)]"
    );
}

#[test]
fn test_errors() {
    let result = parse("3 + 22 * ? + 65");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::User { error } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1---2");
    assert!(result.is_ok());

    let result = parse("1++2");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken { token: (l, token, r), expected } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1 2");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken { token: (l, token, r), expected } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1+");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedEOF { location, expected } => (),
        _ => assert!(false, "wrong error type"),
    };
}
