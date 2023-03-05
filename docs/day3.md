定义语言描述形式
expression     → literal | unary | binary | grouping ;
literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;



如：
1 - (2 * 3) < 4 == false
3 * 2 - 1

只需要保证能生成对应的描述形式，不关注实际表述的意义，如 执行优先级

用代码定义 对应的数据结构 expr.rs
