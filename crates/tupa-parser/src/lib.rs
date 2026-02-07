use thiserror::Error;
use tupa_lexer::{lex_with_spans, Span, Token, TokenSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(Function),
    Enum(EnumDef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        expr: Expr,
    },
    Return(Option<Expr>),
    While {
        condition: Expr,
        body: Block,
    },
    For {
        name: String,
        iter: Expr,
        body: Block,
    },
    Break,
    Continue,
    Expr(Expr),
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Ident(String),
    Safe {
        base: Box<Type>,
        constraints: Vec<String>,
    },
    Array {
        elem: Box<Type>,
        len: i64,
    },
    Slice {
        elem: Box<Type>,
    },
    Func {
        params: Vec<Type>,
        ret: Box<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
    Ident(String),
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    AssignIndex {
        expr: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    ArrayLiteral(Vec<Expr>),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Field {
        expr: Box<Expr>,
        field: FieldAccess,
    },
    Index {
        expr: Box<Expr>,
        index: Box<Expr>,
    },
    Await(Box<Expr>),
    Block(Block),
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
    pub pattern_span: Span,
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
pub enum FieldAccess {
    Ident(String),
    Index(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Range,
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
    #[error("unexpected token {0:?} at {1:?}")]
    Unexpected(Token, Span),
    #[error("expected ';' after expression")]
    MissingSemicolon(Span),
    #[error("unexpected end of input at position {0}")]
    Eof(usize),
}

impl Expr {
    fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

fn merge_span(start: Span, end: Span) -> Span {
    Span {
        start: start.start,
        end: end.end,
    }
}

pub fn parse_program(input: &str) -> Result<Program, ParserError> {
    let tokens = lex_with_spans(input).map_err(|e| ParserError::Lexer(e.to_string()))?;
    let mut parser = Parser::new(tokens, input.len());
    let mut items = Vec::new();

    while !parser.is_eof() {
        items.push(parser.parse_item()?);
    }

    Ok(Program { items })
}

struct Parser {
    tokens: Vec<TokenSpan>,
    pos: usize,
    eof_pos: usize,
}

impl Parser {
    fn new(tokens: Vec<TokenSpan>, eof_pos: usize) -> Self {
        Self {
            tokens,
            pos: 0,
            eof_pos,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|t| &t.token)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1).map(|t| &t.token)
    }

    fn is_index_assignment(&self) -> bool {
        if !matches!(self.peek(), Some(Token::Ident(_))) {
            return false;
        }
        if !matches!(self.peek_next(), Some(Token::LBracket)) {
            return false;
        }
        let mut depth = 0usize;
        let mut idx = self.pos + 1;
        while idx < self.tokens.len() {
            match &self.tokens[idx].token {
                Token::LBracket => depth += 1,
                Token::RBracket => {
                    if depth == 1 {
                        return matches!(
                            self.tokens.get(idx + 1).map(|t| &t.token),
                            Some(Token::Equal)
                        );
                    }
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
            idx += 1;
        }
        false
    }

    fn next(&mut self) -> Option<TokenSpan> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParserError> {
        match self.next() {
            Some(TokenSpan { token, .. }) if token == expected => Ok(()),
            Some(TokenSpan { token, span }) => Err(ParserError::Unexpected(token, span)),
            None => Err(ParserError::Eof(self.eof_pos)),
        }
    }

    fn expect_span(&mut self, expected: Token) -> Result<Span, ParserError> {
        match self.next() {
            Some(TokenSpan { token, span }) if token == expected => Ok(span),
            Some(TokenSpan { token, span }) => Err(ParserError::Unexpected(token, span)),
            None => Err(ParserError::Eof(self.eof_pos)),
        }
    }

    fn parse_item(&mut self) -> Result<Item, ParserError> {
        match self.peek() {
            Some(Token::Fn) => Ok(Item::Function(self.parse_function()?)),
            Some(Token::Enum) => Ok(Item::Enum(self.parse_enum()?)),
            Some(token) => {
                let span = self.tokens.get(self.pos).map(|t| t.span).unwrap_or(Span {
                    start: self.eof_pos,
                    end: self.eof_pos,
                });
                Err(ParserError::Unexpected(token.clone(), span))
            }
            None => Err(ParserError::Eof(self.eof_pos)),
        }
    }

    fn parse_function(&mut self) -> Result<Function, ParserError> {
        self.expect(Token::Fn)?;
        let name = match self.next() {
            Some(TokenSpan {
                token: Token::Ident(name),
                ..
            }) => name,
            Some(TokenSpan { token, span }) => return Err(ParserError::Unexpected(token, span)),
            None => return Err(ParserError::Eof(self.eof_pos)),
        };
        self.expect(Token::LParen)?;
        let params = self.parse_params()?;
        self.expect(Token::RParen)?;
        let return_type = if matches!(self.peek(), Some(Token::Colon)) {
            self.next();
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(Function {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_enum(&mut self) -> Result<EnumDef, ParserError> {
        self.expect(Token::Enum)?;
        let name = match self.next() {
            Some(TokenSpan {
                token: Token::Ident(name),
                ..
            }) => name,
            Some(TokenSpan { token, span }) => return Err(ParserError::Unexpected(token, span)),
            None => return Err(ParserError::Eof(self.eof_pos)),
        };
        self.expect(Token::LBrace)?;
        let mut variants = Vec::new();
        while let Some(tok) = self.peek() {
            if *tok == Token::RBrace {
                break;
            }
            match self.next() {
                Some(TokenSpan {
                    token: Token::Ident(variant),
                    ..
                }) => variants.push(variant),
                Some(TokenSpan { token, span }) => {
                    return Err(ParserError::Unexpected(token, span))
                }
                None => return Err(ParserError::Eof(self.eof_pos)),
            }
            if let Some(Token::Comma) = self.peek() {
                self.next();
            }
        }
        self.expect(Token::RBrace)?;
        Ok(EnumDef { name, variants })
    }

    fn parse_block(&mut self) -> Result<Block, ParserError> {
        let (body, _) = self.parse_block_with_span()?;
        Ok(body)
    }

    fn parse_block_with_span(&mut self) -> Result<(Block, Span), ParserError> {
        let start = self.expect_span(Token::LBrace)?;
        let mut body = Vec::new();
        while let Some(tok) = self.peek() {
            if *tok == Token::RBrace {
                break;
            }
            body.push(self.parse_stmt_in_block()?);
        }
        let end = self.expect_span(Token::RBrace)?;
        Ok((body, merge_span(start, end)))
    }

    fn parse_stmt_in_block(&mut self) -> Result<Stmt, ParserError> {
        match self.peek() {
            Some(Token::Let) => {
                self.next();
                let name = match self.next() {
                    Some(TokenSpan {
                        token: Token::Ident(name),
                        ..
                    }) => name,
                    Some(TokenSpan { token, span }) => {
                        return Err(ParserError::Unexpected(token, span))
                    }
                    None => return Err(ParserError::Eof(self.eof_pos)),
                };
                let ty = if matches!(self.peek(), Some(Token::Colon)) {
                    self.next();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.expect(Token::Equal)?;
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Let { name, ty, expr })
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
            Some(Token::While) => {
                self.next();
                let condition = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Stmt::While { condition, body })
            }
            Some(Token::For) => {
                self.next();
                let name = match self.next() {
                    Some(TokenSpan {
                        token: Token::Ident(name),
                        ..
                    }) => name,
                    Some(TokenSpan { token, span }) => {
                        return Err(ParserError::Unexpected(token, span))
                    }
                    None => return Err(ParserError::Eof(self.eof_pos)),
                };
                self.expect(Token::In)?;
                let iter = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Stmt::For { name, iter, body })
            }
            Some(Token::Break) => {
                self.next();
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Break)
            }
            Some(Token::Continue) => {
                self.next();
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Continue)
            }
            Some(Token::LBrace) => {
                let (block, span) = self.parse_block_with_span()?;
                Ok(Stmt::Expr(Expr::new(ExprKind::Block(block), span)))
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
                if matches!(self.peek(), Some(Token::Semicolon)) {
                    self.next();
                    return Ok(Stmt::Expr(expr));
                }
                if matches!(self.peek(), Some(Token::RBrace)) {
                    return Ok(Stmt::Expr(expr));
                }
                Err(ParserError::MissingSemicolon(expr.span))
            }
        }
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, ParserError> {
        let mut params = Vec::new();
        if matches!(self.peek(), Some(Token::RParen)) {
            return Ok(params);
        }
        loop {
            let name = match self.next() {
                Some(TokenSpan {
                    token: Token::Ident(name),
                    ..
                }) => name,
                Some(TokenSpan { token, span }) => {
                    return Err(ParserError::Unexpected(token, span))
                }
                None => return Err(ParserError::Eof(self.eof_pos)),
            };
            self.expect(Token::Colon)?;
            let ty = self.parse_type()?;
            params.push(Param { name, ty });

            if matches!(self.peek(), Some(Token::Comma)) {
                self.next();
                if matches!(self.peek(), Some(Token::RParen)) {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        match self.next() {
            Some(TokenSpan {
                token: Token::Ident(name),
                ..
            }) => {
                if name == "Safe" {
                    self.expect(Token::Less)?;
                    let base = self.parse_type()?;
                    self.expect(Token::Comma)?;
                    let mut constraints = Vec::new();
                    self.expect(Token::Bang)?;
                    let first = match self.next() {
                        Some(TokenSpan {
                            token: Token::Ident(constraint),
                            ..
                        }) => constraint,
                        Some(TokenSpan { token, span }) => {
                            return Err(ParserError::Unexpected(token, span))
                        }
                        None => return Err(ParserError::Eof(self.eof_pos)),
                    };
                    constraints.push(first);
                    while matches!(self.peek(), Some(Token::Comma)) {
                        self.next();
                        self.expect(Token::Bang)?;
                        let constraint = match self.next() {
                            Some(TokenSpan {
                                token: Token::Ident(constraint),
                                ..
                            }) => constraint,
                            Some(TokenSpan { token, span }) => {
                                return Err(ParserError::Unexpected(token, span))
                            }
                            None => return Err(ParserError::Eof(self.eof_pos)),
                        };
                        constraints.push(constraint);
                    }
                    self.expect(Token::Greater)?;
                    Ok(Type::Safe {
                        base: Box::new(base),
                        constraints,
                    })
                } else {
                    Ok(Type::Ident(name))
                }
            }
            Some(TokenSpan {
                token: Token::Fn, ..
            }) => {
                self.expect(Token::LParen)?;
                let mut params = Vec::new();
                if !matches!(self.peek(), Some(Token::RParen)) {
                    params.push(self.parse_type()?);
                    while matches!(self.peek(), Some(Token::Comma)) {
                        self.next();
                        if matches!(self.peek(), Some(Token::RParen)) {
                            break;
                        }
                        params.push(self.parse_type()?);
                    }
                }
                self.expect(Token::RParen)?;
                self.expect(Token::ThinArrow)?;
                let ret = self.parse_type()?;
                Ok(Type::Func {
                    params,
                    ret: Box::new(ret),
                })
            }
            Some(TokenSpan {
                token: Token::LBracket,
                ..
            }) => {
                let elem = self.parse_type()?;
                if matches!(self.peek(), Some(Token::Semicolon)) {
                    self.next();
                    let len = match self.next() {
                        Some(TokenSpan {
                            token: Token::Int(value),
                            span,
                        }) => value
                            .parse::<i64>()
                            .map_err(|_| ParserError::Unexpected(Token::Int(value), span))?,
                        Some(TokenSpan { token, span }) => {
                            return Err(ParserError::Unexpected(token, span))
                        }
                        None => return Err(ParserError::Eof(self.eof_pos)),
                    };
                    self.expect(Token::RBracket)?;
                    Ok(Type::Array {
                        elem: Box::new(elem),
                        len,
                    })
                } else {
                    self.expect(Token::RBracket)?;
                    Ok(Type::Slice {
                        elem: Box::new(elem),
                    })
                }
            }
            Some(TokenSpan { token, span }) => Err(ParserError::Unexpected(token, span)),
            None => Err(ParserError::Eof(self.eof_pos)),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        if self.is_index_assignment() {
            let ident = self.next().ok_or(ParserError::Eof(self.eof_pos))?;
            let (name, ident_span) = match ident {
                TokenSpan {
                    token: Token::Ident(name),
                    span,
                } => (name, span),
                TokenSpan { token, span } => return Err(ParserError::Unexpected(token, span)),
            };
            self.expect(Token::LBracket)?;
            let index = self.parse_expr()?;
            self.expect(Token::RBracket)?;
            self.expect(Token::Equal)?;
            let value = self.parse_expr()?;
            let base = Expr::new(ExprKind::Ident(name), ident_span);
            let span = merge_span(ident_span, value.span);
            return Ok(Expr::new(
                ExprKind::AssignIndex {
                    expr: Box::new(base),
                    index: Box::new(index),
                    value: Box::new(value),
                },
                span,
            ));
        }
        if matches!(self.peek(), Some(Token::Ident(_)))
            && matches!(
                self.peek_next(),
                Some(
                    Token::Equal
                        | Token::PlusEqual
                        | Token::MinusEqual
                        | Token::StarEqual
                        | Token::SlashEqual
                )
            )
        {
            let ident = self.next().ok_or(ParserError::Eof(self.eof_pos))?;
            let name = match ident {
                TokenSpan {
                    token: Token::Ident(name),
                    span,
                } => (name, span),
                TokenSpan { token, span } => return Err(ParserError::Unexpected(token, span)),
            };
            let op = self.next().ok_or(ParserError::Eof(self.eof_pos))?;
            match op {
                TokenSpan {
                    token: Token::Equal,
                    ..
                } => {
                    let expr = self.parse_expr()?;
                    let span = merge_span(name.1, expr.span);
                    return Ok(Expr::new(
                        ExprKind::Assign {
                            name: name.0,
                            expr: Box::new(expr),
                        },
                        span,
                    ));
                }
                TokenSpan {
                    token: Token::PlusEqual,
                    ..
                } => {
                    let rhs = self.parse_expr()?;
                    let lhs = Expr::new(ExprKind::Ident(name.0.clone()), name.1);
                    let bin_span = merge_span(lhs.span, rhs.span);
                    let bin = Expr::new(
                        ExprKind::Binary {
                            op: BinaryOp::Add,
                            left: Box::new(lhs),
                            right: Box::new(rhs),
                        },
                        bin_span,
                    );
                    let span = merge_span(name.1, bin.span);
                    return Ok(Expr::new(
                        ExprKind::Assign {
                            name: name.0,
                            expr: Box::new(bin),
                        },
                        span,
                    ));
                }
                TokenSpan {
                    token: Token::MinusEqual,
                    ..
                } => {
                    let rhs = self.parse_expr()?;
                    let lhs = Expr::new(ExprKind::Ident(name.0.clone()), name.1);
                    let bin_span = merge_span(lhs.span, rhs.span);
                    let bin = Expr::new(
                        ExprKind::Binary {
                            op: BinaryOp::Sub,
                            left: Box::new(lhs),
                            right: Box::new(rhs),
                        },
                        bin_span,
                    );
                    let span = merge_span(name.1, bin.span);
                    return Ok(Expr::new(
                        ExprKind::Assign {
                            name: name.0,
                            expr: Box::new(bin),
                        },
                        span,
                    ));
                }
                TokenSpan {
                    token: Token::StarEqual,
                    ..
                } => {
                    let rhs = self.parse_expr()?;
                    let lhs = Expr::new(ExprKind::Ident(name.0.clone()), name.1);
                    let bin_span = merge_span(lhs.span, rhs.span);
                    let bin = Expr::new(
                        ExprKind::Binary {
                            op: BinaryOp::Mul,
                            left: Box::new(lhs),
                            right: Box::new(rhs),
                        },
                        bin_span,
                    );
                    let span = merge_span(name.1, bin.span);
                    return Ok(Expr::new(
                        ExprKind::Assign {
                            name: name.0,
                            expr: Box::new(bin),
                        },
                        span,
                    ));
                }
                TokenSpan {
                    token: Token::SlashEqual,
                    ..
                } => {
                    let rhs = self.parse_expr()?;
                    let lhs = Expr::new(ExprKind::Ident(name.0.clone()), name.1);
                    let bin_span = merge_span(lhs.span, rhs.span);
                    let bin = Expr::new(
                        ExprKind::Binary {
                            op: BinaryOp::Div,
                            left: Box::new(lhs),
                            right: Box::new(rhs),
                        },
                        bin_span,
                    );
                    let span = merge_span(name.1, bin.span);
                    return Ok(Expr::new(
                        ExprKind::Assign {
                            name: name.0,
                            expr: Box::new(bin),
                        },
                        span,
                    ));
                }
                TokenSpan { token, span } => return Err(ParserError::Unexpected(token, span)),
            }
        }
        self.parse_precedence(0)
    }

    fn parse_precedence(&mut self, min_prec: u8) -> Result<Expr, ParserError> {
        let mut left = self.parse_unary()?;

        while let Some((op, prec, right_assoc)) = self.peek().and_then(Self::token_to_binary_op) {
            if prec < min_prec {
                break;
            }

            self.next();
            let next_min_prec = if right_assoc { prec } else { prec + 1 };
            let right = self.parse_precedence(next_min_prec)?;
            let span = merge_span(left.span, right.span);
            left = Expr::new(
                ExprKind::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            );
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
        if matches!(self.peek(), Some(Token::Pipe)) {
            // Lambda: |x, y| expr
            let start = self.next().unwrap().span;
            let mut params = Vec::new();
            loop {
                match self.next() {
                    Some(TokenSpan {
                        token: Token::Ident(name),
                        ..
                    }) => params.push(name),
                    Some(TokenSpan {
                        token: Token::Pipe, ..
                    }) => break,
                    Some(TokenSpan { token, span }) => {
                        return Err(ParserError::Unexpected(token, span))
                    }
                    None => return Err(ParserError::Eof(self.eof_pos)),
                }
                if matches!(self.peek(), Some(Token::Comma)) {
                    self.next();
                }
            }
            let body = self.parse_expr()?;
            let span = merge_span(start, body.span);
            Ok(Expr::new(
                ExprKind::Lambda {
                    params,
                    body: Box::new(body),
                },
                span,
            ))
        } else {
            match self.peek() {
                Some(Token::Bang) => {
                    let span = self.next().unwrap().span;
                    let expr = self.parse_unary()?;
                    let full = merge_span(span, expr.span);
                    Ok(Expr::new(
                        ExprKind::Unary {
                            op: UnaryOp::Not,
                            expr: Box::new(expr),
                        },
                        full,
                    ))
                }
                Some(Token::Minus) => {
                    let span = self.next().unwrap().span;
                    let expr = self.parse_unary()?;
                    let full = merge_span(span, expr.span);
                    Ok(Expr::new(
                        ExprKind::Unary {
                            op: UnaryOp::Neg,
                            expr: Box::new(expr),
                        },
                        full,
                    ))
                }
                _ => self.parse_primary(),
            }
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        if matches!(self.peek(), Some(Token::LBrace)) {
            let (block, span) = self.parse_block_with_span()?;
            return Ok(Expr::new(ExprKind::Block(block), span));
        }

        let mut expr = match self.next() {
            Some(TokenSpan {
                token: Token::Int(value),
                span,
            }) => value
                .parse::<i64>()
                .map(|v| Expr::new(ExprKind::Int(v), span))
                .map_err(|_| ParserError::Unexpected(Token::Int(value), span)),
            Some(TokenSpan {
                token: Token::Float(value),
                span,
            }) => value
                .parse::<f64>()
                .map(|v| Expr::new(ExprKind::Float(v), span))
                .map_err(|_| ParserError::Unexpected(Token::Float(value), span)),
            Some(TokenSpan {
                token: Token::Str(value),
                span,
            }) => Ok(Expr::new(ExprKind::Str(value), span)),
            Some(TokenSpan {
                token: Token::True,
                span,
            }) => Ok(Expr::new(ExprKind::Bool(true), span)),
            Some(TokenSpan {
                token: Token::False,
                span,
            }) => Ok(Expr::new(ExprKind::Bool(false), span)),
            Some(TokenSpan {
                token: Token::Null,
                span,
            }) => Ok(Expr::new(ExprKind::Null, span)),
            Some(TokenSpan {
                token: Token::Ident(name),
                span,
            }) => Ok(Expr::new(ExprKind::Ident(name), span)),
            Some(TokenSpan {
                token: Token::LParen,
                span,
            }) => {
                let expr = self.parse_expr()?;
                let end = self.expect_span(Token::RParen)?;
                Ok(Expr::new(expr.kind, merge_span(span, end)))
            }
            Some(TokenSpan {
                token: Token::LBracket,
                span,
            }) => {
                let mut items = Vec::new();
                if !matches!(self.peek(), Some(Token::RBracket)) {
                    items.push(self.parse_expr()?);
                    while matches!(self.peek(), Some(Token::Comma)) {
                        self.next();
                        if matches!(self.peek(), Some(Token::RBracket)) {
                            break;
                        }
                        items.push(self.parse_expr()?);
                    }
                }
                let end = self.expect_span(Token::RBracket)?;
                Ok(Expr::new(
                    ExprKind::ArrayLiteral(items),
                    merge_span(span, end),
                ))
            }
            Some(TokenSpan {
                token: Token::Await,
                span,
            }) => {
                let expr = self.parse_expr()?;
                Ok(Expr::new(
                    ExprKind::Await(Box::new(expr.clone())),
                    merge_span(span, expr.span),
                ))
            }
            Some(TokenSpan {
                token: Token::If,
                span,
            }) => {
                let condition = self.parse_expr()?;
                let (then_branch, then_span) = self.parse_block_with_span()?;
                let (else_branch, end_span) = if matches!(self.peek(), Some(Token::Else)) {
                    self.next();
                    if matches!(self.peek(), Some(Token::If)) {
                        let else_if = self.parse_expr()?;
                        let end = else_if.span;
                        (Some(ElseBranch::If(Box::new(else_if))), end)
                    } else {
                        let (block, block_span) = self.parse_block_with_span()?;
                        (Some(ElseBranch::Block(block)), block_span)
                    }
                } else {
                    (None, then_span)
                };
                Ok(Expr::new(
                    ExprKind::If {
                        condition: Box::new(condition),
                        then_branch,
                        else_branch,
                    },
                    merge_span(span, end_span),
                ))
            }
            Some(TokenSpan {
                token: Token::Match,
                span,
            }) => {
                let expr = self.parse_expr()?;
                self.expect(Token::LBrace)?;
                let mut arms = Vec::new();
                while let Some(tok) = self.peek() {
                    if *tok == Token::RBrace {
                        break;
                    }
                    let (pattern, pattern_span) = self.parse_pattern()?;
                    let guard = if matches!(self.peek(), Some(Token::If)) {
                        self.next();
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };
                    self.expect(Token::Arrow)?;
                    let arm_expr = self.parse_expr()?;
                    if matches!(self.peek(), Some(Token::Comma)) {
                        self.next();
                    }
                    arms.push(MatchArm {
                        pattern,
                        pattern_span,
                        guard,
                        expr: arm_expr,
                    });
                }
                let end = self.expect_span(Token::RBrace)?;
                Ok(Expr::new(
                    ExprKind::Match {
                        expr: Box::new(expr),
                        arms,
                    },
                    merge_span(span, end),
                ))
            }
            Some(TokenSpan { token, span }) => Err(ParserError::Unexpected(token, span)),
            None => Err(ParserError::Eof(self.eof_pos)),
        }?;

        loop {
            match self.peek() {
                Some(Token::LParen) => {
                    self.next();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        args.push(self.parse_expr()?);
                        while matches!(self.peek(), Some(Token::Comma)) {
                            self.next();
                            args.push(self.parse_expr()?);
                        }
                    }
                    let end = self.expect_span(Token::RParen)?;
                    let span = merge_span(expr.span, end);
                    expr = Expr::new(
                        ExprKind::Call {
                            callee: Box::new(expr),
                            args,
                        },
                        span,
                    );
                }
                Some(Token::Dot) => {
                    self.next();
                    let field = match self.next() {
                        Some(TokenSpan {
                            token: Token::Ident(name),
                            span,
                        }) => (FieldAccess::Ident(name), span),
                        Some(TokenSpan {
                            token: Token::Int(value),
                            span,
                        }) => value
                            .parse::<i64>()
                            .map(FieldAccess::Index)
                            .map_err(|_| ParserError::Unexpected(Token::Int(value), span))
                            .map(|f| (f, span))?,
                        Some(TokenSpan { token, span }) => {
                            return Err(ParserError::Unexpected(token, span))
                        }
                        None => return Err(ParserError::Eof(self.eof_pos)),
                    };
                    let span = merge_span(expr.span, field.1);
                    expr = Expr::new(
                        ExprKind::Field {
                            expr: Box::new(expr),
                            field: field.0,
                        },
                        span,
                    );
                }
                Some(Token::LBracket) => {
                    self.next();
                    let index = self.parse_expr()?;
                    let end = self.expect_span(Token::RBracket)?;
                    let span = merge_span(expr.span, end);
                    expr = Expr::new(
                        ExprKind::Index {
                            expr: Box::new(expr),
                            index: Box::new(index),
                        },
                        span,
                    );
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn token_to_binary_op(token: &Token) -> Option<(BinaryOp, u8, bool)> {
        match token {
            Token::DotDot => Some((BinaryOp::Range, 0, false)),
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

    fn parse_pattern(&mut self) -> Result<(Pattern, Span), ParserError> {
        match self.next() {
            Some(TokenSpan {
                token: Token::Ident(name),
                span,
            }) => {
                if name == "_" {
                    Ok((Pattern::Wildcard, span))
                } else {
                    Ok((Pattern::Ident(name), span))
                }
            }
            Some(TokenSpan {
                token: Token::Int(value),
                span,
            }) => value
                .parse::<i64>()
                .map(|value| (Pattern::Int(value), span))
                .map_err(|_| ParserError::Unexpected(Token::Int(value), span)),
            Some(TokenSpan {
                token: Token::Str(value),
                span,
            }) => Ok((Pattern::Str(value), span)),
            Some(TokenSpan { token, span }) => Err(ParserError::Unexpected(token, span)),
            None => Err(ParserError::Eof(self.eof_pos)),
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
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        assert_eq!(func.body.len(), 2);
    }

    #[test]
    fn parse_if_expression() {
        let src = "fn main() { if 1 { return 2; } else { return 3; } }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        assert_eq!(func.body.len(), 1);
    }

    #[test]
    fn parse_match_expression() {
        let src = "fn main() { match x { 1 => foo(), _ => bar(), }; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        assert_eq!(func.body.len(), 1);
    }

    #[test]
    fn parse_binary_precedence() {
        let src = "fn main() { let x = 1 + 2 * 3; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        match &expr.kind {
            ExprKind::Binary { op, left, right } => {
                assert_eq!(*op, BinaryOp::Add);
                assert!(matches!(left.kind, ExprKind::Int(1)));
                assert!(matches!(
                    right.kind,
                    ExprKind::Binary {
                        op: BinaryOp::Mul,
                        ..
                    }
                ));
            }
            _ => panic!("expected binary expression"),
        }
    }

    #[test]
    fn parse_unary_expression() {
        let src = "fn main() { let x = -1; let y = !false; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert!(matches!(
            expr.kind,
            ExprKind::Unary {
                op: UnaryOp::Neg,
                ..
            }
        ));
        let Stmt::Let { expr, .. } = &func.body[1] else {
            panic!("expected let");
        };
        assert!(matches!(
            expr.kind,
            ExprKind::Unary {
                op: UnaryOp::Not,
                ..
            }
        ));
    }

    #[test]
    fn parse_float_literal() {
        let src = "fn main() { let x = 3.14; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert!(matches!(expr.kind, ExprKind::Float(f) if (f - 3.14).abs() < 1e-9));
    }

    #[test]
    fn parse_typed_let_and_params() {
        let src = "fn add(x: i64, y: i64): i64 { let z: i64 = x + y; return z; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.return_type, Some(Type::Ident("i64".into())));
        let Stmt::Let { ty, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert_eq!(ty, &Some(Type::Ident("i64".into())));
    }

    #[test]
    fn parse_array_and_slice_types() {
        let src = "fn main() { let a: [i64; 3] = 0; let b: [i64] = 0; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { ty, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert!(matches!(ty, Some(Type::Array { len: 3, .. })));
        let Stmt::Let { ty, .. } = &func.body[1] else {
            panic!("expected let");
        };
        assert!(matches!(ty, Some(Type::Slice { .. })));
    }

    #[test]
    fn parse_safe_type() {
        let src = "fn main() { let x: Safe<i64, !nan, !inf> = 1; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { ty, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert_eq!(
            ty,
            &Some(Type::Safe {
                base: Box::new(Type::Ident("i64".into())),
                constraints: vec!["nan".into(), "inf".into()],
            })
        );
    }

    #[test]
    fn parse_function_type() {
        let src = "fn main() { let f: fn(i64, i64) -> i64 = add; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { ty, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert_eq!(
            ty,
            &Some(Type::Func {
                params: vec![Type::Ident("i64".into()), Type::Ident("i64".into())],
                ret: Box::new(Type::Ident("i64".into())),
            })
        );
    }

    #[test]
    fn parse_while_and_for() {
        let src = "fn main() { while x < 10 { x = x + 1; } for i in xs { return i; } }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        assert!(matches!(func.body[0], Stmt::While { .. }));
        assert!(matches!(func.body[1], Stmt::For { .. }));
    }

    #[test]
    fn parse_array_literal() {
        let src = "fn main() { let xs = [1, 2, 3]; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert!(matches!(expr.kind, ExprKind::ArrayLiteral(ref items) if items.len() == 3));
    }

    #[test]
    fn parse_range_expression() {
        let src = "fn main() { let xs = 1..10; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert!(matches!(
            expr.kind,
            ExprKind::Binary {
                op: BinaryOp::Range,
                ..
            }
        ));
    }

    #[test]
    fn parse_match_with_guard() {
        let src = "fn main() { match x { y if y > 0 => y, _ => 0, } }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        assert_eq!(func.body.len(), 1);
    }

    #[test]
    fn parse_await_and_block_expr() {
        let src = "fn main() { let x = await foo(); { return x; } }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        assert_eq!(func.body.len(), 2);
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert!(matches!(expr.kind, ExprKind::Await(_)));
        assert!(
            matches!(func.body[1], Stmt::Expr(ref expr) if matches!(expr.kind, ExprKind::Block(_)))
        );
    }

    #[test]
    fn parse_postfix_chain() {
        let src = "fn main() { let x = foo(1).bar[0]; }";
        let program = parse_program(src).unwrap();
        let func = match &program.items[0] {
            Item::Enum(_) => panic!("expected function"),
            Item::Function(func) => func,
        };
        let Stmt::Let { expr, .. } = &func.body[0] else {
            panic!("expected let");
        };
        assert!(matches!(expr.kind, ExprKind::Index { .. }));
    }
}
