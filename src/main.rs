mod parser;

fn main() {
    let src = "3 + 22 * 11 + 65";
    let expr = parser::parse(src);

    if expr.is_ok() {
        println!("{}", expr.unwrap());
    } else {
        println!("{}", expr.unwrap_err());
    }
}
