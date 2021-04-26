mod ast_src;
mod kinds_src;

use crate::utils;
use ast_src::{AstSrc, Field};
use kinds_src::KindsSrc;

use std::fs;

use anyhow::Result;
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use ungrammar::Grammar;

pub fn run() -> Result<()> {
    let kinds_src = KindsSrc::get()?;
    let grammar: Grammar =
        fs::read_to_string(utils::xtask_root().join("assets/lua.ungram"))?.parse()?;
    let ast_src = AstSrc::lower(&grammar);

    let syntax_kinds_file =
        utils::project_root().join("crates/parser/src/syntax_kind/generated.rs");
    let syntax_kinds = gen_syntax_kinds(&kinds_src, &ast_src)?;
    utils::update(syntax_kinds_file.as_path(), &syntax_kinds)?;

    let ast_tokens_file = utils::project_root().join("crates/syntax/src/ast/generated/tokens.rs");
    let contents = gen_tokens(&kinds_src)?;
    utils::update(ast_tokens_file.as_path(), &contents)?;

    let ast_nodes_file = utils::project_root().join("crates/syntax/src/ast/generated/nodes.rs");
    let contents = gen_nodes(kinds_src, &ast_src)?;
    utils::update(ast_nodes_file.as_path(), &contents)?;

    eprintln!("grammar = {:#?}", ast_src);

    Ok(())
}

fn gen_syntax_kinds(kinds_src: &KindsSrc, ast_src: &AstSrc) -> Result<String> {
    let (punctuation_matches, punctuation): (Vec<_>, Vec<_>) = kinds_src
        .punct
        .iter()
        .map(|(token, name)| {
            let value = if "{}[]()".contains(token) {
                let c = token.chars().next().unwrap();
                quote! { #c }
            } else {
                let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
                quote! { #(#cs)* }
            };
            (value, format_ident!("{}", name))
        })
        .unzip();

    let keywords_values = &kinds_src.keywords;
    let keywords: Vec<_> = keywords_values
        .iter()
        .map(|keyword| format_ident!("{}Kw", keyword.to_camel_case()))
        .collect();
    let keyword_matches: Vec<_> = kinds_src
        .keywords
        .iter()
        .map(|keyword| format_ident!("{}", keyword))
        .collect();

    let literal_matches: Vec<_> = kinds_src
        .literals
        .iter()
        .map(|literal| format_ident!("{}", literal.to_snake_case()))
        .collect();
    let literals: Vec<_> = kinds_src
        .literals
        .iter()
        .map(|literal| format_ident!("{}", literal.to_camel_case()))
        .collect();

    let token_matches: Vec<_> = kinds_src
        .tokens
        .iter()
        .map(|token| format_ident!("{}", token.to_snake_case()))
        .collect();
    let tokens = kinds_src
        .tokens
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let trivia_matches: Vec<_> = kinds_src
        .trivia
        .iter()
        .map(|name| format_ident!("{}", name.to_snake_case()))
        .collect::<Vec<_>>();
    let trivia = kinds_src
        .trivia
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let node_names = ast_src.names().map(|s| format_ident!("{}", s)).collect::<Vec<_>>();

    let ast = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        /// The kind of syntax node, e.g. Ident, `UseKw`, or `Struct`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum SyntaxKind {
            // Technical SyntaxKinds: they appear temporally during parsing,
            // but never end up in the final tree
            #[doc(hidden)]
            Tombstone,
            #[doc(hidden)]
            Eof,
            #[doc(hiddent)]
            Unknown,
            #(#punctuation,)*
            #(#keywords,)*
            #(#literals,)*
            #(#tokens,)*
            #(#node_names,)*
            #(#trivia,)*

            // Technical kind so that we can cast from u16 safely
            #[doc(hidden)]
            __LAST,
        }
        use self::SyntaxKind::*;

        impl SyntaxKind {
            pub fn is_trivia(&self) -> bool {
                matches!(self, #(#trivia)|*)
            }

            pub fn is_keyword(&self) -> bool {
                matches!(self, #(#keywords)|*)
            }

            pub fn is_punct(&self) -> bool {
                matches!(self, #(#punctuation)|*)
            }

            pub fn is_literal(&self) -> bool {
                matches!(self, #(#literals)|*)
            }

            pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
                let kw = match ident {
                    #(#keywords_values => #keywords,)*
                    _ => return None,
                };
                Some(kw)
            }
        }

        /// A helper macro to get the token
        #[macro_export]
        macro_rules! T {
            #([#punctuation_matches] => { $crate::SyntaxKind::#punctuation };)*
            #([#keyword_matches] => { $crate::SyntaxKind::#keywords};)*
            #([#literal_matches] => { $crate::SyntaxKind::#literals};)*
            #([#token_matches] => { $crate::SyntaxKind::#tokens};)*
            #([#trivia_matches] => { $crate::SyntaxKind::#trivia};)*
            [__] => { $crate::SyntaxKind::Tombstone };
            [eof] => { $crate::SyntaxKind::Eof };
            [unknown] => { $crate::SyntaxKind::Unknown };
        }

        /// A helper macro to get the node
        #[macro_export]
        macro_rules! N {
            #([#node_names] => { $crate::SyntaxKind::#node_names };)*
        }
    };

    Ok(utils::reformat(&ast.to_string())?)
}

fn gen_tokens(kinds_src: &KindsSrc) -> Result<String> {
    let all_tokens = kinds_src
        .tokens
        .iter()
        .chain(kinds_src.literals.iter())
        .chain(kinds_src.trivia.iter());

    let tokens = all_tokens.map(|token| {
        let name = format_ident!("{}", token);
        let kind = format_ident!("{}", token.to_camel_case());
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct #name {
                pub(crate) syntax: SyntaxToken,
            }
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Display::fmt(&self.syntax, f)
                }
            }
            impl AstToken for #name {
                fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::#kind }
                fn cast(syntax: SyntaxToken) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                }
                fn syntax(&self) -> &SyntaxToken { &self.syntax }
            }
        }
    });

    let pretty = utils::reformat(
        &quote! {
            #![allow(dead_code)]

            use crate::{SyntaxKind, SyntaxToken, ast::AstToken};
            #(#tokens)*
        }
        .to_string(),
    )?
    .replace("#[derive", "\n#[derive");

    Ok(pretty)
}

