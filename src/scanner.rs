enum TokenType {
    LeftParen,
    RightParen,
    EOF,
    Unknown(String),
}

impl TokenType {
    fn as_str(&self) -> &'static str {
        match self {
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::EOF => "EOF",
            _ => panic!("not supported token_type"),
        }
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    for char in line.chars() {
        let token_type = match char {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            _ => TokenType::Unknown(char.into()),
        };
        tokens.push(Token {
            lexeme: match token_type {
                TokenType::Unknown(_) => String::new(),
                _ => char.into(),
            },
            token_type,
        });
    }
    tokens
}

fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens.iter() {
        match token.token_type {
            TokenType::LeftParen | TokenType::RightParen | TokenType::EOF => {
                println!("{} {} null", token.token_type.as_str(), token.lexeme)
            }
            _ => panic!("Unknown token type"),
        }
    }

}
pub fn scanner(source: String) {
    let lines = source.lines();
    let mut tokens = Vec::<Token>::new();

    for line in lines {
        let mut line_tokens = tokenize(line);
        tokens.append(&mut line_tokens);
    }
    tokens.push(Token {
        lexeme: String::new(),
        token_type: TokenType::EOF,
    });
    print_tokens(&tokens)
}
