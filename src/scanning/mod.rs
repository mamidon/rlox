use std::str::Chars;
mod seq;
mod tests;

pub struct Scanner<'a> {
    line_number: usize,
    lexeme_start: usize,
    current_character: usize,
    consumed_characters: Vec<char>,
    source: seq::CharacterSequence<'a, char>
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    token_type: TokenType,
    line_number: usize,
    lexeme_start: usize,
    lexeme: String
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    SemiColon, Slash, Star,
    
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    Identifier, String, Number,
    
    And, Class, Else, False,
    Fun, For, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,
    
    Error(&'static str),
    EndOfFile
}

impl<'a> Scanner<'a> {
    pub fn create(input: &'a mut Chars) -> Scanner<'a> {
        Scanner {
            line_number: 1,
            lexeme_start: 0,
            current_character: 0,
            consumed_characters: Vec::new(),
            source: seq::CharacterSequence::new(input)
        }
    }
    
    pub fn next(&mut self) -> Token {
        self.skip_whitespace();
        
        self.lexeme_start = self.current_character;
        self.consumed_characters.clear();
        
        let next_character = self.consume();
        
        if let None = next_character {
            return self.create_token(TokenType::EndOfFile);
        }
        
        let token_type = match next_character.unwrap() {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus, 
            ';' => TokenType::SemiColon, 
            '/' => TokenType::Slash,
            '*' => TokenType::Star,
            '!' => match self.match_next('=') {
                Some(_) => TokenType::BangEqual,
                None => TokenType::Bang
            },
            '=' => match self.match_next('=') {
                Some(_) => TokenType::EqualEqual,
                None => TokenType::Equal
            },
            '>' => match self.match_next('=') {
                Some(_) => TokenType::GreaterEqual,
                None => TokenType::Greater
            },
            '<' => match self.match_next('=') {
                Some(_) => TokenType::LessEqual,
                None => TokenType::Less
            },
            _ => TokenType::Error("Unexpected character")
        };
        
        return self.create_token(token_type);
    }
    
    fn create_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line_number: self.line_number,
            lexeme_start: self.lexeme_start,
            lexeme: self.consumed_characters.iter().collect()
        }
    }
    
    fn skip_whitespace(&mut self) {
        loop {
            match self.source.peek() {
                Some(' ') => {self.consume();},
                Some('\r') => {self.consume();},
                Some('\t') => {self.consume();},
                Some('\n') => {
                    self.line_number += 1;
                    self.consume();
                },
                Some('/') => {
                    if let Some(_) = self.match_next('/') {
                        while let Some(c) = self.consume() {
                            if c == '\n' {
                                break;
                            }
                        }
                    }
                }
                Some(_) => break,
                None => break
            };
        }
    }
    
    fn match_next(&mut self, expected: char) -> Option<char> {
        if let Some(c) = self.source.peek() {
            if c == expected {
                return self.consume();
            }
        }
        
        return None;
    }
    
    fn consume(&mut self) -> Option<char> {
        let character = self.source.next();
        
        if let Some(c) = character {
            self.current_character += 1;
            self.consumed_characters.push(c);
        }
        
        return character;
    }
}
