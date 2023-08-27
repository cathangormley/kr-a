use std::{io::{self, Write}, collections::HashMap};

mod operator;
use kr::Kr;
use operator::Operator;
mod kr;

#[derive(Eq, Hash, PartialEq)]
struct Name {
    text: Vec<u8>,
}

struct Space {
    text: Vec<u8>,
}

struct Number {
    text: Vec<u8>,
    value: Kr,
}

impl Number {
    fn new(t: Vec<u8>) -> Self {
        let v = match ascii_to_string(&t).parse::<i64>() {
            Ok(v) => v,
            Err(_) => 0,
        };
        Number { text: t, value: Kr::J(v)}
    }
}
enum Token {
    Name(Name),
    Operator(Operator),
    Space(Space),
    Number(Number),
}

impl Token {
    fn _characters(&self) -> String {
        match *self {
            Token::Name(_) => ('a'..='Z').collect::<String>(),
            Token::Operator(_) => "+-*/".to_string(),
            Token::Space(_) => " ".to_string(),
            Token::Number(_) => ('0'..='9').collect::<String>(),
        }
    }
    fn as_string(&self) -> String {
        let t = match self {
            Token::Name(Name { text }) => { text },
            Token::Operator(Operator { text, .. }) => { text },
            Token::Number(Number { text, .. }) => { text },
            Token::Space(Space { text }) => { text },
        };
        ascii_to_string(t)
    }
    fn to_kr(&self, env: &HashMap<Name, Kr> ) -> Kr {
        match self {
            Token::Name(n) => {env.get(n).unwrap_or(&Kr::J(0)).clone()},
            Token::Operator(op) => { Kr::Op(op.clone())},
            Token::Number(num) => {num.value.clone()}
            _ => {Kr::J(0)}
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
        // Determine what type of token by looking at next character
        // Then possibly look ahead to find end of current token
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
                tokens.push(Token::Number(Number::new(input[i..j].to_vec())));
                j
            },
            b'+' | b'-' | b'*' | b'/' => {
                // Operator - push now
                let j = i + 1;
                tokens.push(Token::Operator(Operator::new(input[i..j].to_vec())));
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


fn eval(env: &mut HashMap<Name, Kr>, tokens:&Vec<Token>) -> Kr {
    // For now env is a hashmap of names to Kr variables
    // Later it can become a kr_tree

    // For each token:
    // If number -> Kr(J)
    // If name -> check env

    let krs: Vec<Kr> = tokens.iter().map(|t| t.to_kr(env)).collect();

    if krs.len() < 3 {
        return Kr::J(0)
    };

    let abc = &krs[krs.len() - 3..];

    match abc {
        [a, Kr::Op(b), c] => (b.dyadic)(a,c),
        _ => {
            println!("Could not evaluate pattern");
            Kr::J(0)
        }
    }
}

fn print(input: &String) -> usize {
    let linesize = input.len();
    println!("Input: '{}' with length: '{}'", input.trim().to_string(), linesize);
    linesize
}

fn main() {
    // Startup logic here..
    let mut env: HashMap<Name, Kr> = HashMap::new();
    env.insert(Name { text:"one".to_string().into_bytes() }, Kr::J(1));
    env.insert(Name { text:"two".to_string().into_bytes() }, Kr::J(2));

    loop {
        // REPL loop
        print!("kr>");
        io::stdout().flush().expect("Failed to flush stdout");
        let input = read();
        let tokens = tokenize(&input);
        let token_strings: Vec<String> = tokens.iter().map(|x| x.as_string()).collect();
        println!("{:?}", token_strings);
        let res = print(&input);

        let evalres = eval(&mut env, &tokens);
        println!("{:?}", evalres);

        if res == 0 { break; };
    }
}
