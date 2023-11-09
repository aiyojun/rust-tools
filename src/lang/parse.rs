#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenKind {
    EOF,
    Identifier, Integer,
    LBrace, RBrace, LSquare, RSquare, LParen, RParen,
    Comma, Semi, Colon, Ellipsis, Whitespace,
    Comment,
    Equal, 
    KVar,
}

#[derive(Debug)]
pub struct Token {pub kind: TokenKind, text: String, offset: usize, length: usize, line: usize, line_start: usize}

impl Token {
    // pub fn new(kind: TokenKind, text: String, offset: u32, length: u32, line: u32, line_start: u32) -> Token 
    // { Token {kind, text, line, line_start, offset, length} }

    pub fn is(&self, kind: TokenKind) -> bool {self.kind == kind}

    pub fn eof() -> Token { Token { kind: TokenKind::EOF, text: String::new(), offset: 0, length: 0, line: 0, line_start: 0 } }
}


pub struct Lexer {
    text: String,
    offset: usize,
    state: u32,
}

impl Lexer {
    pub fn new(text: String) -> Lexer { Lexer { text, offset: 0, state: 0 } }
    
    pub fn get_char(&mut self) -> char {
        if self.offset >= self.text.len() { return 0 as char }
        let r = self.text.as_bytes()[self.offset];
        self.offset += 1;
        r as char
    }

    pub fn lex(&mut self) -> Token {
        let mut buf = String::new();
        let mut ch: char;
        let mut tok: Option<Token>;
        loop {
            ch = self.get_char();
            tok = None;
            match ch {
                '\0' => { return Token::eof(); }
                '\n' | '\t' | ' ' => {
                    match self.state {
                        0 => { self.state = 1; buf.push(ch); }
                        1 => { buf.push(ch); }
                        _ => { tok = self.build(buf); self.state = 1; buf = String::from(ch); }
                    }
                }
                ';' => {
                    match self.state {
                        0 => { self.state = 2; buf.push(ch); },
                        _ => { tok = self.build(buf); self.state = 2; buf = String::from(ch); }
                    }
                }
                '=' => {
                    match self.state {
                        0 => { self.state = 3; buf.push(ch); },
                        _ => { tok = self.build(buf); self.state = 3; buf = String::from(ch); }
                    }
                }
                'v' => {
                    match self.state {
                        0 => { self.state = 4; buf.push(ch); }
                        10 => { buf.push(ch) }
                        _ => { tok = self.build(buf); self.state = 51; buf = String::from(ch); }
                    }
                }
                'a' => {
                    match self.state {
                        4 => { self.state = 5; buf.push(ch); }
                        10 => { buf.push(ch); }
                        _ => { tok = self.build(buf); self.state = 10; buf = String::from(ch); }
                    }
                }
                'r' => {
                    match self.state {
                        5 => { self.state = 6; buf.push(ch); }
                        10 => { buf.push(ch); }
                        _ => { tok = self.build(buf); self.state = 6; buf = String::from(ch); }
                    }
                }
                '_' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 's' | 't' | 'u' | 'w' | 'x' | 'y' | 'z' => {
                    match self.state {
                        0 => { self.state = 10; buf.push(ch); }
                        10 => { buf.push(ch); }
                        _ => { tok = self.build(buf); self.state = 10; buf = String::from(ch); }
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    match self.state {
                        10 | 4 | 5 | 6 => { self.state = 10; buf.push(ch); }
                        0 | 9 => { self.state = 9; buf.push(ch) }
                        _ => { tok = self.build(buf); self.state = 9; buf = String::from(ch); }
                    }
                }
                _ => {}
            }
            if let Some(tk) = tok {
                if !tk.is(TokenKind::Whitespace) {
                    return tk;
                }
            }
        }
    }

    pub fn build(&self, text: String) -> Option<Token> {
        let length = text.len();
        let offset = self.offset - length - 1;
        match self.state {
            1  => Some(Token {kind: TokenKind::Whitespace, text, offset, length, ..Token::eof()}),
            2  => Some(Token {kind: TokenKind::Semi, text, offset, length, ..Token::eof()}),
            3  => Some(Token {kind: TokenKind::Equal, text, offset, length, ..Token::eof()}),
            6  => Some(Token {kind: TokenKind::KVar, text, offset, length, ..Token::eof()}),
            9  => Some(Token {kind: TokenKind::Integer, text, offset, length, ..Token::eof()}),
            10 => Some(Token {kind: TokenKind::Identifier, text, offset, length, ..Token::eof()}),
            _ => None,
        }
    }

    
}