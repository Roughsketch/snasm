use lalrpop_util::lalrpop_mod;

pub mod ast;
lalrpop_mod!(pub math);

fn main() {
    let math = math::ProgramParser::new();
    println!("{:?}", math.parse("label_:\n-22 + 3 * (4^-2 + 1)"));
    println!("{:?}", math.parse("$20 - %0110"));
}
