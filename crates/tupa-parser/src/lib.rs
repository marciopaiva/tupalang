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
        match self.next() {
            Some(Token::Int(value)) => value
                .parse::<i64>()
                .map(Expr::Int)
                .map_err(|_| ParserError::Unexpected(Token::Int(value), self.pos.saturating_sub(1))),
            Some(Token::Str(value)) => Ok(Expr::Str(value)),
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
}
