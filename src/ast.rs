#[derive(Clone, Debug)]
pub struct Expr {
    Number(usize),
    Label(String),
    Opcode(String, Vec<Arg>)
}

#[derive(Clone, Debug)]
pub struct Arg {

}

#[derive(Clone, Debug)]
pub enum OpName {

}