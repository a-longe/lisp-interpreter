use crate::ast::{ASTNode, Expression, ProcedureCall, Literal, Symbol};
use core::iter::zip;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MacroDeclarations {
    pub declr: Vec<Macro>
}

impl MacroDeclarations {
    pub fn new() -> MacroDeclarations {
        MacroDeclarations { declr: vec![] }
    }
    pub fn global() -> MacroDeclarations {
        MacroDeclarations { declr: get_global_macros() }
    }
    pub fn add(&mut self, item: Macro) {
        self.declr.push(item);
    }
}

#[derive(Clone, Debug)]
pub struct Macro {
    pub arg_names: Vec<Symbol>,
    pub pattern: ASTNode,
    pub template: ASTNode
}

impl Macro {
    pub fn new(pat: ASTNode, temp: ASTNode, arg_sym: Vec<Symbol>) -> Macro {
        Macro { arg_names: arg_sym, pattern: pat, template: temp }
    }
}

pub type Bindings = HashMap<Symbol, ASTNode>;

pub fn try_match(mac: &Macro, node: &ASTNode) -> Option<Bindings> {
    let mut out: Bindings = HashMap::new();
    if match_rec(&mac.pattern, node, &mac.arg_names, &mut out) {
        Some(out)
    } else {
        None
    }
}

fn match_rec(
    pattern: &ASTNode,
    node: &ASTNode,
    arg_names: &[Symbol],
    out: &mut Bindings,
) -> bool {
    match (&*pattern.expr, &*node.expr) {
        (Expression::Literal(Literal::Symbol(sym)), _) if arg_names.contains(sym) => {
            out.insert(sym.clone(), node.clone());
            true
        }
        (Expression::Literal(pat_lit), Expression::Literal(node_lit)) => {
            pat_lit == node_lit
        }
        (Expression::ProcedureCall(pat_pc), Expression::ProcedureCall(node_pc)) => {
            if pat_pc.operands.len() != node_pc.operands.len() { return false; }
            if !match_rec(&pat_pc.operator, &node_pc.operator, arg_names, out) {
                return false;
            }
            zip(pat_pc.operands.iter(), node_pc.operands.iter())
                .all(|(p, n)| match_rec(p, n, arg_names, out))
        }
        _ => false,
    }
}

pub fn substitute(template: &ASTNode, bindings: &Bindings) -> ASTNode {
    let new_expr: Expression = match &*template.expr {
        Expression::Literal(Literal::Symbol(sym)) => {
            if let Some(bound) = bindings.get(sym) {
                return bound.clone();
            }
            Expression::Literal(Literal::Symbol(sym.clone()))
        }
        Expression::Literal(other) => Expression::Literal(other.clone()),
        Expression::ProcedureCall(pc) => {
            let new_operator = substitute(&pc.operator, bindings);
            let new_operands = pc.operands.iter()
                .map(|o| substitute(o, bindings))
                .collect();
            Expression::ProcedureCall(ProcedureCall {
                operator: new_operator,
                operands: new_operands,
            })
        }
    };
    ASTNode {
        pos: template.pos,
        expr: Box::new(new_expr),
        tokens: template.tokens.clone(),
    }
}

impl ASTNode {
    pub fn expand(&mut self, macros: &MacroDeclarations) {
        for mac in &macros.declr {
            if let Some(bindings) = try_match(mac, self) {
                *self = substitute(&mac.template, &bindings);
                return self.expand(macros);
            }
        }
        if let Expression::ProcedureCall(pc) = self.expr.as_mut() {
            pc.operator.expand(macros);
            for op in &mut pc.operands {
                op.expand(macros);
            }
        }
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

pub fn define_macro(_declr: &mut MacroDeclarations, _operands: Vec<ASTNode>) -> Macro {
    todo!();
}

pub fn get_global_macros() -> Vec<Macro> {
    vec![
        Macro {
            arg_names: vec![Symbol::new("id"), Symbol::new("val"), Symbol::new("body")],
            pattern: ASTNode::from("(let ((id val)) body)"),
            template: ASTNode::from("((lambda id body) val)"),
        },
    ]
}