fn gen_nodes(kinds: KindsSrc, grammar: &AstSrc) -> Result<String> {
    let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = grammar
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let kind = format_ident!("{}", node.name.to_camel_case());
            // let traits = node.traits.iter().map(|trait_name| {
            //     let trait_name = format_ident!("{}", trait_name);
            //     quote!(impl ast::#trait_name for #name {})
            // });

            let methods = node
                .fields
                .iter()
                .filter(|field| !matches!(field, Field::Token(s) if s == ",")) // filter out comma fields
                .map(|field| {
                    let method_name = field.method_name(&kinds.punct);
                    let ty = field.ty();

                    if field.is_many() {
                        quote! {
                            pub fn #method_name(&self) -> AstChildren<#ty> {
                                support::children(&self.syntax)
                            }
                        }
                    } else {
                        if let Some(token_kind) = field.token_kind() {
                            quote! {
                                pub fn #method_name(&self) -> Option<#ty> {
                                    support::token(&self.syntax, #token_kind)
                                }
                            }
                        } else {
                            quote! {
                                pub fn #method_name(&self) -> Option<#ty> {
                                    support::child(&self.syntax)
                                }
                            }
                        }
                    }
                });
            (
                quote! {
                    // #[pretty_doc_comment_placeholder_workaround]
                    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                    pub struct #name {
                        pub(crate) syntax: SyntaxNode,
                    }

                    // #(#traits)*

                    impl #name {
                        #(#methods)*
                    }
                },
                quote! {
                    impl AstNode for #name {
                        fn can_cast(kind: SyntaxKind) -> bool {
                            kind == SyntaxKind::#kind
                        }
                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                            if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                        }
                        fn syntax(&self) -> &SyntaxNode { &self.syntax }
                    }
                },
            )
        })
        .unzip();

    let (enum_defs, enum_boilerplate_impls): (Vec<_>, Vec<_>) = grammar
        .enums
        .iter()
        .map(|en| {
            let variants: Vec<_> = en
                .variants
                .iter()
                .map(|var| format_ident!("{}", var.to_camel_case()))
                .collect();
            let name = format_ident!("{}", en.name);
            let kinds: Vec<_> = variants
                .iter()
                .map(|name| format_ident!("{}", name.to_string().to_camel_case()))
                .collect();
            // let traits = en.traits.iter().map(|trait_name| {
            //     let trait_name = format_ident!("{}", trait_name);
            //     quote!(impl ast::#trait_name for #name {})
            // });

            // let ast_node = if en.name == "Stmt" {
            let ast_node = if false {
                quote! {}
            } else {
                quote! {
                    impl AstNode for #name {
                        fn can_cast(kind: SyntaxKind) -> bool {
                            match kind {
                                #(SyntaxKind::#kinds)|* => true,
                                _ => false,
                            }
                        }
                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                            let res = match syntax.kind() {
                                #(
                                SyntaxKind::#kinds => #name::#variants(#variants { syntax }),
                                )*
                                _ => return None,
                            };
                            Some(res)
                        }
                        fn syntax(&self) -> &SyntaxNode {
                            match self {
                                #(
                                #name::#variants(it) => &it.syntax,
                                )*
                            }
                        }
                    }
                }
            };

            (
                quote! {
                    // #[pretty_doc_comment_placeholder_workaround]
                    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                    pub enum #name {
                        #(#variants(#variants),)*
                    }

                    // #(#traits)*
                },
                quote! {
                    #(
                        impl From<#variants> for #name {
                            fn from(node: #variants) -> #name {
                                #name::#variants(node)
                            }
                        }
                    )*
                    #ast_node
                },
            )
        })
        .unzip();

    let enum_names = grammar.enums.iter().map(|it| &it.name);
    let node_names = grammar.nodes.iter().map(|it| &it.name);

    let display_impls = enum_names
        .chain(node_names.clone())
        .map(|it| format_ident!("{}", it))
        .map(|name| {
            quote! {
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        std::fmt::Display::fmt(self.syntax(), f)
                    }
                }
            }
        });

    // let defined_nodes: Vec<_> = node_names.collect();

    // for node in kinds
    //     .nodes
    //     .iter()
    //     .map(|kind| kind.to_camel_case())
    //     .filter(|name| !defined_nodes.iter().any(|&it| it == name))
    // {
    //     drop(node)
    // }

    let ast = quote! {
        #![allow(dead_code)]

        use super::tokens::*;
        use crate::{
            SyntaxNode, SyntaxToken, SyntaxKind,
            ast::{AstNode, AstChildren, support},
            T,
        };

        #(#node_defs)*
        #(#enum_defs)*
        #(#node_boilerplate_impls)*
        #(#enum_boilerplate_impls)*
        #(#display_impls)*
    };

    let ast = ast.to_string().replace("T ! [", "T![");

    // let mut res = String::with_capacity(ast.len() * 2);

    // let mut docs = grammar
    //     .nodes
    //     .iter()
    //     .map(|it| &it.doc)
    //     .chain(grammar.enums.iter().map(|it| &it.doc));

    // for chunk in ast.split("# [pretty_doc_comment_placeholder_workaround] ") {
    //     res.push_str(chunk);
    //     if let Some(doc) = docs.next() {
    //         write_doc_comment(&doc, &mut res);
    //     }
    // }

    let pretty = utils::reformat(&ast)?;
    Ok(pretty)
}
