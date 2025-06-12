#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String, usize),
    Decimal(String, usize),
    Float(String, usize),

    Plus(usize),
    Minus(usize),
    Slash(usize),
    Asterisk(usize),
    Modulo(usize),
    Ampersand(usize),
    Pipe(usize),
    Caret(usize),
    Assign(usize),
    GreaterThan(usize),
    LessThan(usize),

    AsteriskAsterisk(usize),
    AmpersandAmpersand(usize),
    PipePipe(usize),
    AssignAssign(usize),
    GreaterThanGreaterThan(usize),
    LessThanLessThan(usize),

    ExclamationAssign(usize),
    GreaterThanAssign(usize),
    LessThanAssign(usize),

    LeftParenthesis(usize),
    RightParenthesis(usize),

    EndOfLine(usize),
}
