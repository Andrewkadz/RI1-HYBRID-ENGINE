#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Symbol(char),
    Arrow,       // →
    Plus,        // +
    Colon,       // :
    Slash,       // /
    Pipe,        // |
    LBracket,    // [
    RBracket,    // ]
    Equals,      // =
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub idx: usize,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for (i, ch) in input.chars().enumerate() {
        let kind = match ch {
            '→' => TokenKind::Arrow,
            '+' => TokenKind::Plus,
            ':' => TokenKind::Colon,
            '/' => TokenKind::Slash,
            '|' => TokenKind::Pipe,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '=' => TokenKind::Equals,
            other => TokenKind::Symbol(other),
        };
        tokens.push(Token { kind, idx: i });
    }
    tokens
}
