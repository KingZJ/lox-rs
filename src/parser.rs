use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token::Object;
use crate::token_type::TokenType;
use crate::{error::LoxError, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    fn peek(&self) -> Option<Token> {
        match self.tokens.get(self.current) {
            None => None,
            Some(token) => Some(token.clone()),
        }
    }

    fn previous(&self) -> Option<Token> {
        match self.tokens.get(self.current - 1) {
            None => None,
            Some(token) => Some(token.clone()),
        }
    }

    fn is_match(&mut self, types: &Vec<TokenType>) -> bool {
        for tk_type in types {
            if self.is_expect(*tk_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn is_expect(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().unwrap().tk_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().unwrap().tk_type == TokenType::Eof
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    // expression     → equality ;
    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.comparison()?;
        let equality_operators = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while self.is_match(&equality_operators) {
            let operator = self.previous().unwrap();
            let right = self.comparison()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.term()?;
        let comparison_operators = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.is_match(&comparison_operators) {
            let operator = self.previous().unwrap();
            let right = self.term()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.factor()?;
        let term_operators = vec![TokenType::Plus, TokenType::Minus];
        while self.is_match(&term_operators) {
            let operator = self.previous().unwrap();
            let right = self.factor()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.unary()?;
        let factor_operators = vec![TokenType::Star, TokenType::Slash];
        while self.is_match(&factor_operators) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // unary          → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, LoxError> {
        let unary_operators = vec![TokenType::Bang, TokenType::Minus];
        if self.is_match(&unary_operators) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator: operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&vec![TokenType::Number, TokenType::String]) {
            let value = self.peek().unwrap().literal.unwrap();
            return Ok(Expr::Literal(LiteralExpr { value }));
        } else if self.is_match(&vec![TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::False,
            }));
        } else if self.is_match(&vec![TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::True,
            }));
        } else if self.is_match(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr { value: Object::Nil }));
        } else if self.is_match(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "expect `)` after expression")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        } else {
            let line = self.peek().unwrap().line;
            let message = "failed primary parse".to_string();
            Err(LoxError::error(line, message))
        }
    }

    fn consume(&mut self, tk_type: TokenType, message: &str) -> Result<(), LoxError> {
        if self.is_expect(tk_type) {
            self.advance();
            return Ok(());
        }
        let token = self.peek().unwrap();
        let line = token.line;
        let mut message = message.to_string();
        if token.tk_type == TokenType::Eof {
            message = format!("at end {}", message);
        } else {
            message = format!("at `{}` {}", token.lexeme, message);
        }

        Err(LoxError::error(line, message))
    }

    pub fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().unwrap().tk_type == TokenType::SemiColon {
                return;
            }

            match self.peek().unwrap().tk_type {
                TokenType::Class
                | TokenType::Func
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}
