use crate::compiler::str_store::{MStr, StrStore};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ty {
    Identifier,
    Int,
    Float,
    TrueKeyword,
    FalseKeyword,
    FnKeyword,
    IfKeyword,
    LoopKeyword,
    BreakKeyword,
    TypeKeyword,
    ModKeyword,
    UseKeyword,
    InKeyword,
    ReturnKeyword,
    Comma,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    RangeOperator,
    ModuleOperator,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    EqualEqual,
    LessThan,
    Semicolon,
    Eof,
    Unknown,
}

#[derive(Clone, Copy)]
pub struct Pos(u32);

impl From<usize> for Pos {
    fn from(v: usize) -> Self {
        Pos(v as u32)
    }
}

#[derive(Clone, Copy)]
pub struct Token {
    pub ty: Ty,
    pub pos: Pos,
    pub lexeme: MStr,
}

impl Token {
    pub fn new_open_paren(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::OpenParen,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("("),
        }
    }

    pub fn new_close_paren(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::CloseParen,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr(")"),
        }
    }

    pub fn new_open_brace(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::OpenBrace,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("{"),
        }
    }

    pub fn new_close_brace(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::CloseBrace,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("}"),
        }
    }

    pub fn new_range(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::RangeOperator,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr(".."),
        }
    }

    pub fn new_module(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::ModuleOperator,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("::"),
        }
    }

    pub fn new_plus(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::Plus,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("+"),
        }
    }

    pub fn new_minus(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::Minus,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("-"),
        }
    }

    pub fn new_multiply(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::Multiply,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("*"),
        }
    }

    pub fn new_divide(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::Divide,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("/"),
        }
    }

    pub fn new_equal(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::Equal,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("="),
        }
    }

    pub fn new_equal_equal(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::EqualEqual,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("=="),
        }
    }

    pub fn new_less_than(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::LessThan,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr("<"),
        }
    }

    pub fn new_semicolon(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::Semicolon,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr(";"),
        }
    }

    pub fn new_comma(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::Comma,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr(","),
        }
    }

    pub fn new_eof(str_store: &mut StrStore, pos: usize) -> Self {
        Self {
            ty: Ty::Eof,
            pos: Pos(pos as u32),
            lexeme: str_store.get_mstr(""),
        }
    }
}

pub struct Lexer<'s> {
    source: &'s str,
    pos: usize,
    is_at_eos: bool,
    next: Token,
}

impl<'s> Lexer<'s> {
    pub fn new(str_store: &mut StrStore, source: &'s str) -> Self {
        let mut lexer = Self {
            source,
            pos: 0,
            is_at_eos: false,
            next: Token::new_eof(str_store, 0),
        };

        // populate the first token so the lexer is ready to go
        lexer.next(str_store);

        lexer
    }

    /// if the lexer is in a bad spot, this will just eat tokens till we find the next decl
    pub fn recover_until_decl(&mut self, str_store: &mut StrStore) {
        loop {
            let token = self.next(str_store);
            if [
                Ty::FnKeyword,
                Ty::UseKeyword,
                Ty::ModKeyword,
                Ty::TypeKeyword,
            ]
            .contains(&token.ty)
            {
                break;
            }
        }
    }

    /// if the lexer is in a bad spot, this will just eat tokens till we find the next expression
    pub fn recover_until_expr(&mut self, str_store: &mut StrStore) {
        loop {
            let token = self.next(str_store);
            if [Ty::Semicolon, Ty::CloseBrace].contains(&token.ty) {
                self.next(str_store);
                break;
            }
        }
    }

    pub fn peek(&self) -> &Token {
        &self.next
    }

    pub fn next(&mut self, str_store: &mut StrStore) -> Token {
        self.is_at_eos = false;
        let token = self.next;

        let next = self.lex_token(str_store);
        self.next = next;

        if self.can_end_statement(token.ty) {
            self.is_at_eos = true;
        }

        token
    }

    fn lex_token(&mut self, str_store: &mut StrStore) -> Token {
        // skip whitespace and comments
        self.skip();

        let ch = match self.next_char() {
            Some(ch) => ch,
            None => return Token::new_eof(str_store, self.pos),
        };

        if self.insert_semicolon(ch) {
            return Token::new_semicolon(str_store, self.pos);
        }

        match ch {
            'a'..='z' => self.lex_ident(str_store, ch),
            'A'..='Z' => self.lex_ident(str_store, ch),
            '0'..='9' => self.lex_number(str_store, ch),
            '(' => Token::new_open_paren(str_store, self.bump(ch)),
            ')' => Token::new_close_paren(str_store, self.bump(ch)),
            '{' => Token::new_open_brace(str_store, self.bump(ch)),
            '}' => Token::new_close_brace(str_store, self.bump(ch)),
            '+' => Token::new_plus(str_store, self.bump(ch)),
            '-' => Token::new_minus(str_store, self.bump(ch)),
            '*' => Token::new_multiply(str_store, self.bump(ch)),
            '/' => Token::new_divide(str_store, self.bump(ch)),
            '<' => Token::new_less_than(str_store, self.bump(ch)),
            ';' => Token::new_semicolon(str_store, self.bump(ch)),
            ',' => Token::new_comma(str_store, self.bump(ch)),
            '=' => match self.nth_char(1) {
                Some('=') => {
                    self.bump('=');
                    self.bump('=');
                    Token::new_equal_equal(str_store, self.pos)
                }
                _ => Token::new_equal(str_store, self.bump(ch)),
            },
            '.' => match self.nth_char(1) {
                Some('.') => {
                    self.bump('.');
                    self.bump('.');
                    Token::new_range(str_store, self.pos)
                }
                _ => self.lex_unknown(str_store, ch),
            },
            ':' => match self.nth_char(1) {
                Some(':') => {
                    self.bump(':');
                    self.bump(':');
                    Token::new_module(str_store, self.pos)
                }
                _ => self.lex_unknown(str_store, ch),
            },
            _ => self.lex_unknown(str_store, ch),
        }
    }

