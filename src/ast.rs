use crate::scanner::Token;

pub enum Expr {
    Literal(Token),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

pub trait Visitor<T> {
    fn visit_expr(&self, expr: &Expr) -> T;
}
