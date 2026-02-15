# rust-lox
Rust implementation of Lox language interpreter.  Lox is a simple language created by Robert Nystrom for his book "Crafting Interpreters" (craftinginterpreters.com)


## Language Specification

```
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
```