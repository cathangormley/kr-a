use crate::primitive::{Prim, Primitive};
use crate::token::{Token, KrToken};
use crate::kr::Kr;
use crate::error::KrParseError;

/*
Grammar:
prog := <expr> <;> <prog>                       // nyi
      | <expr>                                  // nyi
expr := <term> <op> <expr>
      | <term> <expr>
      | <term>
term := <kr>
      | <(> <expr> <)> 
      | <[> <expr> {<;> <expr> } <]>
*/


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
    // First part of a expression will always be a term
    let (term, j) = parse_term(tokens, i)?;
    // Check next token
    let next_token = tokens.get(j);
    match next_token {
        // <term> <op> <expr>
        Some(&Token::KrToken(KrToken::Operator(ref op))) => {
            let (expr , k) = parse_expr(tokens, j+1)?;
            Ok((Kr::NN(vec![Kr::Op(op.parse()), term, expr]), k))
        }
        // <term> <expr>
        Some(Token::KrToken(_)) | Some(Token::LParen) | Some(Token::LBracket) => {
            let (expr, k) = parse_expr(tokens, j)?;
            Ok((Kr::NN(vec![term, expr]), k))
        }
        // <term>
        None | Some(Token::RParen) | Some(Token::RBracket) => { Ok((term, j)) },
        Some(Token::SemiColon) => Ok((term, j)),
    }
}

fn parse_term(tokens:&[Token], i: usize) -> Result<(Kr, usize), KrParseError> {
    println!("parsing term at {}", i);
    let t = tokens.get(i).ok_or(KrParseError::UnexpectedEOF)?;
    match t {
        Token::KrToken(kr) => Ok((kr.parse(), i+1)),
        Token::LParen => {
            let (kr, k) = parse_expr(tokens, i+1)?;
            if let Some(&Token::RParen) = tokens.get(k) {
                Ok((kr, k+1))
            } else {
                Err(KrParseError::MissingRParen)
            }
        }
        Token::RParen => Err(KrParseError::UnexpectedRParen),
        Token::LBracket => {
            let mut elements: Vec<Kr> = vec![Kr::Prim(Primitive::new(Prim::Enlist))];
            let mut j = i + 1;
            loop {
                let (expr, k) = parse_expr(tokens, j)?;
                elements.push(expr);
                if let Some(Token::SemiColon) = tokens.get(k) {
                    j = k + 1;
                    continue;
                } else {
                    j = k;
                    break;
                }
            }
            if let Some(&Token::RBracket) = tokens.get(j) {
                Ok((Kr::NN(elements), j+1))
            } else {
                Err(KrParseError::MissingRBracket)
            } 

        }
        Token::RBracket => Err(KrParseError::UnexpectedRBracket),
        Token::SemiColon => Err(KrParseError::UnexpectedSemiColon),
    }
}
