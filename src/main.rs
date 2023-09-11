use std::io::{self, Write};
mod operator;
use operator::Operator;

mod kr;
use kr::Kr;

mod init;
use crate::init::Env;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Name {
    text: Vec<u8>,
}

impl Name {
    fn _new(text: Vec<u8>) -> Self {
        Name { text }
    }
    fn to_kr(&self) -> Kr {
        Kr::S(self.text.clone())
    }
}

#[derive(Clone)]
pub struct Number {
    text: Vec<u8>
}

// Digits possibly preceding a char: 123j
impl Number {
    fn new(text: Vec<u8>) -> Self {
        Number { text }
    }
    fn to_kr(&self) -> Kr {
        // input may be 123 or 123f or 123i etc..
        let input = std::str::from_utf8(&self.text).expect("Could not convert to utf8");

        let (num, letter) = if input.ends_with(|c: char| c.is_ascii_digit()) {
            // input = 123
            (input, "j")
        } else {
            // input 123i or 123j or ..
            input.split_at(input.len() - 1)
        };
        
        match (num, letter) {
            (num,"i") => Kr::I(num.parse().unwrap()),
            (num,"j") => Kr::J(num.parse().unwrap()),
            (num,"e") => Kr::E(num.parse().unwrap()),
            (num,"f") => Kr::F(num.parse().unwrap()),
            (_, _) => Kr::Null
        }
    }
}

// A string surrounded by quotes: "example"
#[derive(Clone)]
pub struct Quoted {
    text: Vec<u8>,
}

impl Quoted {
    fn to_kr(&self) -> Kr {
        if self.text.len() < 3 {
            // If there are less than 3 elements, return an empty Cv variant.
            Kr::Cv(Vec::new())
        } else {
            // Get a slice of the text excluding the first and last elements.
            Kr::Cv(self.text[1..self.text.len() - 1].to_vec())
        }
    }
}

// TODO: Split this into tokens that represent kr values and grammar helpers?

#[derive(Clone)]
pub enum Token {
    Name(Name),
    Operator(Operator),
    Number(Number),
    Quoted(Quoted),
    LParen, RParen,     // ( )
    // LBracket, RBracket, // [ ]
    // LBrace, RBrace,     // { }
}

impl Token {
    fn as_string(&self) -> String {
        let lparen = vec![b'('];
        let rparen = vec![b')'];
        let t = match self {
            Token::Name(Name { text }) => text,
            Token::Operator(Operator { text, .. }) => text,
            Token::Number(Number { text, .. }) => text,
            Token::Quoted(Quoted { text }) => text,
            Token::LParen => { &lparen },
            Token::RParen => { &rparen },
            // Token::LBracket => &vec![b'['],
            // Token::RBracket => &vec![b']'],
            // Token::LBrace => &vec![b'{'],
            // Token::RBrace => &vec![b'}'],
        };
        ascii_to_string(t)
    }
    fn to_kr(&self) -> Kr {
        match self {
            Token::Name(name) => name.to_kr(),
            Token::Operator(op) => op.to_kr(),
            Token::Number(num) => num.to_kr(),
            Token::Quoted(s) => s.to_kr(),
            _ => panic!("Failed to convert token to kr"),
        }
    }
}

fn ascii_to_string(input: &Vec<u8>) -> String {
    match std::str::from_utf8(input) {
        Ok(s) => s.to_string(),  // Conversion successful
        Err(_) => String::new(),       // Invalid UTF-8, return an empty string
    }
}

