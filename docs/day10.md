## for statement

statement      → exprStmt | forStmt | ifStmt | printStmt | whileStmt | block ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;

将 for 转换成 while
<!-- ( varDecl | exprStmt | ";" ) 
expression? ";"  expression 不存在 则代表条件为true
expression? -->

( varDecl | exprStmt | ";" ) 
while expression?
statement 
expression?

将 `statement` stmt1和 第二个 `expression` expr2 整体作为 block 处理  => block statement   [stmt1, exprStmt]
将 第一个 `expression` expr1 和 block 作为 while 处理  => while statement    whileStmt
将 初始化或者变量声明 语句stmt2 和 while  再次整体作为 block 处理 => block statement  [stmt2, whileStmt]
最终结构如下 [stmt2, whileStmt(expression, [stmt1, exprStmt])]
blockStmt {
    stmt: [
        stmt2,
        whileStmt {
            condition: expr1,
            stmt: blockStmt {
                stmt: [
                    stmt1,
                    exprStmt {
                        expression: expr2
                    }
                ]
            }
        }
    ]
}


### break statement


### call 函数调用
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" )* ;
arguments      → expression ( "," expression )* ;

#### 本地函数实现
native clock

### 函数声明
declaration    → funDecl | varDecl | statement ;
funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
