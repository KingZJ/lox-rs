## scope    environment.rs
相关作用域 以 作用域 -> 父临近作用域 链状形式存储 block scope


### block statement
program        → declaration* EOF ;

<!-- statement      → exprStmt | printStmt ; -->


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