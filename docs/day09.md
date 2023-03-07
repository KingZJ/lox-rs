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



### if statement
statement      → exprStmt | ifStmt | printStmt | block ;
ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;


### or and 
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;

### while statement
statement      → exprStmt | ifStmt | printStmt | whileStmt | block ;
whileStmt      → "while" "(" expression ")" statement ;