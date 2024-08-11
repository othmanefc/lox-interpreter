use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    EOF,
    Unknown(String),
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
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '*' => TokenType::Star,
            '.' => TokenType::Dot,
            ',' => TokenType::Comma,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '-' => TokenType::Minus,
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
        println!("{} {} null", token.token_type, token.lexeme);
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
