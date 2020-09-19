mod parser;

fn main() {
    let src = "3 + 22 * 11 + 65";
    let expr = parser::parse(src);

    println!("{0:?}", expr);
}
