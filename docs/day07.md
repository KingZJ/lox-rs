1. statement 翻译语句描述到对应的代码语言结构 stmt.rs

program        → statement* EOF ;
statement      → exprStmt | printStmt ;
exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

1+2;
print "one";
var a = "one";


2. 解释执行语句 stmt_interpreter.rs


3. 变量声明 表述
program        → declaration* EOF ;
declaration    → varDecl | statement ;
statement      → exprStmt | printStmt ;
exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;