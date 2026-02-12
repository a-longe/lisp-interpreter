use std::collections::HashMap;
use rust_decimal::Decimal;

use crate::funcs::symbol_to_function;
use crate::parse::*;

#[derive(Clone, Debug)]
pub struct ASTNode {
    pub pos: (i32, i32),
    pub expr: Box<Expression>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
    ProcedureCall(ProcedureCall),
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
    Boolean(Boolean),
    Symbol(Symbol),
    // Procedure(Procedure) // ik this should probably be a seperate value from sybol but will
    // figure out later
    // more to add
}


#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    pub value: Decimal
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Boolean {
    pub value: bool
}

#[derive(Clone, Debug)]
pub struct ProcedureCall {
    pub operator: ASTNode,
    pub operands: Vec<ASTNode>
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
    pub fn execute(&self, declarations: &Declarations) -> Value {
        match *self.expr {
            Expression::Literal(_)  => self.exec_literal(declarations),
            Expression::ProcedureCall(_) => self.exec_proc(declarations),
            _ => panic!("unsupported expression type")
        }
    }

    pub fn exec_literal(&self, declarations: &Declarations) -> Value {
        match (*self.expr).clone() {
            Expression::Literal(l) => {
                match l {
                    Literal::Number(v) => {Value::Number(v)},
                    Literal::Boolean(v) => {Value::Boolean(v)},
                    Literal::Symbol(v) => {
                        if declarations.declr.contains_key(&v) {
                            return declarations.declr[&v].clone();
                        }
                        else {
                            return Value::Symbol(v);
                        }
                    }
                    _ => panic!("unsuported literal type")
                }
            },
            _ => panic!("Pattern matching error in execute method on ASTNode")
        }
    }

    pub fn exec_proc(&self, declarations: &Declarations) -> Value {
        match *self.expr.clone() {
            Expression::ProcedureCall(p) => {
                let operator_symbol = p.operator.execute(declarations);
                match operator_symbol {
                    Value::Symbol(s) => {
                        return symbol_to_function(s.value)(declarations, p.operands).unwrap();
                    }
                    _ => panic!("operator expression does not return a symbol")
                }
            },
            _ => panic!("Pattern matching error in execute method on ASTNode")
        }
    }

}

impl Declarations {
    pub fn new() -> Declarations { Declarations {declr: HashMap::new()} }
    pub fn add(&mut self, k:Symbol, v:Value) { self.declr.insert(k, v); }
}

impl Expression {
    pub fn try_into_proc(&self) -> Option<ProcedureCall> {
        match self {
            Expression::ProcedureCall(p) => Some(p.clone()),
            _ => None
        }
    }
    pub fn try_into_lit(&self) -> Option<Literal> {
        match self {
            Expression::Literal(l) => Some(l.clone()),
            _ => None
        }
    }
}

pub fn create_ast(tokens: Vec<String>) -> ASTNode {
    ASTNode {pos: (0,0),
        expr: Box::new(create_ast_node_recursive(tokens))
    }
}

fn create_ast_node_recursive(tokens: Vec<String>) -> Expression {
    let expr: Expression;
    if tokens[0] != "(" {
        // must be literal

        if tokens[0] == "#true" || tokens[0] == "#false" { // boolean literal
            let val: bool;
            if tokens[0] == "#true" { val = true; }
            else { val = false; }
            expr = Expression::Literal(Literal::Boolean(
                Boolean { value: val }));
        }

        else if tokens[0].parse::<f64>().is_ok() { // number literal
            expr = Expression::Literal(Literal::Number(
                Number { value: Decimal::from_str_exact(&tokens[0]).expect("cannot be cast to decimal")}));
        }

        else { // symbol
            expr = Expression::Literal(Literal::Symbol(
                Symbol { value: tokens[0].clone() }))
        }

    }
    else {
        // must be proc call
        let arguments = split_tokens_into_args(tokens);
        let operator: ASTNode = ASTNode {pos: (0,0),
            expr: Box::new(create_ast_node_recursive(arguments[0].clone()))};
        let mut operands:Vec<ASTNode> = Vec::new();
        for i in 1..arguments.len() {
            operands.push( ASTNode { pos: (0,0),
                expr: Box::new(create_ast_node_recursive(arguments[i].clone()))});
        }
        expr = Expression::ProcedureCall(ProcedureCall {
            operator,
            operands
        })
    }
    return expr;
}

