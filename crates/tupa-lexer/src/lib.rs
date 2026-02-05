use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take_while, take_while1},
    character::complete::{char, digit1},
    combinator::{map, recognize, value},
    sequence::{delimited, pair},
    IResult,
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Fn,
    Let,
    Ident(String),
    Int(String),
    Str(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("unexpected character '{0}' at position {1}")]
    Unexpected(char, usize),
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut rest = input;
    let mut tokens = Vec::new();

    loop {
        rest = skip_ws_and_comments(rest);
        if rest.is_empty() {
            break;
        }

        match token(rest) {
            Ok((next, tok)) => {
                tokens.push(tok);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_keywords_and_idents() {
        let tokens = lex("fn let foo bar").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Let,
                Token::Ident("foo".into()),
                Token::Ident("bar".into()),
            ]
        );
    }

    #[test]
    fn lex_punct_and_int() {
        let tokens = lex("(x) { let y; 123 }").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Ident("x".into()),
                Token::RParen,
                Token::LBrace,
                Token::Let,
                Token::Ident("y".into()),
                Token::Semicolon,
                Token::Int("123".into()),
                Token::RBrace,
            ]
        );
    }

    #[test]
    fn lex_string_literal() {
        let tokens = lex("\"ola\\n\"").unwrap();
        assert_eq!(tokens, vec![Token::Str("ola\\n".into())]);
    }

    #[test]
    fn lex_skips_comments() {
        let tokens = lex("// c\nfn /* x */ let").unwrap();
        assert_eq!(tokens, vec![Token::Fn, Token::Let]);
    }
}

fn token(input: &str) -> IResult<&str, Token> {
    alt((
        punct,
        string_lit,
        int_lit,
        ident_or_keyword,
    ))(input)
}

fn punct(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("("), |_| Token::LParen),
        map(tag(")"), |_| Token::RParen),
        map(tag("{"), |_| Token::LBrace),
        map(tag("}"), |_| Token::RBrace),
        map(tag(";"), |_| Token::Semicolon),
        map(tag(","), |_| Token::Comma),
    ))(input)
}

fn string_lit(input: &str) -> IResult<&str, Token> {
    let esc = escaped_transform(
        is_not("\\\""),
        '\\',
        alt((
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\n", tag("n")),
            value("\t", tag("t")),
        )),
    );

    map(delimited(char('"'), esc, char('"')), Token::Str)(input)
}

fn int_lit(input: &str) -> IResult<&str, Token> {
    map(recognize(digit1), |s: &str| Token::Int(s.to_string()))(input)
}

fn ident_or_keyword(input: &str) -> IResult<&str, Token> {
    map(
        recognize(pair(
            take_while1(is_ident_start),
            take_while(is_ident_continue),
        )),
        |s: &str| match s {
            "fn" => Token::Fn,
            "let" => Token::Let,
            _ => Token::Ident(s.to_string()),
        },
    )(input)
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

fn skip_ws_and_comments(mut input: &str) -> &str {
    loop {
        let trimmed = input.trim_start_matches(|c: char| c.is_whitespace());

        if let Some(rest) = trimmed.strip_prefix("//") {
            if let Some(pos) = rest.find('\n') {
                input = &rest[pos + 1..];
                continue;
            }
            return "";
        }

        if let Some(rest) = trimmed.strip_prefix("/*") {
            if let Some(pos) = rest.find("*/") {
                input = &rest[pos + 2..];
                continue;
            }
            return "";
        }

        return trimmed;
    }
}
