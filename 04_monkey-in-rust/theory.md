# Monkey Programming

## Basic

```
source code -> tokens -> abstract syntax tree
```

**what's source code?**

```
let x = 1 + 2;
```

**what's tokens?**

```
[
    LET,
    IDENTIFIER("x"),
    EQUAL_SIGN,
    INTEGER(5),
    PLUS_SIGN,
    INTEGER(5),
    SEMICOLON
]
```

## Lexer

```
source code -> tokens
```

## Parser

```
tokens -> ast
```
