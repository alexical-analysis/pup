use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ast::{Decl, DeclValue, FunctionDecl, ParamExpr};
use crate::ast::lexer::{Lexer, Token, Ty};
use crate::ast::parser::Parser;

pub trait DeclParselet {
    fn parse(&self, parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Decl;
}

pub struct FunctionDeclParselet;

impl DeclParselet for FunctionDeclParselet {
    fn parse(&self, parser: &mut Parser, lexer: &mut Lexer, _token: Token) -> Decl {
        let recover_set = &[
            Ty::FnKeyword,
            Ty::UseKeyword,
            Ty::ModKeyword,
            Ty::TypeKeyword,
        ];

        let function_name = lexer.next(parser.ctx_mut());
        if function_name.ty != Ty::Identifier {
            lexer.recover_until(parser.module.ctx, recover_set);

            let decl = parser.get_decl(DeclValue::Invalid("function decl is missing a name"));
            return decl;
        }

        let open_args = lexer.next(parser.ctx_mut());
        if open_args.ty != Ty::OpenParen {
            // eat tokens till we're in a known position again
            lexer.recover_until(parser.ctx_mut(), recover_set);

            let decl =
                parser.get_decl(DeclValue::Invalid("function decl is missing an open paren"));
            return decl;
        }

        let mut params = vec![];

        loop {
            let param_name;
            let param_type;

            let name = lexer.next(parser.ctx_mut());
            match name.ty {
                Ty::CloseParen => break,
                Ty::Eof => {
                    let decl = parser.get_decl(DeclValue::Invalid(
                        "function decl is missing a closing paren",
                    ));
                    return decl;
                }
                Ty::Identifier => param_name = name.lexeme,
                _ => {
                    lexer.recover_until(parser.ctx_mut(), recover_set);

                    let decl = parser.get_decl(DeclValue::Invalid(
                        "function param needs an identifer for a name",
                    ));
                    return decl;
                }
            }

            let ty = lexer.next(parser.ctx_mut());
            match ty.ty {
                Ty::Identifier => param_type = parser.parse_type(ty.lexeme),
                _ => {
                    lexer.recover_until(parser.ctx_mut(), recover_set);

                    let decl = parser.get_decl(DeclValue::Invalid("expecting paramater type spec"));
                    return decl;
                }
            }

            let comma = lexer.next(parser.ctx_mut());
            match comma.ty {
                Ty::Comma => params.push(ParamExpr {
                    name: param_name,
                    ty: param_type,
                }),
                Ty::CloseParen => break,
                _ => {
                    lexer.recover_until(parser.ctx_mut(), recover_set);

                    let decl = parser
                        .get_decl(DeclValue::Invalid("paramaters must be separated by commas"));
                    return decl;
                }
            }
        }

        let return_ty = lexer.peek().clone();
        let return_ty = match return_ty.ty {
            Ty::Identifier => {
                lexer.next(parser.ctx_mut());
                parser.parse_type(return_ty.lexeme)
            }
            Ty::OpenBrace => parser.module.type_unit(),
            _ => {
                lexer.recover_until(parser.ctx_mut(), recover_set);

                let decl = parser.get_decl(DeclValue::Invalid("expecting return type"));
                return decl;
            }
        };

        let open_body = lexer.next(parser.ctx_mut());
        if open_body.ty != Ty::OpenBrace {
            lexer.recover_until(parser.ctx_mut(), recover_set);

            let decl = parser.get_decl(DeclValue::Invalid("function is missing a body"));
            return decl;
        }

        let body = parser.parse_body();

        parser.get_decl(DeclValue::Function(FunctionDecl {
            name: function_name.lexeme,
            params,
            body,
            return_ty,
        }))
    }
}

pub fn new_decl_parselets() -> HashMap<Ty, Rc<dyn DeclParselet>> {
    let mut parselets: HashMap<Ty, Rc<dyn DeclParselet>> = HashMap::new();
    parselets.insert(Ty::FnKeyword, Rc::new(FunctionDeclParselet));

    parselets
}
