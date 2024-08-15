use phf::phf_map;
use strum_macros::Display;

#[derive(Display, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Operator {
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
}

#[derive(Display, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Display, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    RightParen,
    LeftParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Equal,
    #[strum(to_string = "{op}")]
    Operator {
        op: Operator,
    },
    Bang,
    Less,
    Greater,
    EOF,
    Unknown(String),
    Comment,
    Slash,
    // Blank,
    String {
        string: String,
        finished: bool,
    },
    Number(String),
    Identifier(String),
    #[strum(to_string = "{kw}")]
    Keyword {
        kw: Keyword,
        val: String,
    },
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "and" => Keyword::And,
    "class" => Keyword::Class,
    "else" => Keyword::Else,
    "false" => Keyword::False,
    "for" => Keyword::For,
    "fun" =>  Keyword::Fun,
    "if" =>  Keyword::If,
    "nil" =>  Keyword::Nil,
    "or" =>  Keyword::Or,
    "print" =>  Keyword::Print,
    "return" =>  Keyword::Return,
    "super" =>  Keyword::Super,
    "this" =>  Keyword::This,
    "true" =>  Keyword::True,
    "var" =>  Keyword::Var,
    "while" => Keyword::While,
};
