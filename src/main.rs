pub mod lexer;

#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod lua;

fn main() {
    let src = "22 * 11 + 65";
    let lexer = lexer::Lexer::new(src);
    let expr = lua::ExprParser::new().parse(src, lexer).unwrap();

    println!("{0:?}", expr);
}