    fn lex_unknown(&mut self, str_store: &mut StrStore, ch: char) -> Token {
        let start = self.pos;
        self.bump(ch);

        while let Some(ch) = self.next_char() {
            // lex untill we find something that could resonably start a new token
            if ch.is_whitespace() {
                break;
            }

            if matches!(
                ch,
                '(' | ')' | '{' | '}' | '+' | '-' | '*' | '/' | '=' | '<' | ';' | '0'..='9' | 'a'..='z' | 'A'..='Z'
            ) {
                break;
            }

            self.bump(ch);
        }

        let s = &self.source[start..self.pos];
        let lexeme = str_store.get_mstr(s);

        Token {
            ty: Ty::Unknown,
            pos: Pos::from(start),
            lexeme,
        }
    }

    fn lex_ident(&mut self, str_store: &mut StrStore, ch: char) -> Token {
        let start = self.pos;
        self.bump(ch);

        while let Some(ch) = self.next_char() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }

            self.bump(ch);
        }

        let s = &self.source[start..self.pos];
        self.ident_to_keyword(Pos::from(start), str_store, s)
    }

    fn ident_to_keyword(&self, pos: Pos, str_store: &mut StrStore, s: &str) -> Token {
        let ty = match s {
            "fn" => Ty::FnKeyword,
            "true" => Ty::TrueKeyword,
            "false" => Ty::FalseKeyword,
            "if" => Ty::IfKeyword,
            "loop" => Ty::LoopKeyword,
            "break" => Ty::BreakKeyword,
            "type" => Ty::TypeKeyword,
            "in" => Ty::InKeyword,
            "mod" => Ty::ModKeyword,
            "use" => Ty::UseKeyword,
            "return" => Ty::ReturnKeyword,
            _ => Ty::Identifier,
        };

        let lexeme = str_store.get_mstr(s);
        Token { ty, pos, lexeme }
    }

    fn lex_number(&mut self, str_store: &mut StrStore, ch: char) -> Token {
        let start = self.pos;
        self.bump(ch);

        let mut ty = Ty::Int;

        while let Some(ch) = self.next_char() {
            if ch == '.' && ty == Ty::Int {
                // transition to float and consume the '.'
                ty = Ty::Float;
                self.bump(ch);
                continue;
            }

            if ch == '.' && ty == Ty::Float {
                // second '.', stop and let the lexer handle it as a range or unknown
                break;
            }

            if !ch.is_numeric() && ch != '_' {
                break;
            }

            self.bump(ch);
        }

        let s = &self.source[start..self.pos];
        let lexeme = str_store.get_mstr(s);

        Token {
            ty,
            pos: Pos::from(start),
            lexeme,
        }
    }

    fn insert_semicolon(&mut self, ch: char) -> bool {
        if !self.is_at_eos {
            return false;
        }

        match ch {
            '\n' => {
                self.bump(ch);
                return true;
            }
            '}' => return true,
            _ => return false,
        }
    }

    fn skip(&mut self) {
        loop {
            match self.next_char() {
                Some(ch) => {
                    if ch == '\n' && self.is_at_eos {
                        break;
                    }

                    if ch.is_whitespace() {
                        self.bump(ch);
                    }

                    if self.source[self.pos..self.pos + 1] == *"//" {
                        self.skip_comment()
                    }
                }
                None => break,
            }
        }
    }

    fn skip_comment(&mut self) {
        loop {
            match self.next_char() {
                Some(ch) => {
                    if ch == '\n' {
                        break;
                    }

                    self.bump(ch);
                }
                None => break,
            }
        }
    }

    fn bump(&mut self, ch: char) -> usize {
        let adv = ch.len_utf8();
        self.pos += adv;
        self.pos
    }

    fn next_char(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn nth_char(&self, n: usize) -> Option<char> {
        self.source[self.pos..].chars().nth(n)
    }

    fn can_end_statement(&mut self, ty: Ty) -> bool {
        matches!(
            ty,
            Ty::Identifier
                | Ty::Int
                | Ty::Float
                | Ty::TrueKeyword
                | Ty::FalseKeyword
                | Ty::BreakKeyword
                | Ty::CloseBrace
                | Ty::CloseParen
        )
    }
}
