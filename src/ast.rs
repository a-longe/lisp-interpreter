use std::collections::HashMap;
use std::fmt::Debug;
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
    pub tokens: String
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
    value: fn(&mut Declarations, Vec<ASTNode>) -> Result<Value, Error>
}

#[derive(Clone, Debug)]
pub struct UserProcedure {
    value: ASTNode,
    var_id: Symbol
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
    pub fn execute(&self, declarations: &mut Declarations) -> Value {
        match *self.expr {
            Expression::Literal(_)  => self.exec_literal(declarations),
            Expression::ProcedureCall(_) => self.exec_proc(declarations),
            _ => panic!("unsupported expression type")
        }
    }

    fn exec_literal(&self, declarations: &Declarations) -> Value {
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

    fn exec_proc(&self, declarations: &mut Declarations) -> Value {
        match *self.expr.clone() {
            Expression::ProcedureCall(p) => {
                let operator_symbol = p.operator.execute(declarations);
                match operator_symbol {
                    Value::Proc(proc) => {
                        match proc {
                            Procedure::BaseProc(func) =>
                                return (func.value)(declarations, p.operands).unwrap(),
                            Procedure::UserProc(func) => {
                                    let var_val = &p.operands[0].execute(declarations);
                                    declarations.add(func.var_id, var_val.clone());
                                    return func.value.execute(declarations);
                                }
                        }
                    }
                    _ => panic!("operator is not a procedure")
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

impl UserProcedure {
    pub fn new(node: ASTNode, var_id: Symbol) -> UserProcedure {
        UserProcedure { value: node, var_id: var_id }
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
    pub fn add_scope(&mut self) {
        self.declr.push(HashMap::new());
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(val) => write!(f, "Value::Number {}", val.value),
            Value::Boolean(val) => write!(f, "Value::Boolean {}", val.value),
            Value::Symbol(val) => write!(f, "Value::Symbol {}", val.value),
            Value::Proc(val) => val.fmt(f)
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
    pub fn new(func: fn(&mut Declarations, Vec<ASTNode>) -> Result<Value, Error>) -> BaseProcedure {
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
        expr: Box::new(create_ast_node_recursive(tokens.clone())),
        tokens: tokens.join(" ")
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
            expr: Box::new(create_ast_node_recursive(arguments[0].clone())),
            tokens: arguments[0].join(" ") };
        let mut operands:Vec<ASTNode> = Vec::new();
        for i in 1..arguments.len() {
            operands.push( ASTNode { pos: (0,0),
                expr: Box::new(create_ast_node_recursive(arguments[i].clone())),
                tokens: arguments[i].join(" ")});
        }
        expr = Expression::ProcedureCall(ProcedureCall {
            operator,
            operands
        })
    }
    return expr;
}
