//! AST parsing and node counting utilities using `syn`.
//! Counts types of statements, expressions, patterns, and types in Rust code.

use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use syn::{visit::Visit, File};

/// Parse a Rust source file and return a map of AST node counts.
pub fn parse_and_count<P: AsRef<Path>>(path: P) -> Result<HashMap<String, f32>> {
    let src = fs::read_to_string(path)?;
    let file: File = syn::parse_file(&src)?;

    let mut counter = NodeCounter::default();
    counter.visit_file(&file);

    Ok(counter.counts)
}

/// Counts AST nodes.
#[derive(Default)]
pub struct NodeCounter {
    pub counts: HashMap<String, f32>,
}

impl NodeCounter {
    /// Increment count for a node type.
    fn bump(&mut self, key: &str) {
        *self.counts.entry(key.to_string()).or_insert(0.0) += 1.0;
    }
}

impl<'ast> Visit<'ast> for NodeCounter {
    fn visit_stmt(&mut self, s: &'ast syn::Stmt) {
        match s {
            syn::Stmt::Local(_) => self.bump("Stmt::Local"),
            syn::Stmt::Item(_) => self.bump("Stmt::Item"),
            syn::Stmt::Expr(..) => self.bump("Stmt::Expr"),
            syn::Stmt::Macro(_) => self.bump("Stmt::Macro"),
        }
        syn::visit::visit_stmt(self, s);
    }

    fn visit_expr(&mut self, ex: &'ast syn::Expr) {
        use syn::Expr::*;
        match ex {
            If(_) => self.bump("Expr::If"),
            ForLoop(_) => self.bump("Expr::ForLoop"),
            While(_) => self.bump("Expr::While"),
            Loop(_) => self.bump("Expr::Loop"),
            Match(_) => self.bump("Expr::Match"),
            Call(_) => self.bump("Expr::Call"),
            MethodCall(_) => self.bump("Expr::MethodCall"),
            Struct(_) => self.bump("Expr::Struct"),
            Field(_) => self.bump("Expr::Field"),
            Path(_) => self.bump("Expr::Path"),
            Reference(_) => self.bump("Expr::Reference"),
            Return(_) => self.bump("Expr::Return"),
            Macro(_) => self.bump("Expr::Macro"),
            Lit(_) => self.bump("Expr::Lit"),
            Array(_) => self.bump("Expr::Array"),
            Tuple(_) => self.bump("Expr::Tuple"),
            Try(_) => self.bump("Expr::Try"),
            Await(_) => self.bump("Expr::Await"),
            Closure(_) => self.bump("Expr::Closure"),
            Assign(_) => self.bump("Expr::Assign"),
            _ => self.bump("Expr::Other"),
        }
        syn::visit::visit_expr(self, ex);
    }

    fn visit_type(&mut self, t: &'ast syn::Type) {
        use syn::Type::*;
        match t {
            Path(_) => self.bump("Type::Path"),
            Reference(_) => self.bump("Type::Reference"),
            Array(_) => self.bump("Type::Array"),
            Slice(_) => self.bump("Type::Slice"),
            Tuple(_) => self.bump("Type::Tuple"),
            BareFn(_) => self.bump("Type::BareFn"),
            Ptr(_) => self.bump("Type::Ptr"),
            Infer(_) => self.bump("Type::Infer"),
            _ => self.bump("Type::Other"),
        }
        syn::visit::visit_type(self, t);
    }

    fn visit_pat(&mut self, p: &'ast syn::Pat) {
        use syn::Pat::*;
        match p {
            Ident(_) => self.bump("Pat::Ident"),
            Wild(_) => self.bump("Pat::Wild"),
            Struct(_) => self.bump("Pat::Struct"),
            Tuple(_) => self.bump("Pat::Tuple"),
            TupleStruct(_) => self.bump("Pat::TupleStruct"),
            Slice(_) => self.bump("Pat::Slice"),
            Path(_) => self.bump("Pat::Path"),
            Lit(_) => self.bump("Pat::Lit"),
            _ => self.bump("Pat::Other"),
        }
        syn::visit::visit_pat(self, p);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.bump("Macro");
        syn::visit::visit_macro(self, mac);
    }

    fn visit_attribute(&mut self, _i: &'ast syn::Attribute) {
        self.bump("Attribute");
        syn::visit::visit_attribute(self, _i);
    }

    fn visit_block(&mut self, _b: &'ast syn::Block) {
        self.bump("Block");
        syn::visit::visit_block(self, _b);
    }
}
