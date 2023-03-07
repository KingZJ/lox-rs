scope 作用域
作用域 -> 临近作用域


program        → declaration* EOF ;

statement      → exprStmt | printStmt ;


statement      → exprStmt | printStmt | block ;

block          → "{" declaration* "}" ;
declaration    → varDecl | statement ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;