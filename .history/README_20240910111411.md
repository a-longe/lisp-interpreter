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
These arguments are passed to the coresponding python function that is assosiated
with the lisp one.
The python function decides what to do with these arguments, whether further parsing them
or passing them back to the Interpreter


# Lisp Interpreter in Rust

## AST
the AST or abstract syntax tree is a non-primative data structure that repersents the program
in a way that separates expressions into their own node.

Each node will either be a value or a procedure with arguments, these arguments are acctually other AST
nodes and can either be another procedure with arguments or a leaf node with just a value, the
basic structure will be something like so:
ASTNode:
  func: String
  lo_args: \[ASTNode]

## Interpreter Flow
The Interpreter will function in an order like so, first it will read from a file the whole program
Next it will parse the program into an AST
Finally the program will recursively simplify the program from the bottom up until the whole program
returns a final value.

## Notes
For simple programs that use basic functions like math (+, -, /, *) this is an easy problem, the ast for an simple program
(* (+ 1 1) 2)
    (*)
    / \
  (+) (2)
  / \
 (1)(1)

The thing that makes it more complicated are the let and lambda expressions
the let block works like so (let ([var1, val1], [var2, val2] ...) (body))
These declarations only apply to the body block in the let statement and it will evaluate the same way that the body block does but with any variables held
in the let block replaced with their value.

So my plan to deal with these is during the runtime, as the program executes the execute function will take in a hashmap that maps a token to a value, every time
the execution function reaches a let block, it will read the declarations and add them as well as the previous hashmap to a new hashmap, to be passed to the execute
function on the body block

{x: 2}
(let ([y 4]) (+ x y))
{x: 2, y: 4}
(+ x y)
(+ 2 4)
(6)

So the execute function will looks something like this
ASTNode.execute(self, declarations) -> _ {
  func = funcs.lisp_to_rust_func(self)
  return func(self)
}

lisp_to_rust_func(ASTNode, declarations) -> _ {
  switch ASTNode.func {
    "+" => add
    "-" => sub
    ...
  }
}

## On Types
The Ast type structure will look something like this:

AST Node:
  enum Expression:
    Procedure:
      func_token(String),
      args: Vec<AST Node> |
    enum Value:
      Number
      String
      ...
    Token
    Assingments

  declarations

## Funcs
add and multiplication are self explanitary
but subtraction with multiple values and other functions take a bit
more thought.
sub = a1 - a2 - a3 - a4 ...
div = ((a1 / a2) / a3) ...