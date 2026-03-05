use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take_while, take_while1},
    character::complete::{char, digit1},
    combinator::{map, opt, recognize, value},
    sequence::{delimited, pair, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Token {
    Fn,
    Enum,
    Trait,
    Pipeline,
    Step,
    Let,
    Return,
    If,
    Else,
    Match,
    While,
    For,
    Break,
    Continue,
    In,
    Await,
    True,
    False,
    Null,
    Ident(String),
    Int(String),
    Float(String),
    Str(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Colon,
    Equal,
    Arrow,
    ThinArrow,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    DoubleStar,
    DotDot,
    Dot,
    Bang,
    Pipe,
    At,
    Percent,
    PercentEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenSpan {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum LexerError {
    #[error("unexpected character '{0}'")]
    Unexpected(char, usize),
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let tokens = lex_with_spans(input)?;
    Ok(tokens.into_iter().map(|t| t.token).collect())
}

pub fn lex_with_spans(input: &str) -> Result<Vec<TokenSpan>, LexerError> {
    let mut rest = input;
    let mut tokens = Vec::new();

    loop {
        rest = skip_ws_and_comments(rest);
        if rest.is_empty() {
            break;
        }

        if rest.starts_with('.')
            && !rest.starts_with("..")
            && rest.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
        {
            let pos = input.len().saturating_sub(rest.len());
            return Err(LexerError::Unexpected('.', pos));
        }

        let start = input.len().saturating_sub(rest.len());
        match token(rest) {
            Ok((next, tok)) => {
                if matches!(tok, Token::Int(_)) && next.starts_with('.') && !next.starts_with("..")
                {
                    let pos = input.len().saturating_sub(next.len());
                    return Err(LexerError::Unexpected('.', pos));
                }
                let end = input.len().saturating_sub(next.len());
                tokens.push(TokenSpan {
                    token: tok,
                    span: Span { start, end },
                });
                rest = next;
            }
            Err(_) => {
                let pos = input.len().saturating_sub(rest.len());
                let ch = rest.chars().next().unwrap();
                return Err(LexerError::Unexpected(ch, pos));
            }
        }
    }

    Ok(tokens)
}

fn skip_ws_and_comments(input: &str) -> &str {
    let mut rest = input;
    loop {
        let trimmed = rest.trim_start();
        if trimmed.starts_with("//") {
            if let Some(idx) = trimmed.find('\n') {
                rest = &trimmed[idx + 1..];
                continue;
            } else {
                return "";
            }
        }
        if trimmed.starts_with("/*") {
            if let Some(idx) = trimmed.find("*/") {
                rest = &trimmed[idx + 2..];
                continue;
            } else {
                // Unterminated block comment - treat rest as comment
                return "";
            }
        }
        return trimmed;
    }
}

fn token(input: &str) -> IResult<&str, Token> {
    alt((literal, symbol, ident_or_keyword))(input)
}

fn ident_or_keyword(input: &str) -> IResult<&str, Token> {
    map(
        recognize(pair(
            alt((
                take_while1(|c: char| c.is_alphabetic() || c == '_'),
                tag("_"),
            )),
            take_while(|c: char| c.is_alphanumeric() || c == '_'),
        )),
        |s: &str| match s {
            "fn" => Token::Fn,
            "enum" => Token::Enum,
            "trait" => Token::Trait,
            "pipeline" => Token::Pipeline,
            "step" => Token::Step,
            "let" => Token::Let,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "match" => Token::Match,
            "while" => Token::While,
            "for" => Token::For,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "in" => Token::In,
            "await" => Token::Await,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => Token::Ident(s.to_string()),
        },
    )(input)
}

fn literal(input: &str) -> IResult<&str, Token> {
    alt((
        map(float_lit, Token::Float),
        map(int_lit, Token::Int),
        map(string_lit, Token::Str),
    ))(input)
}

fn symbol(input: &str) -> IResult<&str, Token> {
    alt((symbol_two_char, symbol_one_char))(input)
}

fn symbol_two_char(input: &str) -> IResult<&str, Token> {
    alt((
        value(Token::ThinArrow, tag("->")),
        value(Token::Arrow, tag("=>")),
        value(Token::EqualEqual, tag("==")),
        value(Token::BangEqual, tag("!=")),
        value(Token::LessEqual, tag("<=")),
        value(Token::GreaterEqual, tag(">=")),
        value(Token::AndAnd, tag("&&")),
        value(Token::OrOr, tag("||")),
        value(Token::PlusEqual, tag("+=")),
        value(Token::MinusEqual, tag("-=")),
        value(Token::StarEqual, tag("*=")),
        value(Token::SlashEqual, tag("/=")),
        value(Token::DoubleStar, tag("**")),
        value(Token::DotDot, tag("..")),
        value(Token::PercentEqual, tag("%=")),
    ))(input)
}

fn symbol_one_char(input: &str) -> IResult<&str, Token> {
    alt((
        value(Token::LParen, char('(')),
        value(Token::RParen, char(')')),
        value(Token::LBrace, char('{')),
        value(Token::RBrace, char('}')),
        value(Token::LBracket, char('[')),
        value(Token::RBracket, char(']')),
        value(Token::Semicolon, char(';')),
        value(Token::Comma, char(',')),
        value(Token::Colon, char(':')),
        value(Token::Equal, char('=')),
        value(Token::Less, char('<')),
        value(Token::Greater, char('>')),
        value(Token::Plus, char('+')),
        value(Token::Minus, char('-')),
        value(Token::Star, char('*')),
        value(Token::Slash, char('/')),
        value(Token::Dot, char('.')),
        value(Token::Bang, char('!')),
        value(Token::Pipe, char('|')),
        value(Token::At, char('@')),
        value(Token::Percent, char('%')),
    ))(input)
}

fn int_lit(input: &str) -> IResult<&str, String> {
    map(recognize(digit1), |s: &str| s.to_string())(input)
}

fn float_lit(input: &str) -> IResult<&str, String> {
    map(
        recognize(tuple((
            digit1,
            char('.'),
            digit1,
            opt(tuple((
                alt((char('e'), char('E'))),
                opt(alt((char('+'), char('-')))),
                digit1,
            ))),
        ))),
        |s: &str| s.to_string(),
    )(input)
}

fn string_lit(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(
            opt(escaped_transform(
                is_not("\\\""),
                '\\',
                alt((
                    value("\\", tag("\\")),
                    value("\"", tag("\"")),
                    value("\n", tag("n")),
                    value("\r", tag("r")),
                    value("\t", tag("t")),
                )),
            )),
            |s| s.unwrap_or_default(),
        ),
        char('"'),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_keywords_and_idents() {
        let tokens =
            lex("fn let if else match while for in return await true false null foo bar").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Let,
                Token::If,
                Token::Else,
                Token::Match,
                Token::While,
                Token::For,
                Token::In,
                Token::Return,
                Token::Await,
                Token::True,
                Token::False,
                Token::Null,
                Token::Ident("foo".to_string()),
                Token::Ident("bar".to_string()),
            ]
        );
    }
}
