### return statement

statement      → exprStmt | forStmt | ifStmt | printStmt | returnStmt | whileStmt | block ;

returnStmt     → "return" expression? ";" ;

### closure
主要是在函数声明时会将当前的 环境信息保存，在调用时作为新环境的父级作用域

### resolver