```
<program> ::= begin <statement_list> end
<statement-list> ::= <statement> { <statement> }
<statement> ::= let identifier = <expression> ;
<statement> ::= read ( <identifier_list> ) ;
<statement> ::= write ( <expression_list> ) ;
<identifier_list> ::= identifier { , identifier }
<expression_list> ::= <expression> { , <expression> }
```