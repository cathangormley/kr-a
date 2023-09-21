
pub enum KrError {
    Parse(KrParseError),
    Eval(KrEvalError),
}

pub enum KrParseError {
    UnexpectedRParen,
    UnexpectedEOF,
    MissingRParen,
    IncompleteParse,
    UnexpectedRBracket,
    MissingRBracket,
    UnexpectedSemiColon,
}

impl DisplayError for KrParseError {
    fn msg(&self) -> &str {
        use KrParseError as E;
        match self {
            E::IncompleteParse => "finished parse unexpectedly",
            E::MissingRParen => "missing )",
            E::UnexpectedEOF => "unexpected eof",
            E::UnexpectedRParen => "unexpected )",
            E::UnexpectedRBracket => "unexpected ]",
            E::MissingRBracket => "missing ]",
            E::UnexpectedSemiColon => "unexpected ;",
        }
    }
    fn code(&self) -> usize {
        use KrParseError as E;
        match self {
            E::IncompleteParse => 101,
            E::MissingRParen => 102,
            E::UnexpectedEOF => 103,
            E::UnexpectedRParen => 104,
            E::UnexpectedRBracket => 105,
            E::MissingRBracket => 106,
            E::UnexpectedSemiColon => 107,
        }
    }
}

pub enum KrEvalError {
    Type,
    NotAVerb,
    Rank,
    Length,
    Assign,
    NotDefined,
}

impl DisplayError for KrEvalError {
    fn msg(&self) -> &str {
        use KrEvalError as E;
        match self {
            E::Type => "type",
            E::NotAVerb => "not a verb",
            E::Rank => "rank",
            E::Length => "length",
            E::Assign => "assign",
            E::NotDefined => "not defined",
        }
    }
    fn code(&self) -> usize {
        use KrEvalError as E;
        match self {
            E::Type => 201,
            E::NotAVerb => 202,
            E::Rank => 203,
            E::Length => 204,
            E::Assign => 205,
            E::NotDefined => 206,
        }
    }
}

trait DisplayError {
    fn msg(&self) -> &str;
    fn code(&self) -> usize;
    fn display(&self) -> String {
        "'E".to_string() + &format!("{:0>4}", self.code()) + ": " + self.msg()
        
    }
}

impl KrError {
    pub fn print(&self) {
        match self {
            KrError::Parse(e) => println!("{}", e.display()),
            KrError::Eval(e) => println!("{}", e.display()),
        }
    }
}