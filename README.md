# Lisp Interpreter in Python

## Logic

Lisp expressions are broken into three segments; the parenthesese around it,
the function at the start and the following arguments, each separated by a space

ex.
'''racket
(+ 2 2)
'''

Here is that same expression broken down as the Interpreter will understand it.

Function: "+" -> add()
arguments: "2 2" -> [2, 2]

Things get slightly more confusing when you consider that multiple expressions can
be "stacked"

ex.
'''racket
(+ (+ 1 1) 2)
'''

Here, the Interpreter should read this like so;

Function: "+" -> add()
arguments: "(+ 1 1) 2" -> [(+ 1 1), 2]

Then the ***add function*** should tell the Interpreter that it expects all of the
arguments to be evaluated

When I originally started writing this code I thought that the execute function would
make sure all of the arguments were evaluated before executing the root expression but
I forgot that you can't evaluate everything in the block, otherwise if-else would simply
not work, as well as special case functions like let and lambda would not work either as
they use special syntax to define values.

Here is how the chain of logic should work:
A Lisp expression is passed to the Interpreter.
The function is determined and arguments' parsed.
These arguments are passed to the corrosponding python function that is assosiated
with the lisp one.
The python function decides what to do with these arguments, whether further parsing them
or passing them back to the Interpreter
