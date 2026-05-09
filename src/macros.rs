use crate::ast::{Symbol, ASTNode};
use crate::racket_error::Error;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct MacroDeclarations {
    pub declr: Vec<Macro>
}

impl MacroDeclarations {
    pub fn add(&mut self, item: Macro) {
        self.declr.push(item);
    }
}

#[derive(Clone, Debug)]
pub struct Macro {
    id: Symbol,
    arg_names: Vec<Symbol>,
    pattern: ASTNode,
    template: ASTNode
}

impl Macro {
    pub fn expand(&self, arguments: Vec<ASTNode>) -> ASTNode {
        assert!(arguments.len() == self.arg_names.len())
        todo!();
    }
    pub fn matches(&self, node: ASTNode) -> bool {
        todo!();
    }
    pub fn new(pat: ASTNode, temp: ASTNode, id: Symbol, arg_sym: Vec<Symbol>) -> Macro {
        return Macro { id, arg_names: arg_sym, pattern: pat, template: temp }
    }
}

pub fn simple_macro(declr: &mut MacroDeclarations, operands: Vec<ASTNode>) -> Macro {
    // Called in form (define-syntax-rule (PI) 3.14)
    // or form (define-syntax-rule (or a b) (if a #true b))
    assert!(operands.len() == 2, "define-syntax-rule requires exactly 2 operands");
    let pattern = operands[0].clone();
    let template = operands[1].clone();
    let mac = Macro::new(pattern, template, vec![]);
    declr.add(mac.clone());
    mac
}

pub fn define_macro(declr: &mut MacroDeclarations, operands: Vec<ASTNode>) -> Macro {
    todo!();
}

pub fn get_global_macros() -> Vec<Macro> {
    let list = vec![];
    //list.push();
    list
}

