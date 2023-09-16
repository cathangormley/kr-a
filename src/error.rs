
pub enum KrError {
    Parse(KrParseError),
}

pub enum KrParseError {
    UnexpectedRParen,
    UnexpectedEOF,
    MissingRParen,
    IncompleteParse
}

impl DisplayError for KrParseError {
    fn msg(&self) -> &str {
        match self {
            KrParseError::IncompleteParse => "finished parse unexpectedly",
            KrParseError::MissingRParen => "missing )",
            KrParseError::UnexpectedEOF => "unexpected eof",
            KrParseError::UnexpectedRParen => "unexpected )",
        }
    }
    fn code(&self) -> usize {
        match self {
            KrParseError::IncompleteParse => 101,
            KrParseError::MissingRParen => 102,
            KrParseError::UnexpectedEOF => 103,
            KrParseError::UnexpectedRParen => 104,   
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