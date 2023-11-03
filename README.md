# simonju-compiler
A compiler written in Rust for a custom programming language. The language supports comments, variable declaration/assignment, integer arithmetic, integer comparisons, integer logic, conditional statements, repeat loops, and printing to the console. Currently, variables must be global.

## How To Use
```
cargo run -- input_path output_path rs
cargo run -- output_path
```

## Language Grammar
### Program
```
<program>   ::= <block>
<block>     ::= <statement> <ws> <block> | <ws> <statement> <ws> | E
<statement> ::= <let> | <set> | <rep> | <print>
```

### Constructs
```
<let>   ::= "let " <ws> <variable> <ws> " be " <ws> <expr> <ws> ";"
<set>   ::= "set " <ws> <variable> <ws> " to " <ws> <expr> <ws> ";"

<rep>   ::= "rep " <ws> <expr> <ws> "{" <ws> <block> <ws> "}"

<print> ::= "print " <ws> <expr> <ws> ";"
```

### Expressions
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

### Building Blocks
```
<constant> ::= [0-9]+
<variable> ::= ([a-z] | "_")+
<ws>       ::= (" " | "\t" | "\n" | "\r")+ | E
```

## Basic Program
```
# Fibonacci numbers #

# Declares variables
let n be 0;
let m be 1;

# Sets n to the seventh fibonacci number
rep 7 {
    set m to m + n;
    set n to m - n;
}

# Prints the value of n to the console
print n;
```

## Operator precedence (left to right)
1. ``+a``, ``-a``, ``?a``, ``!a`` (unary)
2. ``*``, ``/``
3. ``+``, ``-``
4. ``=``, ``<``, ``>``
5. ``&``, ``|``

## Every expression becomes an integer!
Comparisons return either 1 (true) or 0 (false),
meaning we can do arithmetic with logical expressions.
Example:
```
let x be 1;
let y be 2;
let z be (x = 2) + y

let wtf be x<9>6=y+z*y-(+6-!x?5)=-(!x);
``` 

## If-statements are fancy loops!
The ``rep`` keyword is used to repeatedly execute a block some number of times.
Because comparisons return either 1 or 0 we can use loops to simulate if-statements;
1 (true) means execute once, while 0 (false) means do not execute at all. The value is checked once and only once. This means that if a value in the condition changes during the execution of the block, 
it will not have any effect on the number of iterations. Negative repeat values are treated as positive.
Example:
```
# If x Equals 1 Then Print x
rep x = 1 {
    print x;
}

# If x Less Than 2 Then Print x
rep -(x < 2) {
    print x;
}
``` 

## Comments
Comments are single line and begin with ``#``.

## The ? operator
The ``?`` (is) operator converts positive numbers to 1 and negative numbers to -1.
Example:
```
let x be (7*8);

# Prints x if x is non-zero
rep ?x {
    print x;
}

# Declares the absolute value of x
let abs_x be x * ?x;
```