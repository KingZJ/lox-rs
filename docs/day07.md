## statement 翻译语句描述到对应的代码语言结构 stmt.rs

## 解析 语句
### print statement | expression statement
program        → statement* EOF ;
statement      → exprStmt | printStmt ;
exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

1+2;
print "one";
var a = "one";


## 解释执行语句 stmt_interpreter.rs


### variable declaration statement
program        → declaration* EOF ;
declaration    → varDecl | statement ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;
