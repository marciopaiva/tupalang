use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take_while, take_while1},
    character::complete::{char, digit1},
    combinator::{map, map_res, opt, recognize, value},
    error::{Error as NomError, ErrorKind},
    sequence::{delimited, pair, tuple},
    IResult,
};
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Fn,
    Enum,
    Trait,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSpan {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("unexpected character '{0}' at position {1}")]
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
    fn lex_arrow() {
        let tokens = lex("=> ->").unwrap();
        assert_eq!(tokens, vec![Token::Arrow, Token::ThinArrow]);
    }

    #[test]
    fn lex_operators() {
        let tokens = lex("== != <= >= && || + += - -= * *= / /= ** .. . ! < > @").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::EqualEqual,
                Token::BangEqual,
                Token::LessEqual,
                Token::GreaterEqual,
                Token::AndAnd,
                Token::OrOr,
                Token::Plus,
                Token::PlusEqual,
                Token::Minus,
                Token::MinusEqual,
                Token::Star,
                Token::StarEqual,
                Token::Slash,
                Token::SlashEqual,
                Token::DoubleStar,
                Token::DotDot,
                Token::Dot,
                Token::Bang,
                Token::Less,
                Token::Greater,
                Token::At,
            ]
        );
    }

    #[test]
    fn lex_colon() {
        let tokens = lex(":").unwrap();
        assert_eq!(tokens, vec![Token::Colon]);
    }

    #[test]
    fn lex_brackets() {
        let tokens = lex("[]").unwrap();
        assert_eq!(tokens, vec![Token::LBracket, Token::RBracket]);
    }

    #[test]
    fn lex_float_literal() {
        let tokens = lex("3.14 1.0e-3").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Float("3.14".into()), Token::Float("1.0e-3".into()),]
        );
    }

    #[test]
    fn reject_incomplete_floats() {
        assert!(lex("1.").is_err());
        assert!(lex(".5").is_err());
    }

    #[test]
    fn reject_non_nfc_identifiers() {
        let nfd = "a\u{0303}";
        assert!(lex(nfd).is_err());
    }

    #[test]
    fn lex_string_literal() {
        let tokens = lex("\"ola\\n\"").unwrap();
        assert_eq!(tokens, vec![Token::Str("ola\n".into())]);
    }

    #[test]
    fn lex_skips_comments() {
        let tokens = lex("// c\nfn /* x */ let").unwrap();
        assert_eq!(tokens, vec![Token::Fn, Token::Let]);
    }
}

fn token(input: &str) -> IResult<&str, Token> {
    alt((punct, string_lit, float_lit, int_lit, ident_or_keyword))(input)
}

fn punct(input: &str) -> IResult<&str, Token> {
    let cases = [
        ("=>", Token::Arrow),
        ("->", Token::ThinArrow),
        ("==", Token::EqualEqual),
        ("!=", Token::BangEqual),
        ("<=", Token::LessEqual),
        (">=", Token::GreaterEqual),
        ("&&", Token::AndAnd),
        ("||", Token::OrOr),
        ("+=", Token::PlusEqual),
        ("-=", Token::MinusEqual),
        ("*=", Token::StarEqual),
        ("/=", Token::SlashEqual),
        ("**", Token::DoubleStar),
        ("..", Token::DotDot),
        (".", Token::Dot),
        ("(", Token::LParen),
        (")", Token::RParen),
        ("{", Token::LBrace),
        ("}", Token::RBrace),
        ("[", Token::LBracket),
        ("]", Token::RBracket),
        (";", Token::Semicolon),
        (",", Token::Comma),
        (":", Token::Colon),
        ("=", Token::Equal),
        ("<", Token::Less),
        (">", Token::Greater),
        ("+", Token::Plus),
        ("-", Token::Minus),
        ("*", Token::Star),
        ("/", Token::Slash),
        ("!", Token::Bang),
        ("|", Token::Pipe),
        ("@", Token::At),
    ];

    for (pat, tok) in cases {
        if let Some(rest) = input.strip_prefix(pat) {
            return Ok((rest, tok));
        }
    }

    Err(nom::Err::Error(NomError::new(input, ErrorKind::Tag)))
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

fn float_lit(input: &str) -> IResult<&str, Token> {
    let exp = tuple((
        alt((tag("e"), tag("E"))),
        opt(alt((tag("+"), tag("-")))),
        digit1,
    ));

    map(
        recognize(tuple((digit1, char('.'), digit1, opt(exp)))),
        |s: &str| Token::Float(s.to_string()),
    )(input)
}

fn ident_or_keyword(input: &str) -> IResult<&str, Token> {
    map_res(
        recognize(pair(
            take_while1(is_ident_start),
            take_while(is_ident_continue),
        )),
        |s: &str| {
            let normalized = s.nfc().collect::<String>();
            if normalized != s {
                return Err(());
            }
            Ok(match normalized.as_str() {
                "fn" => Token::Fn,
                "enum" => Token::Enum,
                "trait" => Token::Trait,
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
                _ => Token::Ident(normalized),
            })
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
