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


4. assignment  day08

expression     → assignment ;
<!-- 循环实现 assignment -->
<!-- assignment     → IDENTIFIER ( "=" equality )+ ; -->  
<!-- 递归实现 assignment -->
assignment     → IDENTIFIER "=" assignment | equality ;


## 表达式结构定义  
二元运算结构 equality comparison term factor  {left expression, binaryOp, right expression}
一元运算机构 unary {unaryOp, right expression}
分组结构 group { expression }
值结构 literal {value}
变量结构 variable {name}
赋值结构 assign {name, expression}

operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" | "!" ;
binaryOp       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
unaryOp        → "-" | "!" ;
group          → "(" expression ")" ;
value          → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;
variable       → IDENTIFIER ;