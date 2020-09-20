#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod syntax;
mod lexer;
mod types;

pub fn parse(src: &str) -> types::Block {
    let lexer = lexer::Lexer::new(src);
    syntax::SourceParser::new().parse(src, lexer).unwrap()
}

#[test]
fn test_parser() {
    assert_eq!(
        &format!("{:?}", parse("3 + 22 * 11 + 65")),
        "[((3 + (22 * 11)) + 65)]"
    );
    assert_eq!(
        &format!("{:?}", parse("(3 + 22) * 11 + 65")),
        "[(((3 + 22) * 11) + 65)]"
    );
    assert_eq!(
        &format!("{:?}", parse("3 + 22 * (11 + 65)")),
        "[(3 + (22 * (11 + 65)))]"
    );
    assert_eq!(
        &format!("{:?}", parse("(3 + 22) * (11 + 65)")),
        "[((3 + 22) * (11 + 65))]"
    );
    assert_eq!(
        &format!("{:?}", parse("-(3 + - 22) * (-11 + 65)")),
        "[(-(3 + -22) * (-11 + 65))]"
    );
    assert_eq!(
        &format!("{:?}", parse("-3+-22*-11**3*1+ 65")),
        "[((-3 + ((-22 * (-11 ** 3)) * 1)) + 65)]"
    );
    assert_eq!(
        &format!("{:?}", parse("-3+-22--11**3*1+ 65")),
        "[(((-3 + -22) - ((-11 ** 3) * 1)) + 65)]"
    );
}
