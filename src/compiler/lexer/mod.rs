use token::Token;

pub mod token;

pub struct Lexer {
    content: String,
    index: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self { content, index: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current().is_some() {
            tokens.push(self.tokenize_next());
        }

        tokens
    }

    fn tokenize_next(&mut self) -> Token {
        match self.skip_whitespace() {
            Some(b) if b.is_ascii_alphabetic() || b == b'_' => self.tokenize_identifier(),
            Some(b) if b.is_ascii_digit() || b == b'.' => self.tokenize_number(),
            Some(b'+') => self.advance_with_token(Token::Plus(self.index)),
            Some(b'-') => self.advance_with_token(Token::Minus(self.index)),
            Some(b'/') => self.advance_with_token(Token::Slash(self.index)),
            Some(b'*') => self.tokenize_asterisk(),
            Some(b'%') => self.advance_with_token(Token::Modulo(self.index)),
            Some(b'&') => self.tokenize_ampersand(),
            Some(b'|') => self.tokenize_pipe(),
            Some(b'^') => self.advance_with_token(Token::Caret(self.index)),
            Some(b'!') => self.tokenize_exclamation(),
            Some(b'=') => self.tokenize_assign(),
            Some(b'>') => self.tokenize_greater_than(),
            Some(b'<') => self.tokenize_less_than(),
            Some(b'(') => self.advance_with_token(Token::LeftParenthesis(self.index)),
            Some(b')') => self.advance_with_token(Token::RightParenthesis(self.index)),
            None => Token::EndOfLine(self.index),

            Some(other) => {
                println!("unhandled character: {}", other as char);
                self.advance();
                self.tokenize_next()
            }
        }
    }

    fn tokenize_identifier(&mut self) -> Token {
        let index = self.index;
        let mut value = String::new();

        while let Some(b) = self.current()
            && (b.is_ascii_alphanumeric() || b == b'_')
        {
            value.push(b as char);
            self.advance();
        }

        Token::Identifier(value, index)
    }

    fn tokenize_number(&mut self) -> Token {
        let index = self.index;
        let mut value = String::new();
        let mut is_float = false;

        while let Some(b) = self.current()
            && (b.is_ascii_digit() || b == b'.')
        {
            if b == b'.' {
                if is_float {
                    println!("syntax error");
                    return Token::Decimal(String::new(), index);
                } else {
                    is_float = true;
                }
            }

            value.push(b as char);
            self.advance();
        }

        if is_float {
            Token::Float(value, index)
        } else {
            Token::Decimal(value, index)
        }
    }

    fn tokenize_asterisk(&mut self) -> Token {
        let index = self.index;

        match self.advance() {
            Some(b'*') => self.advance_with_token(Token::AsteriskAsterisk(index)),
            _ => Token::Asterisk(index),
        }
    }

    fn tokenize_ampersand(&mut self) -> Token {
        let index = self.index;

        match self.advance() {
            Some(b'&') => self.advance_with_token(Token::AmpersandAmpersand(index)),
            _ => Token::Ampersand(index),
        }
    }

    fn tokenize_pipe(&mut self) -> Token {
        let index = self.index;

        match self.advance() {
            Some(b'|') => self.advance_with_token(Token::PipePipe(index)),
            _ => Token::Pipe(index),
        }
    }

    fn tokenize_exclamation(&mut self) -> Token {
        let index = self.index;

        match self.advance() {
            Some(b'=') => self.advance_with_token(Token::ExclamationAssign(index)),
            _ => {
                println!("(!) is unsupported");
                self.tokenize_next()
            }
        }
    }

    fn tokenize_assign(&mut self) -> Token {
        let index = self.index;

        match self.advance() {
            Some(b'=') => self.advance_with_token(Token::AssignAssign(index)),
            _ => Token::Assign(index),
        }
    }

    fn tokenize_greater_than(&mut self) -> Token {
        let index = self.index;

        match self.advance() {
            Some(b'>') => self.advance_with_token(Token::GreaterThanGreaterThan(index)),
            Some(b'=') => self.advance_with_token(Token::GreaterThanAssign(index)),
            _ => Token::GreaterThan(index),
        }
    }

    fn tokenize_less_than(&mut self) -> Token {
        let index = self.index;

        match self.advance() {
            Some(b'<') => self.advance_with_token(Token::LessThanLessThan(index)),
            Some(b'=') => self.advance_with_token(Token::LessThanAssign(index)),
            _ => Token::LessThan(index),
        }
    }

    fn advance_with_token(&mut self, token: Token) -> Token {
        self.advance();
        token
    }

    fn skip_whitespace(&mut self) -> Option<u8> {
        while self.current().is_some_and(|b| b.is_ascii_whitespace()) {
            self.advance();
        }

        self.current()
    }

    fn advance(&mut self) -> Option<u8> {
        self.index += 1;
        self.current()
    }

    fn current(&self) -> Option<u8> {
        self.content.as_bytes().iter().nth(self.index).copied()
    }
}
