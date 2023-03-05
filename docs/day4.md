语法解析器

1. 语言描述模型，枚举所有语言的表现形式
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;


确定 语言描述中的 优先级、关联性，解决语言表述中的歧义问题
如 3 * 2 - 1   先计算乘法

2. 代码转换指导
Grammar notation	Code representation
Terminal	          Code to match and consume a token
Nonterminal	          Call to that rule’s function
|	                  if or switch statement
* or +	              while or for loop
?	                  if statement