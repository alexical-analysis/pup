use crate::ast::ast::{Decl, DeclValue, FuncDecl, ModDecl, TyDecl, UseDecl};
use crate::ast::lexer::{Lexer, Token, Ty};
use crate::ast::parser::Parser;

fn parse_mod_decl(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Decl {
    let name = lexer.next(parser.str_store());
    if name.ty != Ty::Identifier {
        lexer.recover_until_decl(parser.str_store());
        return parser.get_decl(name, DeclValue::Invalid("function is missing a body"));
    }

    parser.get_decl(token, DeclValue::Mod(ModDecl { name: name.lexeme }))
}

fn parse_use_decl(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Decl {
    let open_paren = lexer.next(parser.str_store());
    if open_paren.ty != Ty::OpenParen {
        lexer.recover_until_decl(parser.str_store());
        return parser.get_decl(
            open_paren,
            DeclValue::Invalid("missing opening paren in use decl"),
        );
    }

    let mut deps = vec![];
    loop {
        let next = lexer.next(parser.str_store());
        match next.ty {
            Ty::Identifier => deps.push(next.lexeme),
            Ty::CloseParen => break,
            _ => {
                lexer.recover_until_decl(parser.str_store());
                return parser.get_decl(next, DeclValue::Invalid("invalid token inside use block"));
            }
        }
        let end = lexer.next(parser.str_store());
        if end.ty != Ty::Semicolon {
            lexer.recover_until_decl(parser.str_store());
            return parser.get_decl(
                end,
                DeclValue::Invalid("missing semicolon/new line after module name"),
            );
        }
    }

    parser.get_decl(token, DeclValue::Use(UseDecl { deps }))
}

fn parse_type_decl(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Decl {
    let name = lexer.next(parser.str_store());
    if name.ty != Ty::Identifier {
        lexer.recover_until_decl(parser.str_store());
        return parser.get_decl(name, DeclValue::Invalid("missing name for type decl"));
    }

    let ty = lexer.next(parser.str_store());
    if name.ty != Ty::Identifier {
        lexer.recover_until_decl(parser.str_store());
        return parser.get_decl(name, DeclValue::Invalid("missing type alias"));
    }

    let ty = parser.parse_type(ty.lexeme);

    parser.get_decl(
        token,
        DeclValue::Ty(TyDecl {
            name: name.lexeme,
            ty,
        }),
    )
}

fn parse_function_decl(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Decl {
    let function_name = lexer.next(parser.str_store());
    if function_name.ty != Ty::Identifier {
        lexer.recover_until_decl(parser.str_store());
        return parser.get_decl(
            function_name,
            DeclValue::Invalid("function decl is missing a name"),
        );
    }

    let open_args = lexer.next(parser.str_store());
    if open_args.ty != Ty::OpenParen {
        lexer.recover_until_decl(parser.str_store());
        return parser.get_decl(
            open_args,
            DeclValue::Invalid("function decl is missing an open paren"),
        );
    }

    let mut param_names = vec![];
    let mut param_tys = vec![];

    loop {
        let param_name;
        let param_ty;

        let name = lexer.next(parser.str_store());
        match name.ty {
            Ty::CloseParen => break,
            Ty::Eof => {
                return parser.get_decl(
                    name,
                    DeclValue::Invalid("function decl is missing a closing paren"),
                );
            }
            Ty::Identifier => param_name = name.lexeme,
            _ => {
                lexer.recover_until_decl(parser.str_store());
                return parser.get_decl(
                    name,
                    DeclValue::Invalid("function param needs an identifer for a name"),
                );
            }
        }

        let ty = lexer.next(parser.str_store());
        match ty.ty {
            Ty::Identifier => {
                param_ty = parser.parse_type(ty.lexeme);
            }
            _ => {
                lexer.recover_until_decl(parser.str_store());
                return parser.get_decl(ty, DeclValue::Invalid("expecting paramater type spec"));
            }
        }

        let comma = lexer.next(parser.str_store());
        match comma.ty {
            Ty::Comma => {
                param_names.push(param_name);
                param_tys.push(param_ty);
            }
            Ty::CloseParen => break,
            _ => {
                lexer.recover_until_decl(parser.str_store());
                return parser.get_decl(
                    comma,
                    DeclValue::Invalid("paramaters must be separated by commas"),
                );
            }
        }
    }

    let return_ty = lexer.peek().clone();
    let return_ty = match return_ty.ty {
        Ty::Identifier => {
            lexer.next(parser.str_store());
            parser.parse_type(return_ty.lexeme)
        }
        Ty::OpenBrace => parser.module.type_unit(),
        _ => {
            lexer.recover_until_decl(parser.str_store());
            return parser.get_decl(return_ty, DeclValue::Invalid("expecting return type"));
        }
    };

    let open_body = lexer.next(parser.str_store());
    if open_body.ty != Ty::OpenBrace {
        lexer.recover_until_decl(parser.str_store());
        return parser.get_decl(open_body, DeclValue::Invalid("function is missing a body"));
    }

    let body = parser.parse_block(lexer);

    let function_ty = parser.get_fn_type(param_tys, return_ty);
    parser.get_decl(
        token,
        DeclValue::Func(FuncDecl {
            name: function_name.lexeme,
            params: param_names,
            body,
            ty: function_ty,
        }),
    )
}

pub fn parse_decl(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Decl {
    match token.ty {
        Ty::ModKeyword => parse_mod_decl(parser, lexer, token),
        Ty::UseKeyword => parse_use_decl(parser, lexer, token),
        Ty::TypeKeyword => parse_type_decl(parser, lexer, token),
        Ty::FnKeyword => parse_function_decl(parser, lexer, token),
        _ => {
            lexer.recover_until_decl(parser.str_store());
            parser.get_decl(token, DeclValue::Invalid("unknown declaration type"))
        }
    }
}