fn read() -> String {
    print!("kr>");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    let _res = io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

// Returns index of first element of v that satisfies some condition, ignores first n characters
fn find_first_index<T, F>(v: &Vec<T>, cond: F, n: usize) -> usize
where
    F: Fn(&T) -> bool,
{
    for (i, c) in v.iter().enumerate().skip(n) {
        if cond(c) { return i }
    }
    return v.len()
}

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


fn lex(input: &String) -> Vec<Token> {
    // input.split_whitespace().collect()
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0; // Index
    let input = input.as_bytes().to_owned();
    while i < input.len() {
        // Determine what type of token by looking at next character
        // Then possibly look ahead to find end of current token
        let c  = input[i];
        let j: usize;
        match c {
            b'a'..=b'z' | b'A'..=b'Z' => {
                // Name - must look ahead
                j = find_first_index(&input, |x: &u8| !x.is_ascii_alphabetic(), i);
                tokens.push(Token::Name(Name { text: input[i..j].to_vec() }));
            },
            b'0'..=b'9' => {
                // Number - must look ahead
                j = i + read_number(&input[i..]);
                // j = find_first_index(&input, |x: &u8| !x.is_ascii_digit(), i);
                // let ff = &input[i..];
                tokens.push(Token::Number(Number::new(input[i..j].to_vec())));
            },
            b'+' | b'-' | b'*' | b'%' | b':' | b',' => {
                // Operator - push now
                j = i + 1;
                tokens.push(Token::Operator(Operator::new(input[i..j].to_vec())));
            },
            b'"' => {
                j = 1 + find_first_index(&input, |x: &u8| *x == b'"', i+1);
                tokens.push(Token::Quoted(Quoted { text: input[i..j].to_vec() }));
            },
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

fn parse(tokens:&Vec<Token>) -> Kr {
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
        _ => {
            match tokens.get(1) {
                Some(Token::Operator(op)) => Kr::NN(vec![op.to_kr(), first_token.to_kr(), parse(&tokens[2..].to_vec())]),
                Some(_token) => Kr::NN(vec![first_token.to_kr(), parse(&tokens[1..].to_vec())]),
                None => first_token.to_kr(),
            }
        }


    }
}
/*
fn _eval(env: Env, ast: Kr) -> (Env, Kr) {
    // Recursively evaluate ast
    match ast {
        Kr::NN(t) => {
            match t.get(0) {
                Some(Kr::Op(op)) => {
                    if op.text == vec![b':'] {
                        let (env, res2) = eval(env, t[2].clone());
                        (op.dyadic)(env, &t[1], &res2)
                    } else {
                        let (env, res1) = eval(env, t[1].clone());
                        let (env, res2) = eval(env, t[2].clone());
                        (op.dyadic)(env, &res1, &res2)
                    }
                },
                Some(k) => {
                    let (env, res) = eval(env, k.clone());
                    (env, res)
                },
                _ => (env, Kr::NN(t))
            }

        },
        Kr::S(ref _t) => {
            let val = env.value(&ast).clone();
            (env,val)
            // (env, env.value(&Kr::S(t.to_vec())).clone())
        },
        other => (env, other),
    }
}
*/

fn eval(mut env: Env, ast: Kr) -> (Env, Kr) {
    // Recursively evaluate ast
    // A singleton list means the value assigned to that name
    match ast {
        Kr::NN(t) => {
            match t.len() {
                0 => (env, Kr::Null),
                1 => {
                    match &t[0] {
                        Kr::S(_name) => (env.clone(), env.val(&t[0])),
                        k => (env, k.clone()),
                    }
                },
                _ => {
                    // Initialize a vector to store the results
                    let mut results: Vec<Kr> = Vec::new();

                    // Iterate through the elements of t, starting from the second element (index 1)
                    for i in 1..t.len() {
                        let (new_env, result) = eval(env.clone(), t[i].clone());
                        // Append the result to the results vector
                        results.push(result);
                        // Update the environment for the next iteration
                        env = new_env;
                    }

                    // Return the final environment and the vector of results
                    match &t[0] {
                        Kr::Op(op) => { (op.dyadic)(env, results) },
                        _ => (env, t[0].clone())
                    }
                }
            }
        }
        other => (env, other),
    }
}

fn print(output: &Kr) {
    println!("{}", output.print());
}

fn main() {
    // Startup logic here..
    let mut env: Env = init::init();
    let debug = env.opts.iter().any(|s| s == "--debug");
    
    if debug { println!("Options {:?}", env.opts); };

    loop {
        // REPL loop
        let input = read();
        let tokens = lex(&input);
        if debug { 
            let token_strings: Vec<String> = tokens.iter().map(|x| x.as_string()).collect();
            println!("{:?}", token_strings);
        };
        let ast = parse(&tokens);
        if debug { println!("{:?}", ast); };

        let result: Kr;
        (env, result) = eval(env, ast);
        if debug { println!("{:?}", result); };

        print(&result);

        if input.len() == 0 { break; };
    }
}
