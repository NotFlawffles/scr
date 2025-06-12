use std::mem::discriminant;

use expression::{Expression, Literal};
use syntax::Syntax;

use super::lexer::token::Token;

pub mod expression;
pub mod syntax;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Syntax {
        match self.current() {
            Some(Token::Identifier(..)) => self.parse_name(),
            Some(Token::Decimal(..))
            | Some(Token::Float(..))
            | Some(Token::LeftParenthesis(..)) => self.parse_expression(),
            Some(Token::EndOfLine(..)) | None => Syntax::Nop,

            _ => {
                println!("expected statement");
                Syntax::Nop
            }
        }
    }

    fn parse_name(&mut self) -> Syntax {
        let Some(Token::Identifier(name, ..)) = self.current() else {
            unreachable!()
        };

        match name.as_str() {
            "exit" => self.advance_with(Syntax::Command(name.clone())),
            "clear" => self.advance_with(Syntax::Command(name.clone())),
            "help" => self.advance_with(Syntax::Command(name.clone())),
            "list" => self.advance_with(Syntax::Command(name.clone())),
            "let" => self.parse_variable(),
            _ => self.parse_expression(),
        }
    }

    fn parse_variable(&mut self) -> Syntax {
        self.advance();

        let name = if let Some(Token::Identifier(name, ..)) = self.current() {
            name.clone()
        } else {
            String::new()
        };

        self.advance();
        self.eat(Token::Assign(0));
        let value = self.parse_expression_expression();

        if let Some(value) = value {
            Syntax::Variable(name.clone(), value)
        } else {
            Syntax::Nop
        }
    }

    fn parse_expression(&mut self) -> Syntax {
        let expression = self.parse_expression_expression();

        if let Some(expression) = expression {
            Syntax::Expression(expression)
        } else {
            Syntax::Nop
        }
    }

    fn parse_expression_expression(&mut self) -> Option<Expression> {
        self.parse_logical_or_expression()
    }

    fn parse_logical_or_expression(&mut self) -> Option<Expression> {
        let left = self.parse_logical_and_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::PipePipe(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_logical_and_expression(&mut self) -> Option<Expression> {
        let left = self.parse_bitwise_inclusive_or_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::AmpersandAmpersand(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_bitwise_inclusive_or_expression(&mut self) -> Option<Expression> {
        let left = self.parse_bitwise_exclusive_or_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::Pipe(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_bitwise_exclusive_or_expression(&mut self) -> Option<Expression> {
        let left = self.parse_bitwise_and_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::Ampersand(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_bitwise_and_expression(&mut self) -> Option<Expression> {
        let left = self.parse_rational_equality_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::Ampersand(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_rational_equality_expression(&mut self) -> Option<Expression> {
        let left = self.parse_rational_difference_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::AssignAssign(..)) | Some(Token::ExclamationAssign(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_rational_difference_expression(&mut self) -> Option<Expression> {
        let left = self.parse_bitwise_shift_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::GreaterThan(..))
            | Some(Token::LessThan(..))
            | Some(Token::GreaterThanAssign(..))
            | Some(Token::LessThanAssign(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_bitwise_shift_expression(&mut self) -> Option<Expression> {
        let left = self.parse_additive_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::GreaterThanGreaterThan(..)) | Some(Token::LessThanLessThan(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_additive_expression(&mut self) -> Option<Expression> {
        let left = self.parse_multiplicative_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::Plus(..)) | Some(Token::Minus(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_multiplicative_expression(&mut self) -> Option<Expression> {
        let left = self.parse_literal_expression();

        if left.is_none() {
            return left;
        }

        let mut left = left.unwrap();

        Some(match self.current() {
            Some(Token::Asterisk(..)) | Some(Token::Slash(..)) | Some(Token::Modulo(..)) => {
                let operator = self.current().unwrap().clone();
                self.eat(operator.clone());
                let right = self.parse_expression_expression();
                left =
                    Expression::Binary(Box::new(left), operator.clone(), Box::new(right.unwrap()));
                left
            }

            _ => left,
        })
    }

    fn parse_literal_expression(&mut self) -> Option<Expression> {
        match self.current() {
            Some(Token::Identifier(name, index)) => self.advance_with(Some(Expression::Literal(
                Literal::Name(name.clone()),
                index.clone(),
            ))),

            Some(Token::Decimal(value, index)) => self.advance_with(Some(Expression::Literal(
                Literal::Integer(value.parse().unwrap()),
                index.clone(),
            ))),

            Some(Token::Float(value, index)) => self.advance_with(Some(Expression::Literal(
                Literal::Float(value.parse().unwrap()),
                index.clone(),
            ))),

            Some(Token::LeftParenthesis(..)) => {
                self.advance();
                let expression = self.parse_expression_expression();
                self.eat(Token::RightParenthesis(0));
                expression
            }

            _ => {
                println!("expected literal");
                None
            }
        }
    }

    fn advance_with<T>(&mut self, any: T) -> T {
        self.advance();
        any
    }

    fn eat(&mut self, expect: Token) -> Option<&Token> {
        if let Some(current) = self.current() {
            if discriminant(current) != discriminant(&expect) {
                println!("expected: {expect:?}, got: {current:?}");
                return None;
            } else {
                return self.advance();
            }
        }

        None
    }

    fn advance(&mut self) -> Option<&Token> {
        self.index += 1;
        self.current()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
}
