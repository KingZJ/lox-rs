use std::rc::Rc;

use super::Parser;

use crate::core::*;
use crate::error::LoxResult;
use crate::expr::*;
use crate::stmt::*;
use crate::token::Token;
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
    //  declaration    → funDecl | varDecl | statement ;
    fn declaration(&mut self) -> Result<Stmt, LoxResult> {
        let result = if self.is_match(&vec![TokenType::Var]) {
            self.var_declaration()
        } else if self.is_match(&vec![TokenType::Func]) {
            self.func_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    // funDecl        → "fun" function ;
    fn func_declaration(&mut self) -> Result<Stmt, LoxResult> {
        self.function("function")
    }

    // function       → IDENTIFIER "(" parameters? ")" block ;
    fn function(&mut self, kind: &str) -> Result<Stmt, LoxResult> {
        let name = self.consume(TokenType::Identifier, &format!("expect a {kind} name"))?;

        self.consume(
            TokenType::LeftParen,
            &format!("expect `(` after {kind} name"),
        )?;
        let mut params: Vec<Token> = vec![];
        if !self.is_expect(TokenType::RightParen) {
            params = self.parameters()?;
        }
        self.consume(TokenType::RightParen, "expect `)` after parameters")?;

        // block          → "{" declaration* "}" ; 逻辑与statement 中判断block处理一致，须先将 `{` 匹配处理
        self.consume(
            TokenType::LeftBrace,
            &format!("expect `{{` before {kind} body"),
        )?;
        let body = Rc::new(self.block()?);

        Ok(Stmt::Function(FunctionStmt {
            name,
            params: Rc::new(params),
            body,
        }))
    }

    // parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
    fn parameters(&mut self) -> Result<Vec<Token>, LoxResult> {
        let mut params: Vec<Token> = vec![];
        params.push(self.consume(TokenType::Identifier, "expect a param name")?);

        while self.is_match(&vec![TokenType::Comma]) {
            if params.len() >= 255 {
                return Err(LoxResult::runtime_error(
                    &self.peek().unwrap(),
                    "can't have more than 255 parameters".to_owned(),
                ));
            }
            params.push(self.consume(TokenType::Identifier, "expect a param name")?);
        }

        Ok(params)
    }

    //  varDecl        → "var" IDENTIFIER ( "=" expression )? ";"
    fn var_declaration(&mut self) -> Result<Stmt, LoxResult> {
        let name = self.consume(TokenType::Identifier, "expect variable name")?;
        let initializer = if self.is_match(&vec![TokenType::Equal]) {
            Some(Rc::new(self.expression()?))
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
    // statement      → exprStmt | forStmt | ifStmt | printStmt | whileStmt | block | breakStmt;
    // statement      → exprStmt | forStmt | ifStmt | printStmt | returnStmt | whileStmt | block ;
    fn statement(&mut self) -> Result<Stmt, LoxResult> {
        if self.is_match(&vec![TokenType::Return]) {
            self.return_statement()
        } else if self.is_match(&vec![TokenType::Break]) {
            self.break_statement()
        } else if self.is_match(&vec![TokenType::For]) {
            self.for_statement()
        } else if self.is_match(&vec![TokenType::While]) {
            self.while_statement()
        } else if self.is_match(&vec![TokenType::If]) {
            self.if_statement()
        } else if self.is_match(&vec![TokenType::LeftBrace]) {
            Ok(Stmt::Block(BlockStmt {
                statements: Rc::new(self.block()?),
            }))
        } else if self.is_match(&vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    // returnStmt     → "return" expression? ";" ;
    fn return_statement(&mut self) -> Result<Stmt, LoxResult> {
        let name = self.previous().unwrap();
        let value = if self.is_expect(TokenType::SemiColon) {
            None
        } else {
            Some(Rc::new(self.expression()?))
        };
        self.consume(TokenType::SemiColon, "expect `;` after return")?;

        Ok(Stmt::Return(ReturnStmt { name, value }))
    }

    // breakStmt      → "break" ";" ;
    fn break_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::SemiColon, "expect `;` after break statement")?;
        Ok(Stmt::Break(BreakStmt { u: 0 }))
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
                statements: Rc::new(vec![
                    Rc::new(body),
                    Rc::new(Stmt::Expression(ExpressionStmt {
                        expression: Rc::new(expression),
                    })),
                ]),
            })
        }
        body = Stmt::While(WhileStmt {
            condition: Rc::new(condition.unwrap()),
            body: Rc::new(body),
        });
        if let Some(init) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: Rc::new(vec![Rc::new(init), Rc::new(body)]),
            });
        }

        Ok(body)
    }

    // whileStmt      → "while" "(" expression ")" statement ;
    fn while_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "expect `(` after while")?;
        let condition = Rc::new(self.expression()?);
        self.consume(TokenType::RightParen, "expect `)` after condition")?;

        let body = Rc::new(self.statement()?);

        Ok(Stmt::While(WhileStmt { condition, body }))
    }

    // ifStmt         → "if" "(" expression ")" statement ( "else" statement )?
    fn if_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "expect `(` after if")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "expect `)`")?;
        let then_branch = Rc::new(self.statement()?);

        let else_branch = if self.is_match(&vec![TokenType::Else]) {
            Some(Rc::new(self.statement()?))
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
    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, LoxResult> {
        let mut statements: Vec<Rc<Stmt>> = vec![];
        while !self.is_expect(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(Rc::new(self.declaration()?));
        }

        self.consume(TokenType::RightBrace, "expected `}`")?;

        Ok(statements)
    }

    // printStmt      → "print" expression ";"
    fn print_statement(&mut self) -> Result<Stmt, LoxResult> {
        let expression = Rc::new(self.expression()?);
        self.consume(TokenType::SemiColon, "expect `;` after expression")?;
        Ok(Stmt::Print(PrintStmt { expression }))
    }

    // exprStmt       → expression ";"
    fn expression_statement(&mut self) -> Result<Stmt, LoxResult> {
        let expression = Rc::new(self.expression()?);
        self.consume(TokenType::SemiColon, "expect `;` after expression")?;
        Ok(Stmt::Expression(ExpressionStmt { expression }))
    }
}
