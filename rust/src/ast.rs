#![allow(unused_imports, dead_code)]
use std::collections::HashMap;
use crate::{funcs, parse::{self, get_tokens}};

#[derive(Clone, Debug)]
pub struct ASTNode {
    pub pos: (i32, i32),
    pub expr: Box<Expression>,
    pub declarations: Declarations
}

// Future Challenge: implement the deinition and let expression using only
// the lambda functionality like racket does.
#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
    Definition(Definition),
    Lambda(Lambda),
    IfElse(IfElse),
    Let(Let)
}

#[derive(Clone, Debug)]
pub enum Literal {
    Number(Number),
    Symbol(Symbol),
    Boolean(Boolean)
    // more to add
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(Number),
    Boolean(Boolean)
    // more to add
}

#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    pub value: f64
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub value: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Boolean {
    pub value: bool
}

#[derive(Clone, Debug)]
pub struct ProcedureCall {
    operator: ASTNode,
    operands: Vec<ASTNode>
}

#[derive(Clone, Debug)]
pub struct Definition {
    name: Symbol,
    value: ASTNode
}

#[derive(Clone, Debug)]
pub struct Lambda {
    params: Vec<Symbol>,
    function: ASTNode,
    body: Vec<ASTNode>
}

#[derive(Clone, Debug)]
pub struct IfElse {
    condition: ASTNode,
    then_branch: ASTNode,
    else_branch: ASTNode
}

#[derive(Clone, Debug)]
pub struct Let {
    bindings: Vec<Definition>,
    body: Vec<ASTNode>
}

#[derive(Clone, Debug)]
pub struct Declarations {
    pub declr: HashMap<Symbol, Value>
}

/*
*
* IMPLEMENTATIONS BELOW
*
*/

impl ASTNode {
    pub fn execute(&self) -> Value {
        match *self.expr {
            Expression::Literal(_) => self.exec_literal(),
            _ => panic!("unsupported expression type")
        }
    }

    pub fn exec_literal(&self) -> Value {
        match (*self.expr).clone() {
            Expression::Literal(l) => {
                match l {
                    Literal::Number(v) => {Value::Number(v)},
                    Literal::Boolean(v) => {Value::Boolean(v)},
                    _ => panic!("unsuported literal type")
                }
            },
            _ => panic!("Pattern matching error")
        }
    }

    pub fn simple_node(expr: Expression) -> ASTNode {
        return ASTNode {pos: (0, 0),
                        expr: Box::new(expr),
                        declarations: Declarations {declr: HashMap::new()}}
    }
}

pub fn create_ast(tokens: Vec<String>) {}

fn create_ast_node_recursive(parent: &ASTNode, tokens: Vec<String>) {}
