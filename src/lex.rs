use crate::text::Text;
use crate::token::{Token, KrToken, NameToken, NumberToken, OperatorToken, QuotedToken, SymbolToken};


fn read_number(input: &[u8]) -> usize {
    let mut i = 0;
    for &c in input.iter() {
        let end = match c {
            b'0'..=b'9' => { i = i + 1; false },
            b'e' | b'f' | b'i' | b'j' => { i = i + 1; true },
            _ => true,
        };
        if end { break; }
    }
    i
}

pub fn lex(input: &String) -> Vec<Token> {
    // input.split_whitespace().collect()
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0; // Index
    // let input = input.as_bytes().to_owned();
    let input = Text::from_str(input);
    while i < input.len() {
        // Determine what type of token by looking at next character
        // Then possibly look ahead to find end of current token
        let c  = input.get(i).unwrap();
        let j: usize;
        let tok: Token;
        match c {
            b'a'..=b'z' | b'A'..=b'Z' => {
                // Name - must look ahead
                j = input.find_first(|x: &u8| !x.is_ascii_alphabetic(), i);
                tok = Token::KrToken(KrToken::Name(NameToken::new(Text::from_slice(&input.0[i..j]))));
            },
            b'0'..=b'9' => {
                // Number - must look ahead
                j = i + read_number(&input.0[i..]);
                tok = Token::KrToken(KrToken::Number(NumberToken::new(Text::from_slice(&input.0[i..j]))));
            },
            b'+' | b'-' | b'*' | b'%' | b':' | b',' => {
                // Operator - push now
                j = i + 1;
                tok = Token::KrToken(KrToken::Operator(OperatorToken::new(Text::from_slice(&input.0[i..j]))));
            },
            b'"' => {
                j = 1 + input.find_first(|x: &u8| *x == b'"', i+1);
                tok = Token::KrToken(KrToken::Quoted(QuotedToken::new(Text::from_slice(&input.0[i+1..j-1])))); 
            },
            b'`' => {
                j = input.find_first(|x: &u8| !x.is_ascii_alphabetic(), i+1);
                tok = Token::KrToken(KrToken::Symbol(SymbolToken::new(Text::from_slice(&input.0[i+1..j])))); 
            },
            b'(' => {
                j = i + 1;
                tok = Token::LParen;
            },
            b')' => {
                j = i + 1;
                tok = Token::RParen;
            },
            _ => {
                // Other - ignore
                j = i + 1;
                i = j;
                continue;
            },
        };
        tokens.push(tok);
        i = j;
    }
    tokens
}