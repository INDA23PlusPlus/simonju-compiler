# Program
```
<program>   ::= <block>
<block>     ::= <statement> <ws> <block> | <ws> <statement> <ws> | E
<statement> ::= <let> | <set> | <rep> | <print>
```

# Constructs
```
<let>   ::= "let " <ws> <variable> <ws> " be " <ws> <expr> <ws> ";"
<set>   ::= "set " <ws> <variable> <ws> " to " <ws> <expr> <ws> ";"

<rep>   ::= "rep " <ws> <expr> <ws> "{" <ws> <block> <ws> "}"

<print> ::= "print " <ws> <expr> <ws> ";"
```

# Expressions
```
<expr> ::= 
    <expr> <ws> "&" <ws> <sent> |
    <expr> <ws> "|" <ws> <sent> |
                         <sent>

<sent> ::= 
    <sent> <ws> "=" <ws> <comp> |
    <sent> <ws> ">" <ws> <comp> |
    <sent> <ws> "<" <ws> <comp> |
                         <comp>

<comp> ::= 
    <comp> <ws> "+" <ws> <term> |
    <comp> <ws> "-" <ws> <term> |
                         <term>

<term> ::=
    <term> <ws> "*" <ws> <fact> |
    <term> <ws> "/" <ws> <fact> |
                         <fact>

<fact> ::=
    "?" <ws> <prim> |
    "!" <ws> <prim> |
    "+" <ws> <prim> |
    "-" <ws> <prim> |
             <prim>

<prim> ::= "(" <ws> <expr> <ws> ")" | <constant> | <variable>
```

# Building Blocks
```
<constant> ::= [0-9]+
<variable> ::= ([a-z] | "_")+
<ws>       ::= (" " | "\t" | "\n" | "\r")+ | E
```