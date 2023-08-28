use std::io::{self, Write};

mod operator;
use operator::Operator;

mod kr;
use kr::Kr;

mod init;
use crate::init::Env;

#[derive(Eq, Hash, PartialEq)]
pub struct Name {
    text: Vec<u8>,
}

pub struct Number {
    text: Vec<u8>,
    value: Kr,
}

impl Number {
    fn new(input: &Vec<u8>) -> Self {
        // input may be 123 or 123f or 123i etc..
        let input = std::str::from_utf8(input).expect("Could not convert to utf8");

        let (num, letter) = if input.ends_with(|c: char| c.is_ascii_digit()) {
            // input = 123
            (input, "j")
        } else {
            // input 123i or 123j or ..
            input.split_at(input.len() - 1)
        };

        
        let value = match (num, letter) {
            (num,"i") => Kr::I(num.parse().unwrap()),
            (num,"j") => Kr::J(num.parse().unwrap()),
            (num,"e") => Kr::E(num.parse().unwrap()),
            (num,"f") => Kr::F(num.parse().unwrap()),
            (_, _) => Kr::Null
        };

        Number { text: num.into(), value }
    }
}
pub enum Token {
    Name(Name),
    Operator(Operator),
    Number(Number),
}

impl Token {
    fn as_string(&self) -> String {
        let t = match self {
            Token::Name(Name { text }) => { text },
            Token::Operator(Operator { text, .. }) => { text },
            Token::Number(Number { text, .. }) => { text },
        };
        ascii_to_string(t)
    }
    fn to_kr(&self) -> Kr {
        match self {
            Token::Name(n) => { Kr::Cv(n.text.clone()) },
            Token::Operator(op) => { Kr::Op(op.clone())},
            Token::Number(num) => {num.value.clone()},
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
    let mut end = false;
    for (ind, &c) in input.iter().enumerate() {
        end = match c {
            b'0'..=b'9' => { i = i + 1; false },
            b'e' | b'f' | b'i' | b'j' => { i = i + 1; true },
            _ => true,
        };
        if end { break; }
    }
    i
}


fn tokenize(input: &String) -> Vec<Token> {
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
                tokens.push(Token::Number(Number::new(&input[i..j].to_vec())));
            },
            b'+' | b'-' | b'*' | b'%' | b':' | b',' => {
                // Operator - push now
                j = i + 1;
                tokens.push(Token::Operator(Operator::new(input[i..j].to_vec())));
            },
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

fn parse(tokens:&Vec<Token>) -> Vec<Kr> {
    // For now we return a list of Kr variables: [f, arg1, arg2, ..]
    // Later, it will be the entire AST
    let krv: Vec<Kr> = tokens.iter().map(|t| t.to_kr()).collect();

    if krv.len() < 3 { return krv; }

    // Otherwise krv.len() >= 3 ..
    let abc = &krv[krv.len() - 3..];

    match abc {
        [a, Kr::Op(b), c] => vec![Kr::Op(b.clone()),a.clone(),c.clone()],
        p => {
            println!("Could not evaluate pattern");
            p.to_vec()
        }
    }
}

fn eval(env: &mut Env, ast: Vec<Kr>) -> Kr {
    
    match &ast[..] {
        [Kr::Op(f), a, b] => (f.dyadic)(env, &a, &b),
        _ => Kr::J(0),
    }

}


fn print(input: &String) -> usize {
    let linesize = input.len();
    println!("Input: '{}' with length: '{}'", input.trim().to_string(), linesize);
    linesize
}

fn main() {
    // Startup logic here..
    let mut env: Env = init::init();

    loop {
        // REPL loop
        let input = read();
        let tokens = tokenize(&input);
        let token_strings: Vec<String> = tokens.iter().map(|x| x.as_string()).collect();
        println!("{:?}", token_strings);
        let res = print(&input);

        let ast = parse(&tokens);

        let evalres = eval(&mut env, ast);
        println!("{:?}", evalres);

        if res == 0 { break; };
    }
}
