use super::Parser;

use crate::error::LoxResult;
use crate::expr::*;
use crate::stmt::*;
use crate::token::Object;
use crate::token_type::TokenType;

impl Parser {
    //  program        → declaration* EOF ;
    pub fn program(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    //  declaration    → varDecl | statement ;
    fn declaration(&mut self) -> Result<Stmt, LoxResult> {
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
    fn var_declaration(&mut self) -> Result<Stmt, LoxResult> {
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
    // statement      → exprStmt | printStmt | block ;
    // statement      → exprStmt | ifStmt | printStmt | block ;
    // statement      → exprStmt | ifStmt | printStmt | whileStmt | block ;
    // statement      → exprStmt | forStmt | ifStmt | printStmt | whileStmt | block ;
    fn statement(&mut self) -> Result<Stmt, LoxResult> {
        if self.is_match(&vec![TokenType::For]) {
            self.for_statement()
        } else if self.is_match(&vec![TokenType::While]) {
            self.while_statement()
        } else if self.is_match(&vec![TokenType::If]) {
            self.if_statement()
        } else if self.is_match(&vec![TokenType::LeftBrace]) {
            Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }))
        } else if self.is_match(&vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    // forStmt        → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement
    fn for_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "expect `(` after `for`")?;

        let initializer = if self.is_match(&vec![TokenType::SemiColon]) {
            None
        } else if self.is_match(&vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.is_expect(TokenType::SemiColon) {
            Some(Expr::Literal(LiteralExpr {
                value: Object::True,
            }))
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::SemiColon, "expect `;` in `for` middle")?;

        let increment = if self.is_expect(TokenType::SemiColon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen, "expect `)` after `for` end")?;

        let mut body = self.statement()?;
        if let Some(expression) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![body, Stmt::Expression(ExpressionStmt { expression })],
            })
        }
        body = Stmt::While(WhileStmt {
            condition: condition.unwrap(),
            body: Box::new(body),
        });
        if let Some(init) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: vec![init, body],
            });
        }

        Ok(body)
    }

    // whileStmt      → "while" "(" expression ")" statement ;
    fn while_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "expect `(` after while")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "expect `)` after condition")?;

        let body = self.statement()?;

        Ok(Stmt::While(WhileStmt {
            condition,
            body: Box::new(body),
        }))
    }

    // ifStmt         → "if" "(" expression ")" statement ( "else" statement )?
    fn if_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "parser error expect `(` after if")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "parser error expect `)`")?;
        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.is_match(&vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    // block          → "{" declaration* "}" ;
    fn block(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_expect(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "expected `}`")?;

        Ok(statements)
    }

    // printStmt      → "print" expression ";"
    fn print_statement(&mut self) -> Result<Stmt, LoxResult> {
        let expression = self.expression()?;
        self.consume(TokenType::SemiColon, "expect `;` after expression")?;
        Ok(Stmt::Print(PrintStmt { expression }))
    }

    // exprStmt       → expression ";"
    fn expression_statement(&mut self) -> Result<Stmt, LoxResult> {
        let expression = self.expression()?;
        self.consume(TokenType::SemiColon, "expect `;` after expression")?;
        Ok(Stmt::Expression(ExpressionStmt { expression }))
    }
}
