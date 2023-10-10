# Arithmetic:
```
<expression> ::= <expression> <addop> <term> | <addop> <term> | <term>
<term> ::= <term> <multop> <power> | <power>
<power> ::= <factor> ^ <power> | <factor> 
<factor> ::= ( <expression> ) | integer | identifier

<addop> ::= + | -
<multop> ::= * | /
```

