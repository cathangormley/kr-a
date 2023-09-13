use crate::token::{Token, KrToken};
use crate::kr::Kr;

// Returns index of first element of v that satisfies some condition, ignores first n characters
// TODO: Get rid of this?
fn find_first_index<T, F>(v: &Vec<T>, cond: F, n: usize) -> usize
where
    F: Fn(&T) -> bool,
{
    for (i, c) in v.iter().enumerate().skip(n) {
        if cond(c) { return i }
    }
    return v.len()
}

pub fn parse(tokens:&Vec<Token>) -> Kr {
    // The result of parsing is an AST which itself is a Kr object
    let first_token = match tokens.get(0) {
        Some(token) => token,
        None => return Kr::Null
    };

    match first_token {
        Token::LParen => { 
            let n = find_first_index(tokens, |x: &Token| match x { Token::RParen => true, _ => false }, 1);
            Kr::NN(vec![parse(&tokens[1..n].to_vec()), parse(&tokens[n+1..].to_vec())])
        },
        Token::RParen => { Kr::Null },
        Token::KrToken(kr_token) => {
            match tokens.get(1) {
                Some(Token::KrToken(KrToken::Operator(op))) => Kr::NN(vec![Kr::Op(op.parse()), kr_token.to_kr(), parse(&tokens[2..].to_vec())]),
                Some(_token) => Kr::NN(vec![kr_token.to_kr(), parse(&tokens[1..].to_vec())]),
                None => kr_token.to_kr(),
            }
        }
    }
}