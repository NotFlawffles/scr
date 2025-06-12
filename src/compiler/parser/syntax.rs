use super::expression::Expression;

pub enum Syntax {
    Command(String),
    Expression(Expression),
    Variable(String, Expression),
    Nop,
}
