use crate::token::{Token, KrToken};
use crate::kr::Kr;
use crate::error::KrParseError;

/*
Grammar:
expr => <term> <op> <expr> | <term> <expr> | <term>
term => <KrToken> | <(> <expr> <)>

*/

// (2)
// So there's a (, ok
// parse_expr( "2)" ) -> 2 is a term, now parse_expr( ")" ) -> Error


pub fn parse(tokens:&[Token]) -> Result<Kr, KrParseError> {
    if tokens.len() == 0 { return Ok(Kr::Null) };
    let (res, n) = parse_expr(tokens, 0)?;
    if n == tokens.len() {
        Ok(res)
    } else {
        Err(KrParseError::IncompleteParse)
    }
}

fn parse_expr(tokens:&[Token], i: usize) -> Result<(Kr, usize), KrParseError> {
    let (term, j) = parse_term(tokens, i)?;
    let t = tokens.get(j);
    match t {
        Some(&Token::KrToken(KrToken::Operator(ref op))) => {
            let (expr , k) = parse_expr(tokens, j+1)?;
            Ok((Kr::NN(vec![Kr::Op(op.parse()), term, expr]), k))
        }
        Some(Token::KrToken(_)) => {
            let (expr, k) = parse_expr(tokens, j)?;
            Ok((Kr::NN(vec![term, expr]), k))
        }
        _ => {
            Ok((term, j))
        }
    }
}

fn parse_term(tokens:&[Token], i: usize) -> Result<(Kr, usize), KrParseError> {
    let t = tokens.get(i).ok_or(KrParseError::UnexpectedEOF)?;
    let j = i + 1;
    match t {
        Token::KrToken(kr) => Ok((kr.parse(), j)),
        Token::LParen => {
            let (kr, k) = parse_expr(tokens, j)?;
            if let Some(&Token::RParen) = tokens.get(k) {
                Ok((kr, k+1))
            } else {
                Err(KrParseError::MissingRParen)
            }
        }
        Token::RParen => Err(KrParseError::UnexpectedRParen),
    }
}