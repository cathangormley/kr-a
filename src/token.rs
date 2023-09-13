use crate::text::Text;
use crate::kr::Kr;
use crate::operator::Operator;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Name(Text);

impl Name {
    pub fn new(text: Text) -> Self {
        Name(text)
    }
    fn parse(&self) -> Kr {
        Kr::NN(vec![Kr::Null, Kr::S(self.0.clone())])
    }
}

#[derive(Clone)]
pub struct Number(Text);

// Digits possibly preceding a char: 123j
impl Number {
    pub fn new(text: Text) -> Self {
        Number(text)
    }
    fn parse(&self) -> Kr {
        // input may be 123 or 123f or 123i etc..
        let input = self.0.to_string();
        let (num, letter) = if input.ends_with(|c: char| c.is_ascii_digit()) {
            // input = 123
            (&input[..], "j")
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
// Text is the characters between the quotes
#[derive(Clone)]
pub struct Quoted(Text);

impl Quoted {
    pub fn new(text: Text) -> Self {
        Quoted(text)
    }
    fn parse(&self) -> Kr {
        // TODO: there should be logic to handle "\t\netc.."
        Kr::Cv(self.0.0.clone())
    }
}

#[derive(Clone)]
pub struct Symbol(Text);

impl Symbol {
    pub fn new(text: Text) -> Self {
        Symbol(text)
    }
    fn parse(&self) -> Kr {
        Kr::S(self.0.clone())
    }
}


// TODO: Split this into tokens that represent kr values and grammar helpers?

#[derive(Clone)]
pub enum Token {
    Name(Name),
    Operator(Operator),
    Number(Number),
    Quoted(Quoted),
    Symbol(Symbol),
    LParen, RParen,     // ( )
    // LBracket, RBracket, // [ ]
    // LBrace, RBrace,     // { }
}

impl Token {
    pub fn as_string(&self) -> String {
        let lparen = Text::from_str("(");
        let rparen = Text::from_str(")");
        let t = match self {
            Token::Name(Name(text)) => text,
            Token::Operator(Operator { text, .. }) => text,
            Token::Number(Number(text)) => text,
            Token::Quoted(Quoted(text)) => text,
            Token::Symbol(Symbol(text)) => text,
            Token::LParen => { &lparen },
            Token::RParen => { &rparen },
            // Token::LBracket => &vec![b'['],
            // Token::RBracket => &vec![b']'],
            // Token::LBrace => &vec![b'{'],
            // Token::RBrace => &vec![b'}'],
        };
        t.to_string()
    }
    pub fn to_kr(&self) -> Kr {
        match self {
            Token::Name(name) => name.parse(),
            Token::Operator(op) => op.to_kr(),
            Token::Number(num) => num.parse(),
            Token::Quoted(s) => s.parse(),
            Token::Symbol(s) => s.parse(),
            _ => panic!("Failed to convert token to kr"),
        }
    }
}