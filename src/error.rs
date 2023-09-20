
pub enum KrError {
    Parse(KrParseError),
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
        }
    }
}