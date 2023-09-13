use crate::text::Text;
use crate::token::{Name, Number, Quoted, Symbol, Token};
use crate::operator::Operator;

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
        match c {
            b'a'..=b'z' | b'A'..=b'Z' => {
                // Name - must look ahead
                j = input.find_first(|x: &u8| !x.is_ascii_alphabetic(), i);
                // tokens.push(Token::Name(Name { text: input[i..j].to_vec() }));
                tokens.push(Token::Name(Name::new(Text::from_slice(&input.0[i..j]))))
            },
            b'0'..=b'9' => {
                // Number - must look ahead
                j = i + read_number(&input.0[i..]);
                tokens.push(Token::Number(Number::new(Text::from_slice(&input.0[i..j]))));
            },
            b'+' | b'-' | b'*' | b'%' | b':' | b',' => {
                // Operator - push now
                j = i + 1;
                tokens.push(Token::Operator(Operator::new(Text::from_slice(&input.0[i..j]))));
            },
            b'"' => {
                j = 1 + input.find_first(|x: &u8| *x == b'"', i+1);
                tokens.push(Token::Quoted(Quoted::new(Text::from_slice(&input.0[i+1..j-1]))));
            },
            b'`' => {
                j = input.find_first(|x: &u8| !x.is_ascii_alphabetic(), i+1);
                tokens.push(Token::Symbol(Symbol::new(Text::from_slice(&input.0[i+1..j]))));
            }
            b'(' => {
                j = i + 1;
                tokens.push(Token::LParen);
            },
            b')' => {
                j = i + 1;
                tokens.push(Token::RParen);
            }
            b' ' => {
                // Space - push now
                j = i + 1;
                // tokens.push(Token::Space(Space { text: input[i..j].to_vec() }));
            },
            _ => {
                // Other - ignore
                j = i + 1;
            }
        };
        i = j;
    }
    tokens
}