use thiserror::Error;
use tupa_lexer::{lex, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let { name: String, expr: Expr },
    Return(Option<Expr>),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Str(String),
    Bool(bool),
    Null,
    Ident(String),
    Call { callee: String, args: Vec<Expr> },
    If {
        condition: Box<Expr>,
        then_branch: Block,
        else_branch: Option<ElseBranch>,
    },
    Match {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub type Block = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum ElseBranch {
    Block(Block),
    If(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    Int(i64),
    Str(String),
    Ident(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Or,
    And,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("lexer error: {0}")]
    Lexer(String),
    #[error("unexpected token {0:?} at position {1}")]
    Unexpected(Token, usize),
    #[error("unexpected end of input")]
    Eof,
}

pub fn parse_program(input: &str) -> Result<Program, ParserError> {
    let tokens = lex(input).map_err(|e| ParserError::Lexer(e.to_string()))?;
    let mut parser = Parser::new(tokens);
    let mut items = Vec::new();

    while !parser.is_eof() {
        items.push(Item::Function(parser.parse_function()?));
    }

    Ok(Program { items })
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParserError> {
        match self.next() {
            Some(tok) if tok == expected => Ok(()),
            Some(tok) => Err(ParserError::Unexpected(tok, self.pos.saturating_sub(1))),
            None => Err(ParserError::Eof),
        }
    }

    fn parse_function(&mut self) -> Result<Function, ParserError> {
        self.expect(Token::Fn)?;
        let name = match self.next() {
            Some(Token::Ident(name)) => name,
            Some(tok) => return Err(ParserError::Unexpected(tok, self.pos.saturating_sub(1))),
            None => return Err(ParserError::Eof),
        };
        self.expect(Token::LParen)?;
        self.expect(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Function { name, body })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParserError> {
        match self.peek() {
            Some(Token::Let) => {
                self.next();
                let name = match self.next() {
                    Some(Token::Ident(name)) => name,
                    Some(tok) => return Err(ParserError::Unexpected(tok, self.pos.saturating_sub(1))),
                    None => return Err(ParserError::Eof),
                };
                self.expect(Token::Equal)?;
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Let { name, expr })
            }
            Some(Token::Return) => {
                self.next();
                let expr = if matches!(self.peek(), Some(Token::Semicolon)) {
                    None
                } else {
                    Some(self.parse_expr()?)
                };
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Return(expr))
            }
            Some(Token::If) | Some(Token::Match) => {
                let expr = self.parse_expr()?;
                if matches!(self.peek(), Some(Token::Semicolon)) {
                    self.next();
                }
                Ok(Stmt::Expr(expr))
            }
            _ => {
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Block, ParserError> {
        self.expect(Token::LBrace)?;
        let mut body = Vec::new();
        while let Some(tok) = self.peek() {
            if *tok == Token::RBrace {
                break;
            }
            body.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(body)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_precedence(0)
    }

    fn parse_precedence(&mut self, min_prec: u8) -> Result<Expr, ParserError> {
        let mut left = self.parse_unary()?;

        loop {
            let (op, prec, right_assoc) = match self.peek().and_then(Self::token_to_binary_op) {
                Some(info) => info,
                None => break,
            };

            if prec < min_prec {
                break;
            }

            self.next();
            let next_min_prec = if right_assoc { prec } else { prec + 1 };
            let right = self.parse_precedence(next_min_prec)?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
        match self.peek() {
            Some(Token::Bang) => {
                self.next();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            Some(Token::Minus) => {
                self.next();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match self.next() {
            Some(Token::Int(value)) => value
                .parse::<i64>()
                .map(Expr::Int)
                .map_err(|_| ParserError::Unexpected(Token::Int(value), self.pos.saturating_sub(1))),
            Some(Token::Str(value)) => Ok(Expr::Str(value)),
            Some(Token::True) => Ok(Expr::Bool(true)),
            Some(Token::False) => Ok(Expr::Bool(false)),
            Some(Token::Null) => Ok(Expr::Null),
            Some(Token::Ident(name)) => {
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.next();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        args.push(self.parse_expr()?);
                        while matches!(self.peek(), Some(Token::Comma)) {
                            self.next();
                            args.push(self.parse_expr()?);
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { callee: name, args })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Some(Token::If) => {
                let condition = self.parse_expr()?;
                let then_branch = self.parse_block()?;
                let else_branch = if matches!(self.peek(), Some(Token::Else)) {
                    self.next();
                    if matches!(self.peek(), Some(Token::If)) {
                        let else_if = self.parse_expr()?;
                        Some(ElseBranch::If(Box::new(else_if)))
                    } else {
                        let block = self.parse_block()?;
                        Some(ElseBranch::Block(block))
                    }
                } else {
                    None
                };
                Ok(Expr::If {
                    condition: Box::new(condition),
                    then_branch,
                    else_branch,
                })
            }
            Some(Token::Match) => {
                let expr = self.parse_expr()?;
                self.expect(Token::LBrace)?;
                let mut arms = Vec::new();
                while let Some(tok) = self.peek() {
                    if *tok == Token::RBrace {
                        break;
                    }
                    let pattern = self.parse_pattern()?;
                    let guard = if matches!(self.peek(), Some(Token::If)) {
                        self.next();
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };
                    self.expect(Token::Arrow)?;
                    let expr = self.parse_expr()?;
                    if matches!(self.peek(), Some(Token::Comma)) {
                        self.next();
                    }
                    arms.push(MatchArm { pattern, guard, expr });
                }
                self.expect(Token::RBrace)?;
                Ok(Expr::Match {
                    expr: Box::new(expr),
                    arms,
                })
            }
            Some(tok) => Err(ParserError::Unexpected(tok, self.pos.saturating_sub(1))),
            None => Err(ParserError::Eof),
        }
    }

    fn token_to_binary_op(token: &Token) -> Option<(BinaryOp, u8, bool)> {
        match token {
            Token::OrOr => Some((BinaryOp::Or, 1, false)),
            Token::AndAnd => Some((BinaryOp::And, 2, false)),
            Token::EqualEqual => Some((BinaryOp::Equal, 3, false)),
            Token::BangEqual => Some((BinaryOp::NotEqual, 3, false)),
            Token::Less => Some((BinaryOp::Less, 4, false)),
            Token::LessEqual => Some((BinaryOp::LessEqual, 4, false)),
            Token::Greater => Some((BinaryOp::Greater, 4, false)),
            Token::GreaterEqual => Some((BinaryOp::GreaterEqual, 4, false)),
            Token::Plus => Some((BinaryOp::Add, 5, false)),
            Token::Minus => Some((BinaryOp::Sub, 5, false)),
            Token::Star => Some((BinaryOp::Mul, 6, false)),
            Token::Slash => Some((BinaryOp::Div, 6, false)),
            Token::DoubleStar => Some((BinaryOp::Pow, 7, true)),
            _ => None,
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParserError> {
        match self.next() {
            Some(Token::Ident(name)) => {
                if name == "_" {
                    Ok(Pattern::Wildcard)
                } else {
                    Ok(Pattern::Ident(name))
                }
            }
            Some(Token::Int(value)) => value
                .parse::<i64>()
                .map(Pattern::Int)
                .map_err(|_| ParserError::Unexpected(Token::Int(value), self.pos.saturating_sub(1))),
            Some(Token::Str(value)) => Ok(Pattern::Str(value)),
            Some(tok) => Err(ParserError::Unexpected(tok, self.pos.saturating_sub(1))),
            None => Err(ParserError::Eof),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_function() {
        let program = parse_program("fn main() {}\n").unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn parse_let_and_return() {
        let src = "fn main() { let x = 1; return x; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Function(func) => func,
        };
        assert_eq!(func.body.len(), 2);
    }

    #[test]
    fn parse_if_expression() {
        let src = "fn main() { if 1 { return 2; } else { return 3; } }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Function(func) => func,
        };
        assert_eq!(func.body.len(), 1);
    }

    #[test]
    fn parse_match_expression() {
        let src = "fn main() { match x { 1 => foo(), _ => bar(), }; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Function(func) => func,
        };
        assert_eq!(func.body.len(), 1);
    }

    #[test]
    fn parse_binary_precedence() {
        let src = "fn main() { let x = 1 + 2 * 3; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Function(func) => func,
        };
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        match expr {
            Expr::Binary { op, left, right } => {
                assert_eq!(*op, BinaryOp::Add);
                assert!(matches!(**left, Expr::Int(1)));
                assert!(matches!(**right, Expr::Binary { op: BinaryOp::Mul, .. }));
            }
            _ => panic!("expected binary expression"),
        }
    }

    #[test]
    fn parse_unary_expression() {
        let src = "fn main() { let x = -1; let y = !false; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Function(func) => func,
        };
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert!(matches!(expr, Expr::Unary { op: UnaryOp::Neg, .. }));
        let Stmt::Let { expr, .. } = &func.body[1] else {
            panic!("expected let");
        };
        assert!(matches!(expr, Expr::Unary { op: UnaryOp::Not, .. }));
    }
}
