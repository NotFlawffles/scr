use std::{collections::HashMap, fmt::Display};

use crate::compiler::lexer::token::Token;

#[derive(Clone)]
pub enum Literal {
    Name(String),
    Integer(usize),
    Float(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{name}"),
            Self::Integer(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
        }
    }
}

pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Literal(Literal, usize),
}

impl Expression {
    pub fn evaluate(&self, variables: &HashMap<String, Literal>) -> Literal {
        match self {
            Self::Binary(..) => self.evaluate_binary(variables),

            Self::Literal(literal, ..) => match literal {
                Literal::Name(name) => {
                    if let Some(value) = variables.get(name) {
                        value.clone()
                    } else {
                        println!("undefined name: {name}");
                        Literal::Integer(0)
                    }
                }

                other => other.clone(),
            },
        }
    }

    fn evaluate_binary_integers(
        &self,
        left: &Literal,
        operator: &Token,
        right: &Literal,
    ) -> Literal {
        match operator {
            Token::Plus(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left + right),
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float(*left as f64 + right)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float(left + *right as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left + right),
                _ => unreachable!(),
            },

            Token::Minus(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left - right),
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float(*left as f64 - right)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float(left - *right as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left - right),
                _ => unreachable!(),
            },

            Token::Asterisk(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left * right),
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float(*left as f64 * right)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float(left * *right as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left * right),
                _ => unreachable!(),
            },

            Token::Slash(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Float(*left as f64 / *right as f64)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float(*left as f64 / right)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float(left / *right as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left / right),
                _ => unreachable!(),
            },

            Token::Modulo(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left % right),
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float(*left as f64 % right)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float(left % *right as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left % right),
                _ => unreachable!(),
            },

            Token::Pipe(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left | right),

                _ => {
                    println!("cannot perform bitwise inclusive or (|) on non-integer literals");
                    Literal::Integer(0)
                }
            },

            Token::Ampersand(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left & right),

                _ => {
                    println!("cannot perform bitwise and (&) on non-integer literals");
                    Literal::Integer(0)
                }
            },

            Token::Caret(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left ^ right),

                _ => {
                    println!("cannot perform bitwise exclusive or (^) on non-integer literals");
                    Literal::Integer(0)
                }
            },

            Token::GreaterThan(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer((left > right) as usize)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float((*left as f64 > *right) as usize as f64)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float((*left > *right as f64) as usize as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    Literal::Float((left > right) as usize as f64)
                }
                _ => unreachable!(),
            },

            Token::LessThan(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer((left < right) as usize)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float(((*left as f64) < *right) as usize as f64)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float((*left < *right as f64) as usize as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    Literal::Float((left < right) as usize as f64)
                }
                _ => unreachable!(),
            },

            Token::AsteriskAsterisk(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer(left.pow(*right as u32))
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float((*left as f64).powf(*right))
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float(left.powf(*right as f64))
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left.powf(*right)),
                _ => unreachable!(),
            },

            Token::AmpersandAmpersand(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer(((*left > 0) && (*right > 0)) as usize)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float(((*left > 0) && (*right > 0.0)) as usize as f64)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float(((*left > 0.0) && (*right > 0)) as usize as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    Literal::Float(((*left > 0.0) && (*right > 0.0)) as usize as f64)
                }
                _ => unreachable!(),
            },

            Token::PipePipe(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer(((*left > 0) || (*right > 0)) as usize)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float(((*left > 0) || (*right > 0.0)) as usize as f64)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float(((*left > 0.0) || (*right > 0)) as usize as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    Literal::Float(((*left > 0.0) || (*right > 0.0)) as usize as f64)
                }
                _ => unreachable!(),
            },

            Token::AssignAssign(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer((left == right) as usize)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float((*left as f64 == *right) as usize as f64)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float((*left == *right as f64) as usize as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    Literal::Float((left == right) as usize as f64)
                }
                _ => unreachable!(),
            },

            Token::GreaterThanGreaterThan(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer(left >> right)
                }

                _ => {
                    println!("cannot perform bitwise shift (>>) on non-integer literals");
                    Literal::Integer(0)
                }
            },

            Token::LessThanLessThan(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer(left << right)
                }

                _ => {
                    println!("cannot perform bitwise shift (>>) on non-integer literals");
                    Literal::Integer(0)
                }
            },

            Token::ExclamationAssign(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer((left != right) as usize)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float((*left as f64 != *right) as usize as f64)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float((*left != *right as f64) as usize as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    Literal::Float((left != right) as usize as f64)
                }
                _ => unreachable!(),
            },

            Token::GreaterThanAssign(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer((left >= right) as usize)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float((*left as f64 >= *right) as usize as f64)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float((*left >= *right as f64) as usize as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    Literal::Float((left >= right) as usize as f64)
                }
                _ => unreachable!(),
            },

            Token::LessThanAssign(..) => match (left, right) {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Integer((left <= right) as usize)
                }
                (Literal::Integer(left), Literal::Float(right)) => {
                    Literal::Float((*left as f64 <= *right) as usize as f64)
                }
                (Literal::Float(left), Literal::Integer(right)) => {
                    Literal::Float((*left <= *right as f64) as usize as f64)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    Literal::Float((left <= right) as usize as f64)
                }
                _ => unreachable!(),
            },

            _ => Literal::Integer(0),
        }
    }

    fn evaluate_binary(&self, variables: &HashMap<String, Literal>) -> Literal {
        let Self::Binary(left, operator, right) = self else {
            unreachable!()
        };

        let left = left.evaluate(variables);
        let right = right.evaluate(variables);
        let result = self.evaluate_binary_integers(&left, operator, &right);

        result
    }
}
