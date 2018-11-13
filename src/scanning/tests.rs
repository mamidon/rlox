use super::*;
use std::fmt::Debug;


#[test]
pub fn scanner_recognizes_single_character_tokens() {
    let corpus = "(){},.-+;/!*><";
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

    test_scanner(corpus, &expected);
}

#[test]
fn scanner_recognizes_double_character_tokens() {
    let corpus = "< <= > >= ! != = ==";
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

    test_scanner(corpus, &expected);
}

#[test]
fn scanner_ignores_whitespace() {
    let corpus = " \t \n \r . ";
    let expected = [TokenType::Dot, TokenType::EndOfFile];

    test_scanner(corpus, &expected);
}

#[test]
fn scanner_ignore_comments() {
    let corpus = "+//totally a % comment % with illegal characters\n.";
    let expected = [TokenType::Plus, TokenType::Dot, TokenType::EndOfFile];

    test_scanner(corpus, &expected);
}

fn test_scanner(corpus: &str, expected_tokens: &[TokenType]) {
    let actual_tokens: Vec<TokenType> = tokenize(corpus)
        .iter()
        .map(|t| t.token_type)
        .collect();
    
    assert_slice_eq(actual_tokens.as_slice(), &expected_tokens);
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
    use std::cmp::min;
    let safe_len = min(actual.len(), expected.len()) - 1;
    
    for i in 0..safe_len {
        println!("actual[{}]={:?}\texpected[{}]={:?}", i, actual[i], i, expected[i]);
        assert_eq!(actual[i], expected[i]);
    }

    assert_eq!(actual.len(), expected.len());
}

