use std::fs;

use serde::{Serialize, Deserialize};
use quote::{format_ident, quote};
use xshell::cmd;
use anyhow::Result;
use proc_macro2::{Punct, Spacing};
use ungrammar::Grammar;

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct KindsSrc {
    punct: Vec<(String, String)>,
    keywords: Vec<String>,
    literals: Vec<String>,
    tokens: Vec<String>,
    nodes: Vec<String>,
}

impl KindsSrc {
    pub fn get() -> Result<KindsSrc> {
        let s = fs::read_to_string(utils::xtask_root().join("assets/ast_src.toml"))?;
        let kinds: KindsSrc = toml::from_str(&s)?;
        Ok(kinds)
    }

    pub fn gen_syntax_kinds(&self) -> Result<String> {
        let (punctuation_matches, punctuation): (Vec<_>, Vec<_>) = self
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

        let keywords: Vec<_> = self
            .keywords
            .iter()
            .map(|keyword| format_ident!("{}Kw", to_camel_case(keyword)))
            .collect();

        let keyword_matches: Vec<_> = self
            .keywords
            .iter()
            .map(|keyword| format_ident!("{}", keyword))
            .collect();

        let literal_matches: Vec<_> = self
            .literals
            .iter()
            .map(|literal| format_ident!("{}", literal))
            .collect();

        let literals: Vec<_> = self
            .literals
            .iter()
            .map(|literal| format_ident!("{}Lit", to_camel_case(literal)))
            .collect();

        let tokens = self
            .tokens
            .iter()
            .map(|name| format_ident!("{}", name))
            .collect::<Vec<_>>();

        let nodes = self.nodes.iter().map(|node| format_ident!("{}", node));

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
                #(#punctuation,)*
                #(#keywords,)*
                #(#literals,)*
                #(#tokens,)*
                #(#nodes,)*
                // #(#tokens,)*
                // #(#nodes,)*

                // Technical kind so that we can cast from u16 safely
                #[doc(hidden)]
                __LAST,
            }

            /// A helper macro to get the SyntaxKind
            #[macro_export]
            macro_rules! T {
                #([#punctuation_matches] => { $crate::SyntaxKind::#punctuation };)*
                #([#keyword_matches] => { $crate::SyntaxKind::#keywords};)*
                #([#literal_matches] => { $crate::SyntaxKind::#literals};)*
                [ident] => { $crate::SyntaxKind::Ident };
                [__] => { $crate::SyntaxKind::Tombstone };
                [eof] => { $crate::SyntaxKind::Eof };
            }
        };

        Ok(reformat(&ast.to_string())?)
    }
}

pub const PREAMBLE: &str = "Generated file, do not edit by hand, see `xtask/src/codegen`";

pub fn reformat(text: &str) -> Result<String> {
    let stdout = cmd!("rustfmt").stdin(text).read()?;
    Ok(format!("//! {}\n\n{}\n", PREAMBLE, stdout))
}

pub fn to_camel_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let (idx, first) = s.char_indices().next().unwrap();

    buf.push(first.to_ascii_uppercase());
    buf.push_str(&s[idx + 1..]);
    buf
}

fn lingo_grammar() -> Result<Grammar> {
    let grammar_path = utils::xtask_root().join("assets/lingo.ungram");
    let src = fs::read_to_string(grammar_path)?;
    Ok(src.parse()?)
}
