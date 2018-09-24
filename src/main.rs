use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub math);

fn main() {
    let math = math::ExprParser::new();
    println!("{:?}", math.parse("-22 + 3 * (4^-2 + 1)"));
    println!("{:?}", math.parse("$20 - %0110"));
}
