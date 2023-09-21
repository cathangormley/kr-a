use crate::primitive::{Primitive, Prim};
use crate::text::Text;
use crate::kr::Kr;
use crate::operator::{Operator, Op};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct NameToken(Text);

impl NameToken {
    pub fn new(text: Text) -> Self {
        NameToken(text)
    }
    fn parse(&self) -> Vec<Kr> {
        vec![Kr::Prim(Primitive::new(Prim::Value)), Kr::S(self.0.clone())]
    }
}

#[derive(Clone, Debug)]
pub struct NumberToken(Text);

// Digits possibly preceding a char: 123j
impl NumberToken {
    pub fn new(text: Text) -> Self {
        NumberToken(text)
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
#[derive(Clone, Debug)]
pub struct QuotedToken(Text);

impl QuotedToken {
    pub fn new(text: Text) -> Self {
        QuotedToken(text)
    }
    fn parse(&self) -> Vec<u8> {
        // TODO: there should be logic to handle "\t\netc.."
        self.0.0.clone()
    }
}

#[derive(Clone, Debug)]
pub struct SymbolToken(Text);

impl SymbolToken {
    pub fn new(text: Text) -> Self {
        SymbolToken(text)
    }
    fn parse(&self) -> Text {
        self.0.clone()
    }
}

#[derive(Clone, Debug)]
pub struct OperatorToken {
    text: Text,
    op: Op
}


impl OperatorToken {
    pub fn new(text: Text) -> Self {
        let op =match text.0[..] {
            [b'+'] => Op::Addition,
            [b'-'] => Op::Subtraction,
            [b'*'] => Op::Multiplication,
            [b'%'] => Op::Division,
            [b':'] => Op::Assign,
            [b','] => Op::Join,
            // [b"**"] => OperatorToken::Power,
            _ => panic!("Unexpected token")
        };
        OperatorToken { text, op }
    }
    pub fn parse(&self) -> Operator {
        Operator::new(self.op)
    }
}

// Tokens that represent some Kr data
#[derive(Clone, Debug)]
pub enum KrToken {
    Name(NameToken),
    Operator(OperatorToken),
    Number(NumberToken),
    Quoted(QuotedToken),
    Symbol(SymbolToken),
}

impl KrToken {
    pub fn parse(&self) -> Kr {
        match self {
            KrToken::Name(name) => Kr::NN(name.parse()),
            KrToken::Operator(op) => Kr::Op(op.parse()),
            KrToken::Number(num) => num.parse(),
            KrToken::Quoted(s) => Kr::Cv(s.parse()),
            KrToken::Symbol(s) => Kr::S(s.parse()),
        }
    }
    fn get_text(&self) -> &Text {
        match self {
            KrToken::Name(name) => &name.0,
            KrToken::Operator(op) => &op.text,
            KrToken::Number(num) => &num.0,
            KrToken::Quoted(s) => &s.0,
            KrToken::Symbol(s) => &s.0,
        }
    }
    fn as_string(&self) -> String {
        self.get_text().to_string()
    }
}


#[derive(Clone, Debug)]
pub enum Token {
    KrToken(KrToken),
    LParen, RParen,         // ( )
    LBracket, RBracket,     // [ ]
    // LBrace, RBrace,      // { }
    SemiColon,
}

impl Token {
    pub fn as_string(&self) -> String {
        match self {
            Token::KrToken(t) => t.as_string(),
            Token::LParen => { "(".to_string() },
            Token::RParen => { ")".to_string() },
            Token::LBracket => { "[".to_string() },
            Token::RBracket => { "]".to_string() },
            Token::SemiColon => { ";".to_string() },
        }
    }
}