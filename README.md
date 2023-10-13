# simonju-compiler

## The Language

## Basic program (Fibonacci numbers)
```
let n be 0;
let m be 1;

rep 5 {
    m be m + n;
    n be m - n;
}

out n;
```

## Operator precedence (left to right)
1. +a, -a, ?a, !a (unary)
2. *, /
3. +, -
4. =, <, >
5. &, |

## Every expression becomes an integer!
Comparisons return either 1 (true) or 0 (false),
meaning we can do arithmetic on logical statements.
Example:
```
let x be 1;
let y be 2;
let z be (x = 2) + y

let wtf be x<9>6=y+z*y-(+6-!x?5)=-(!x);
``` 

## Everything is a loop!
The "rep" keyword is used to repeatedly execute a block some number of times.
Because comparisons return either 1 or 0 we can use loops to simulate if-statements;
1 (true) means execute once, while 0 (false) means do not execute at all. The value is checked once and only once. This means that if a value in the condition changes during the execution of the block, 
it will not have any effect on the number of iterations. Negative iteration values are treated like positive iteration values.
Example:
```
let x be 1;

rep x = 1 {
    out x;
}

rep -(x < 2) {
    out x;
}
``` 

## The ? operator
The ? (is) operator converts positive numbers to 1 and negative numbers -1.
Example:
```
let x be (7*8);
rep ?x {
    print !x;
}
```

## Comments
Single line comments are supprted with '#'.