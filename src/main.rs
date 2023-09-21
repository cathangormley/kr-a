use std::io::{self, Write};
mod operator;

mod kr;
use error::KrEvalError;
use kr::Kr;

mod init;
use crate::error::KrError;
use crate::init::Env;

mod text;
mod token;
use crate::token::Token;
mod lex;
mod parse;
mod error;
mod primitive;


fn read() -> String {
    print!("kr>");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    let _res = io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn eval(env: Env, ast: &Kr) -> (Env, Result<Kr, KrEvalError>) {
    // Recursively evaluate ast
    // A list of the form [(::); `name] means the value assigned to that name
    match ast {
        Kr::NN(t) => {
            match t.len() {
                0 => (env, Ok(Kr::Null)),
                1 => (env, Ok(t[0].clone())),
                _ => {
                    // Initialize a vector to store the results
                    let mut results: Vec<Kr> = Vec::new();
                    let mut e: Env = env;
                    // Iterate through the elements of t, starting from the second element (index 1)
                    for i in 0..t.len() {
                        let (new_env, kr) = eval(e, &t[i]);
                        let kr = match kr {
                            Ok(x) => x,
                            Err(err) => return (new_env, Err(err)),
                        };
                        // Append the result to the results vector
                        results.push(kr);
                        // Update the environment for the next iteration
                        e = new_env;
                    }
                    let (first, rest) = results.split_first().expect("results should not be empty");
                    first.apply(e, rest)                   
                }
            }
        }
        other => (env, Ok(other.clone())),
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
            Err(e) => {
                KrError::Parse(e).print();
                continue;
            },
            Ok(ast) => ast,
        };

        if debug { println!("{}",ast.print()); };

        let result: Result<Kr, KrEvalError>;
        (env, result) = eval(env, &ast);
        let result = match result {
            Err(e) => {
                KrError::Eval(e).print();
                continue;
            },
            Ok(res) => res,
        };
        if debug { println!("{:?}", result); };

        print(&result);

        if input.len() == 0 { break; };
    }
}
