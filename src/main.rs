use std::io::{self, Write};

#[derive(Debug)]
struct Name {
    text: Vec<u8>,
}

#[derive(Debug)]
struct Operator {
    text: Vec<u8>,
}

#[derive(Debug)]
struct Space {
    text: Vec<u8>,
}

#[derive(Debug)]
struct Number {
    text: Vec<u8>,
}

#[derive(Debug)]
enum Token {
    Name(Name),
    Operator(Operator),
    Space(Space),
    Number(Number),
}

impl Token {
    fn characters(&self) -> String {
        match *self {
            Token::Name(_) => ('a'..='Z').collect::<String>(),
            Token::Operator(_) => "+-*/".to_string(),
            Token::Space(_) => " ".to_string(),
            Token::Number(_) => ('0'..='9').collect::<String>(),
        }
    } 
}

fn read() -> String {
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


fn tokenize(input: &String) -> Vec<Token> {
    // input.split_whitespace().collect()
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0; // Index
    let input = input.as_bytes().to_owned();
    while i < input.len() {
        let c  = input[i];
        i = match c {
            b'a'..=b'z' | b'A'..=b'Z' => {
                // Name - must look ahead
                let j = find_first_index(&input, |x: &u8| !x.is_ascii_alphabetic(), i);
                tokens.push(Token::Name(Name { text: input[i..j].to_vec() }));
                j
            },
            b'0'..=b'9' => {
                // Number - must look ahead
                let j = find_first_index(&input, |x: &u8| !x.is_ascii_digit(), i);
                tokens.push(Token::Number(Number { text: input[i..j].to_vec() }));
                j
            },
            b'+' | b'-' | b'*' | b'/' => {
                // Operator - push now
                let j = i + 1;
                tokens.push(Token::Operator(Operator { text: input[i..j].to_vec() }));
                j
            },
            b' ' => {
                // Space - push now
                let j = i + 1;
                tokens.push(Token::Space(Space { text: input[i..j].to_vec() }));
                j
            },
            _ => {
                // Other - ignore
                let j = i + 1;
                j
            }
        };
    }
    tokens
}

fn print(input: &String) -> usize {
    let linesize = input.len();
    println!("Input: '{}' with length: '{}'", input.trim().to_string(), linesize);
    linesize
}

fn main() {
    loop {
        print!("kr>");
        io::stdout().flush().expect("Failed to flush stdout");
        let input = read();
        let tokens = tokenize(&input);
        println!("{:?}", tokens);
        let res = print(&input);
        if res == 0 { break; };
    }
}
