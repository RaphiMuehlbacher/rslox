use crate::ast::{
    AssignExpr, BinaryExpr, CallExpr, Expr, FunDeclStmt, Ident, IfStmt, LogicalExpr, Program,
    Spanned, Stmt, UnaryExpr, VarDeclStmt, WhileStmt,
};
use miette::Report;
use std::ops::Deref;

pub struct Resolver<'a> {
    program: &'a Program,
    errors: Vec<Report>,
}

impl<'a> Resolver<'a> {
    pub fn new(ast: &'a Program) -> Self {
        Self {
            program: ast,
            errors: vec![],
        }
    }

    pub fn resolve(mut self) -> Vec<Report> {
        for stmt in &self.program.statements {
            self.resolve_stmt(stmt);
        }
        self.errors
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::ExprStmt(expr_stmt) => self.resolve_expr_stmt(expr_stmt),
            Stmt::PrintStmt(print_stmt) => self.resolve_print_stmt(print_stmt),
            Stmt::VarDecl(var_decl) => self.resolve_var_decl(var_decl),
            Stmt::FunDecl(fun_decl) => self.resolve_fun_decl(fun_decl),
            Stmt::Block(block) => self.resolve_block(block),
            Stmt::If(if_stmt) => self.resolve_if_stmt(if_stmt),
            Stmt::While(while_stmt) => self.resolve_while_stmt(while_stmt),
            Stmt::Return(return_stmt) => self.resolve_return_stmt(return_stmt),
        }
    }

    fn resolve_expr_stmt(&mut self, expr_stmt: &Spanned<Expr>) {
        self.resolve_expr(&expr_stmt.node);
    }

    fn resolve_print_stmt(&mut self, print_stmt: &Spanned<Expr>) {
        self.resolve_expr(&print_stmt.node);
    }

    fn resolve_var_decl(&mut self, var_decl: &Spanned<VarDeclStmt>) {
        todo!()
    }

    fn resolve_fun_decl(&mut self, fun_decl: &Spanned<FunDeclStmt>) {
        todo!()
    }

    fn resolve_block(&mut self, block: &Spanned<Vec<Stmt>>) {
        todo!()
    }

    fn resolve_if_stmt(&mut self, if_stmt: &Spanned<IfStmt>) {
        todo!()
    }

    fn resolve_while_stmt(&mut self, while_stmt: &Spanned<WhileStmt>) {
        todo!()
    }

    fn resolve_return_stmt(&mut self, return_stmt: &Spanned<Option<Expr>>) {
        if let Some(node) = &return_stmt.node {
            self.resolve_expr(node);
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(_) => {}
            Expr::Unary(unary_expr) => self.resolve_unary_expr(unary_expr),
            Expr::Binary(binary_expr) => self.resolve_binary_expr(binary_expr),
            Expr::Grouping(grouping) => self.resolve_grouping_expr(grouping),
            Expr::Variable(variable_expr) => self.resolve_variable_expr(variable_expr),
            Expr::Assign(assign) => self.resolve_assign_expr(assign),
            Expr::Logical(logical_expr) => self.resolve_logical_expr(logical_expr),
            Expr::Call(call) => self.resolve_call_expr(call),
        }
    }

    fn resolve_unary_expr(&mut self, unary_expr: &Spanned<UnaryExpr>) {
        self.resolve_expr(&unary_expr.node.expr);
    }

    fn resolve_binary_expr(&mut self, binary_expr: &Spanned<BinaryExpr>) {
        self.resolve_expr(&binary_expr.node.left);
        self.resolve_expr(&binary_expr.node.right);
    }

    fn resolve_grouping_expr(&mut self, grouping_expr: &Spanned<Box<Expr>>) {
        self.resolve_expr(grouping_expr.node.deref());
    }

    fn resolve_variable_expr(&mut self, variable_expr: &Ident) {
        todo!()
    }

    fn resolve_assign_expr(&mut self, assign_expr: &Spanned<AssignExpr>) {
        todo!()
    }

    fn resolve_logical_expr(&mut self, logical_expr: &Spanned<LogicalExpr>) {
        self.resolve_expr(&logical_expr.node.left);
        self.resolve_expr(&logical_expr.node.right);
    }

    fn resolve_call_expr(&mut self, call_expr: &Spanned<CallExpr>) {
        todo!()
    }
}
