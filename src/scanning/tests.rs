use super::*;
use std::fmt::Debug;


#[test]
pub fn scanner_recognizes_single_character_tokens() {
    let corpus = "(){},.-+;/!*><";
    let tokens : Vec<TokenType> = tokenize(corpus)
        .iter()
        .map(|t| t.token_type)
        .collect();
    
    let expected = [
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::Minus,
        TokenType::Plus,
        TokenType::SemiColon,
        TokenType::Slash,
        TokenType::Bang,
        TokenType::Star,
        TokenType::Greater,
        TokenType::Less,
        TokenType::EndOfFile
    ];
    
    assert_slice_eq(tokens.as_slice(), &expected);
}

#[test]
fn scanner_recognizes_double_character_tokens() {
    let corpus = "< <= > >= ! != = ==";
    let tokens : Vec<TokenType> = tokenize(corpus)
        .iter()
        .map(|t| t.token_type)
        .collect();
    let expected = [
        TokenType::Less,
        TokenType::LessEqual,
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Bang,
        TokenType::BangEqual,
        TokenType::Equal,
        TokenType::EqualEqual,
        TokenType::EndOfFile
    ];
    
    assert_slice_eq(tokens.as_slice(), &expected);
}

#[test]
fn scanner_ignores_whitespace() {
    let corpus = " \t \n \r . ";
    let tokens : Vec<TokenType> = tokenize(corpus)
        .iter()
        .map(|t| t.token_type)
        .collect();
    let expected = [TokenType::Dot, TokenType::EndOfFile];
    
    assert_slice_eq(tokens.as_slice(), &expected);
}

#[test]
fn scanner_ignore_comments() {
    let corpus = "+//totally a % comment % with illegal characters\n.";
    let tokens : Vec<TokenType> = tokenize(corpus)
        .iter()
        .map(|t| t.token_type)
        .collect();
    let expected = [TokenType::Plus, TokenType::Dot, TokenType::EndOfFile];

    assert_slice_eq(tokens.as_slice(), &expected);
}

fn tokenize(corpus: &str) -> Vec<Token> {
    let mut chars = corpus.chars();
    let mut scanner = Scanner::create(&mut chars);
    let mut tokens : Vec<Token> = Vec::new();
    loop {
        let token = scanner.next();
        let token_type = token.token_type;
        tokens.push(token);
        
        if token_type == TokenType::EndOfFile {
            break;
        }
        
        if let TokenType::Error(_) = token_type {
            break;
        }
    };
    
    return tokens;
}

fn assert_slice_eq<T: Eq + Debug>(actual: &[T], expected: &[T]) {
    
    assert_eq!(actual.len(), expected.len());
    
    for i in 0..(actual.len()-1) {
        assert_eq!(actual[i], expected[i]);
    }
}

