use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::panic;
use rust_decimal::Decimal;

use crate::funcs::symbol_to_function;
use crate::funcs::get_base_global_scope;
use crate::parse::*;
use crate::racket_error::Error;

#[derive(Clone, Debug)]
pub struct ASTNode {
    pub pos: (usize, usize),
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
    Proc(Procedure)

    // figure out later
    // more to add
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Number {
    pub value: Decimal
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: String
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Boolean {
    pub value: bool
}

#[derive(Clone, Debug, PartialEq)]
pub enum Procedure {
    BaseProc(BaseProcedure),
    UserProc(UserProcedure)
}

#[derive(Clone, Debug)]
pub struct BaseProcedure {
    value: fn(&Declarations, Vec<ASTNode>) -> Result<Value, Error>
}

#[derive(Clone, Debug)]
pub struct UserProcedure {
    value: ASTNode
}

#[derive(Clone, Debug)]
pub struct ProcedureCall {
    pub operator: ASTNode,
    pub operands: Vec<ASTNode>
}

#[derive(Clone, Debug)]
pub struct Declarations {
    // Note that the last element in the Vec will reference the deepest scope
    pub declr: Vec<HashMap<Symbol, Value>>
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
                        if let Some(val) = declarations.get(&v) {
                            return val
                        }
                        else {
                            _ = panic!("unknown variable: {}", v);
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
                        if let Some(val) = symbol_to_function(&s) {
                            match val {
                                Value::Proc(Procedure::BaseProc(func)) =>
                                    return (func.value)(declarations, p.operands).unwrap(),
                                Value::Proc(Procedure::UserProc(func)) =>
                                    return func.value.execute(declarations)
                                _ => panic!("not a procedure: {}", val)
                            }
                        }
                        else {
                            panic!("procedure not defined: {}", s);
                        }
                    }
                    _ => panic!("operator expression does not return a symbol")
                }
            },
            _ => panic!("Pattern matching error in execute method on ASTNode")
        }
    }

}

impl PartialEq for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<Vec<String>> for ASTNode {
    fn from(value: Vec<String>) -> Self {
        create_ast(value)
    }
}

impl From<&str> for ASTNode {
    fn from(value: &str) -> Self {
        ASTNode::from(get_tokens(value))
    }
}

impl Declarations {
    pub fn new() -> Declarations {
        Declarations {declr: vec![HashMap::new()]}
    }
    pub fn global() -> Declarations {
        Declarations { declr: vec![get_base_global_scope()] }
    }
    pub fn add(&mut self, k:Symbol, v:Value) {
        self.declr.last_mut()
            .expect("Declarations Vector is empty")
            .insert(k, v);
    }
    pub fn get(&self, s:&Symbol) -> Option<Value> {
        for scope in self.declr.iter().rev() {
            if scope.contains_key(s) {
               return Some(scope[s].clone())
            }
        }
        return None
    }
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

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl Symbol {
    pub fn new(s:&str) -> Symbol {
        Symbol { value: s.to_string() }
    }
}

impl PartialEq for BaseProcedure {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl BaseProcedure {
    pub fn new(func: fn(&Declarations, Vec<ASTNode>) -> Result<Value, Error>) -> BaseProcedure {
        BaseProcedure { value: func }
    }
}

impl PartialEq for UserProcedure {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn create_ast(tokens: Vec<String>) -> ASTNode {
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
