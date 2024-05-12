use crate::ast::Expr;
use crate::ast::Visitor;

pub struct PrettyPrinter;

impl Visitor<String> for PrettyPrinter {
    fn visit_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(token) => token.to_string(),
            Expr::Unary(token, expr) => parenthesize([token.to_string(), self.visit_expr(expr)]),
            Expr::Binary(lhs, operator, rhs) => parenthesize([
                operator.to_string(),
                self.visit_expr(lhs),
                self.visit_expr(rhs),
            ]),
            Expr::Grouping(expr) => parenthesize(["group".to_string(), self.visit_expr(expr)]),
        }
    }
}

fn parenthesize(list: impl IntoIterator<Item = String>) -> String {
    let mut iter = list.into_iter();

    let mut s = String::new();
    s.push('(');

    if let Some(first) = iter.next() {
        s.push_str(&first);
    }

    for item in iter {
        s.push(' ');
        s.push_str(&item);
    }

    s.push(')');
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Token;

    #[test]
    fn test_pretty_print() {
        let expr = Expr::Binary(
            Box::new(Expr::Unary(
                Token::Minus,
                Box::new(Expr::Literal(Token::Number(123.0))),
            )),
            Token::Star,
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Token::Number(
                45.67,
            ))))),
        );

        let printer = PrettyPrinter;
        assert_eq!(
            printer.visit_expr(&expr),
            format!("(* (- 123) (group 45.67))")
        );
    }
}
