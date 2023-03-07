use crate::expr::*;
use crate::stmt::*;
use crate::token::Object;
use crate::token_type::TokenType;
use crate::{error::LoxError, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        let mut own_tokens: Vec<Token> = vec![];
        for token in tokens {
            own_tokens.push(token.clone())
        }
        Self {
            tokens: own_tokens,
            current: 0,
        }
    }

    // pub fn parse(&mut self) -> Option<Expr> {
    //     match self.expression() {
    //         Ok(expr) => Some(expr),
    //         Err(_) => None,
    //     }
    // }
    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        self.program()
    }

    //  program        → declaration* EOF ;
    fn program(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    //  declaration    → varDecl | statement ;
    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let result = if self.is_match(&vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    //  varDecl        → "var" IDENTIFIER ( "=" expression )? ";"
    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "expect variable name")?;
        let initializer = if self.is_match(&vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::SemiColon,
            "expect `;` after variable declaration",
        )?;

        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    // statement      → exprStmt | printStmt ;
    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.is_match(&vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    // printStmt      → "print" expression ";"
    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let expression = self.expression()?;
        self.consume(TokenType::SemiColon, "expect `;` after expression")?;
        Ok(Stmt::Print(PrintStmt { expression }))
    }

    // exprStmt       → expression ";"
    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expression = self.expression()?;
        self.consume(TokenType::SemiColon, "expect `;` after expression")?;
        Ok(Stmt::Expression(ExpressionStmt { expression }))
    }

    fn peek(&self) -> Option<Token> {
        // match self.tokens.get(self.current) {
        //     None => None,
        //     Some(token) => Some(token.clone()),
        // }

        // self.tokens.get(self.current).map(|token| token.clone())
        self.tokens.get(self.current).cloned()
    }

    fn previous(&self) -> Option<Token> {
        self.tokens.get(self.current - 1).cloned()
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
        match self.peek() {
            None => true,
            Some(token) => token.tk_type == TokenType::Eof,
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    // expression     → assignment ;
    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    // assignment     → IDENTIFIER "=" assignment | equality ;
    // a + b = c;
    // fn assignment(&mut self) -> Result<Expr, LoxError> {
    //     if self.is_match(&vec![TokenType::Identifier]) {
    //         let name = self.previous().unwrap();
    //         if self.is_match(&vec![TokenType::Equal]) {
    //             let value = self.assignment()?;
    //             Ok(Expr::Assign(AssignExpr { name, value: Box::new(value) }))
    //         } else {
    //             Err(LoxError::error(name.line, "parser error invalid assign".to_string()))
    //         }
            
    //     } else {
    //         self.equality()
    //     }
    // }
    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;
        if self.is_match(&vec![TokenType::Equal]) {
            let equals = self.previous().unwrap();
            if let Expr::Variable(e) = expr {
                let name = e.name;
                let value = self.assignment()?;

                return Ok(Expr::Assign(AssignExpr { name, value: Box::new(value) }));
            }

            return Err(LoxError::error(equals.line, "parser error invalid assign".to_string()));
        }

        Ok(expr)
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
                operator,
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
                operator,
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
                operator,
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
                operator,
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
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&vec![TokenType::Number, TokenType::String]) {
            let value = self.previous().unwrap().literal.unwrap();
            Ok(Expr::Literal(LiteralExpr { value }))
        } else if self.is_match(&vec![TokenType::False]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Object::False,
            }))
        } else if self.is_match(&vec![TokenType::True]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Object::True,
            }))
        } else if self.is_match(&vec![TokenType::Nil]) {
            Ok(Expr::Literal(LiteralExpr { value: Object::Nil }))
        } else if self.is_match(&vec![TokenType::Identifier]) {
            Ok(Expr::Variable(VariableExpr {
                name: self.previous().unwrap(),
            }))
        } else if self.is_match(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "expect `)` after expression")?;
            Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }))
        } else {
            let line = self.peek().unwrap().line;
            let message = "failed primary parse".to_string();
            Err(LoxError::error(line, message))
        }
    }

    fn consume(&mut self, tk_type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.is_expect(tk_type) {
            self.advance();
            return Ok(self.previous().unwrap());
        }
        let token = self.peek().unwrap();
        let line = token.line;
        let mut message = message.to_string();
        if token.tk_type == TokenType::Eof {
            message = format!("parser error at end {}", message);
        } else {
            message = format!("parser error at `{}` {}", token.lexeme, message);
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
