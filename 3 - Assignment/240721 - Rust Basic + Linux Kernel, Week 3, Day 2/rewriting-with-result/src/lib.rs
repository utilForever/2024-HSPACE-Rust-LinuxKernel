use std::iter::Peekable;
use std::str::Chars;

// An arithmetic operator.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

// A token in the expression language.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

// An expression in the expression language.
#[derive(Debug, PartialEq)]
enum Expression {
    // A reference to a variable.
    Var(String),
    // A literal number.
    Number(u32),
    // A binary operation.
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let c = self.0.next()?;

        match c {
            '0'..='9' => {
                let mut num = String::from(c);
                while let Some(c @ '0'..='9') = self.0.peek() {
                    num.push(*c);
                    self.0.next();
                }
                Some(Token::Number(num))
            }
            'a'..='z' => {
                let mut ident = String::from(c);
                while let Some(c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
                    ident.push(*c);
                    self.0.next();
                }
                Some(Token::Identifier(ident))
            }
            '+' => Some(Token::Operator(Op::Add)),
            '-' => Some(Token::Operator(Op::Sub)),
            _ => panic!("Unexpected character {c}"),
        }
    }
}

fn parse(input: &str) -> Expression {
    let mut tokens = tokenize(input);

    fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Expression {
        let Some(tok) = tokens.next() else {
            panic!("Unexpected end of input");
        };

        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse().expect("Invalid 32-bit integer'");
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => panic!("Unexpected token {tok:?}"),
        };

        // Look ahead to parse a binary operation if present.
        match tokens.next() {
            None => expr,
            Some(Token::Operator(op)) => {
                Expression::Operation(Box::new(expr), op, Box::new(parse_expr(tokens)))
            }
            Some(tok) => panic!("Unexpected token {tok:?}"),
        }
    }

    parse_expr(&mut tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expression_valid() {
        let expr = parse("10+foo+20-30");
        assert!(expr.is_ok());
        assert_eq!(format!("{:?}", expr.unwrap()), "Operation(Number(10), Add, Operation(Var(\"foo\"), Add, Operation(Number(20), Sub, Number(30))))");
    }

    #[test]
    fn expression_invalid() {
        let expr = parse("10+foo+20-");
        assert!(expr.is_err());
        assert_eq!(format!("{:?}", expr.unwrap_err()), "UnexpectedEOF");
    }
}
