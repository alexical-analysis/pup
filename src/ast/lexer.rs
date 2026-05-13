use crate::compiler::{context::Context, str_store::MStr};

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
    pub fn new_open_paren(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::OpenParen,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("("),
        }
    }

    pub fn new_close_paren(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::CloseParen,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr(")"),
        }
    }

    pub fn new_open_brace(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::OpenBrace,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("{"),
        }
    }

    pub fn new_close_brace(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::CloseBrace,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("}"),
        }
    }

    pub fn new_range(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::RangeOperator,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr(".."),
        }
    }

    pub fn new_module(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::ModuleOperator,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("::"),
        }
    }

    pub fn new_plus(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::Plus,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("+"),
        }
    }

    pub fn new_minus(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::Minus,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("-"),
        }
    }

    pub fn new_multiply(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::Multiply,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("*"),
        }
    }

    pub fn new_divide(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::Divide,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("/"),
        }
    }

    pub fn new_equal(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::Equal,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("="),
        }
    }

    pub fn new_equal_equal(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::EqualEqual,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("=="),
        }
    }

    pub fn new_less_than(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::LessThan,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr("<"),
        }
    }

    pub fn new_semicolon(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::Semicolon,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr(";"),
        }
    }

    pub fn new_comma(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::Comma,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr(","),
        }
    }

    pub fn new_eof(ctx: &mut Context, pos: usize) -> Self {
        Self {
            ty: Ty::Eof,
            pos: Pos(pos as u32),
            lexeme: ctx.get_mstr(""),
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
    pub fn new(ctx: &mut Context, source: &'s str) -> Self {
        let mut lexer = Self {
            source,
            pos: 0,
            is_at_eos: false,
            next: Token::new_eof(ctx, 0),
        };

        // populate the first token so the lexer is ready to go
        lexer.next(ctx);

        lexer
    }

    /// if the lexer is in a bad spot, this will just eat tokens till we find the next decl
    pub fn recover_until_decl(&mut self, ctx: &mut Context) {
        loop {
            let token = self.next(ctx);
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
    pub fn recover_until_expr(&mut self, ctx: &mut Context) {
        loop {
            let token = self.next(ctx);
            if [Ty::Semicolon, Ty::CloseBrace].contains(&token.ty) {
                self.next(ctx);
                break;
            }
        }
    }

    pub fn peek(&self) -> &Token {
        &self.next
    }

    pub fn next(&mut self, ctx: &mut Context) -> Token {
        self.is_at_eos = false;
        let token = self.next;

        let next = self.lex_token(ctx);
        self.next = next;

        if self.can_end_statement(token.ty) {
            self.is_at_eos = true;
        }

        token
    }

    fn lex_token(&mut self, ctx: &mut Context) -> Token {
        // skip whitespace and comments
        self.skip();

        let ch = match self.next_char() {
            Some(ch) => ch,
            None => return Token::new_eof(ctx, self.pos),
        };

        if self.insert_semicolon(ch) {
            return Token::new_semicolon(ctx, self.pos);
        }

        match ch {
            'a'..='z' => self.lex_ident(ctx, ch),
            'A'..='Z' => self.lex_ident(ctx, ch),
            '0'..='9' => self.lex_number(ctx, ch),
            '(' => Token::new_open_paren(ctx, self.bump(ch)),
            ')' => Token::new_close_paren(ctx, self.bump(ch)),
            '{' => Token::new_open_brace(ctx, self.bump(ch)),
            '}' => Token::new_close_brace(ctx, self.bump(ch)),
            '+' => Token::new_plus(ctx, self.bump(ch)),
            '-' => Token::new_minus(ctx, self.bump(ch)),
            '*' => Token::new_multiply(ctx, self.bump(ch)),
            '/' => Token::new_divide(ctx, self.bump(ch)),
            '<' => Token::new_less_than(ctx, self.bump(ch)),
            ';' => Token::new_semicolon(ctx, self.bump(ch)),
            ',' => Token::new_comma(ctx, self.bump(ch)),
            '=' => match self.source.get(self.pos + 1..self.pos + 2) {
                Some("=") => {
                    self.bump('=');
                    self.bump('=');
                    Token::new_equal_equal(ctx, self.pos)
                }
                _ => Token::new_equal(ctx, self.bump(ch)),
            },
            '.' => match self.source.get(self.pos + 1..self.pos + 2) {
                Some(".") => {
                    self.bump('.');
                    self.bump('.');
                    Token::new_range(ctx, self.pos)
                }
                _ => self.lex_unknown(ctx, ch),
            },
            ':' => match self.source.get(self.pos + 1..self.pos + 2) {
                Some(":") => {
                    self.bump(':');
                    self.bump(':');
                    Token::new_module(ctx, self.pos)
                }
                _ => self.lex_unknown(ctx, ch),
            },
            _ => self.lex_unknown(ctx, ch),
        }
    }

    fn lex_unknown(&mut self, ctx: &mut Context, ch: char) -> Token {
        let start = self.pos;
        self.bump(ch);

        let mut len = 1;

        while let Some(ch) = self.next_char() {
            len += 1;

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

        let end = start + len;
        let s = &self.source[start..end];
        let lexeme = ctx.get_mstr(s);

        Token {
            ty: Ty::Unknown,
            pos: Pos::from(start),
            lexeme,
        }
    }

    fn lex_ident(&mut self, ctx: &mut Context, ch: char) -> Token {
        let start = self.pos;
        self.bump(ch);

        let mut len = ch.len_utf8();
        while let Some(ch) = self.next_char() {
            len += ch.len_utf8();

            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }

            self.bump(ch);
        }

        let end = start + len;
        let s = &self.source[start..end];

        self.ident_to_keyword(Pos::from(start), s, ctx)
    }

    fn ident_to_keyword(&self, pos: Pos, s: &str, ctx: &mut Context) -> Token {
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
            _ => Ty::Identifier,
        };

        let lexeme = ctx.get_mstr(s);
        Token { ty, pos, lexeme }
    }

    fn lex_number(&mut self, ctx: &mut Context, ch: char) -> Token {
        let start = self.pos;
        self.bump(ch);

        let mut ty = Ty::Int;
        // safe to use 1 here since digits are always a single byte
        let mut len = 1;

        while let Some(ch) = self.next_char() {
            // safe to use 1 here since digits are always a single byte
            len += 1;

            if ch == '.' && ty == Ty::Float {
                // we found a second '.' since this is already a float, simply return the floating point
                // number here and let lexer figure out what to do with the next '.'
                break;
            }

            if ch == '.' && ty == Ty::Int {
                // we found a '.' so this is actually a floating point number
                ty = Ty::Float
            }

            if !ch.is_numeric() && ch != '_' {
                break;
            }

            self.bump(ch);
        }

        let end = start + len;
        let s = &self.source[start..end];
        let lexeme = ctx.get_mstr(s);

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
