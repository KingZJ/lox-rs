## for statement

statement      → exprStmt | forStmt | ifStmt | printStmt | whileStmt | block ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;