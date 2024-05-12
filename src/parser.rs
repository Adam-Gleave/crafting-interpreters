use crate::ast::Expr;
use crate::scanner::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: impl IntoIterator<Item = Token>) -> Self {
        Self {
            tokens: tokens.into_iter().collect(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while self.match_any([Token::Ne, Token::EqEq]) {
            let operator = self.previous().cloned().unwrap();
            let rhs = self.parse_comparison();

            expr = Expr::Binary(Box::new(expr), operator, Box::new(rhs));
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while self.match_any([Token::Gt, Token::Ge, Token::Lt, Token::Le]) {
            let operator = self.previous().cloned().unwrap();
            let rhs = self.parse_term();

            expr = Expr::Binary(Box::new(expr), operator, Box::new(rhs));
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while self.match_any([Token::Plus, Token::Minus]) {
            let operator = self.previous().cloned().unwrap();
            let rhs = self.parse_factor();

            expr = Expr::Binary(Box::new(expr), operator, Box::new(rhs));
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while self.match_any([Token::Slash, Token::Star]) {
            let operator = self.previous().cloned().unwrap();
            let rhs = self.parse_unary();

            expr = Expr::Binary(Box::new(expr), operator, Box::new(rhs));
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        if self.match_any([Token::Not, Token::Minus]) {
            let operator = self.previous().cloned().unwrap();
            let rhs = self.parse_unary();

            return Expr::Unary(operator, Box::new(rhs));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        if self.match_any([Token::False, Token::True, Token::Nil])
            || self.match_number()
            || self.match_string()
        {
            return Expr::Literal(self.previous().cloned().unwrap());
        }

        if self.match_any([Token::LeftParen]) {
            let expr = self.parse_expression();

            if !self.match_any([Token::RightParen]) {
                panic!("Missing closing parenthesis");
            }

            return Expr::Grouping(Box::new(expr));
        }

        panic!("Failed to parse primary expression")
    }

    fn match_any(&mut self, iter: impl IntoIterator<Item = Token>) -> bool {
        for token in iter {
            if !self.is_at_end() && self.peek().is_some_and(|t| t == &token) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn match_string(&mut self) -> bool {
        if matches!(self.peek(), Some(Token::String(_))) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_number(&mut self) -> bool {
        if matches!(self.peek(), Some(Token::Number(_))) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.current.checked_sub(1).and_then(|i| self.tokens.get(i))
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.tokens
            .get(self.current)
            .map_or(true, |token| matches!(token, Token::Eof))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Token;

    #[test]
    fn test_parse() {
        let tokens = vec![
            Token::Number(6.0),
            Token::Slash,
            Token::Number(3.0),
            Token::Minus,
            Token::Number(1.0),
            Token::Eof,
        ];

        let expr = Parser::new(tokens).parse();
        println!("{expr:?}");
    }
}
