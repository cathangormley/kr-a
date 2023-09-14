use std::io::{self, Write};
mod operator;

mod kr;
use kr::Kr;

mod init;
use crate::init::Env;

mod text;
mod token;
use crate::token::Token;
mod lex;
mod parse;
mod error;


fn read() -> String {
    print!("kr>");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    let _res = io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn eval(mut env: Env, ast: Kr) -> (Env, Kr) {
    // Recursively evaluate ast
    // A list of the form [(::); `name] means the value assigned to that name
    match ast {
        Kr::NN(t) => {
            match t.len() {
                0 => (env, Kr::Null),
                1 => (env, t[0].clone()),
                _ => {
                    // Initialize a vector to store the results
                    let mut results: Vec<Kr> = Vec::new();
 
                    // Iterate through the elements of t, starting from the second element (index 1)
                    for i in 0..t.len() {
                        let (new_env, result) = eval(env, t[i].clone());
                        // Append the result to the results vector
                        results.push(result);
                        // Update the environment for the next iteration
                        env = new_env;
                    }
                    let (first, rest) = results.split_first().unwrap();
                    first.apply(env, Vec::from(rest))                   
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
        let tokens: Vec<Token> = lex::lex(&input);
        if debug { 
            let token_strings: Vec<String> = tokens.iter().map(|x| x.as_string()).collect();
            println!("{:?}", token_strings);
        };
        let ast = match parse::parse(&tokens) {
            Err(_) => continue,
            Ok(ast) => ast,
        };

        if debug { println!("{:?}", ast); };

        let result: Kr;
        (env, result) = eval(env, ast);
        if debug { println!("{:?}", result); };

        print(&result);

        if input.len() == 0 { break; };
    }
}
