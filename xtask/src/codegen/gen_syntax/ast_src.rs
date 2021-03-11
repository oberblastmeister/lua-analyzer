use heck::SnakeCase;
use quote::{format_ident, quote};
use ungrammar::{Grammar, Rule};

use super::kinds_src::PunctMap;

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

impl AstSrc {
    pub fn lower(grammar: &Grammar) -> AstSrc {
        lower(grammar)
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.nodes
            .iter()
            .map(|node| &*node.name)
            .chain(self.enums.iter().map(|enoom| &*enoom.name))
    }
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node {
        name: String,
        ty: String,
        cardinality: Cardinality,
    },
}

impl Field {
    pub fn is_many(&self) -> bool {
        matches!(
            self,
            Field::Node {
                cardinality: Cardinality::Many,
                ..
            }
        )
    }

    pub fn is_many_trailing(&self) -> bool {
        matches!(
            self,
            Field::Node {
                cardinality: Cardinality::ManyTrailing,
                ..
            }
        )
    }

    pub fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Field::Token(token) => {
                let token: proc_macro2::TokenStream = token.parse().unwrap();
                Some(quote! { T![#token] })
            }
            _ => None,
        }
    }

    pub fn method_name(&self, punct_map: &PunctMap) -> proc_macro2::Ident {
        match self {
            Field::Token(name) => {
                let name = match &**name {
                    "'{'" => "l_curly",
                    "'}'" => "r_curly",
                    "'('" => "l_paren",
                    "')'" => "r_paren",
                    "'['" => "l_brack",
                    "']'" => "r_brack",
                    _ => punct_map.get(name.as_str()).unwrap_or(name),
                };
                format_ident!("{}_token", name.to_snake_case())
            }
            Field::Node { name, .. } => {
                format_ident!("{}", name)
            }
        }
    }

    pub fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(_) => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    ManyTrailing,
    Many,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) variants: Vec<String>,
}

pub(crate) fn lower(grammar: &Grammar) -> AstSrc {
    let mut res = AstSrc::default();

    let nodes = grammar.iter().collect::<Vec<_>>();

    for &node in &nodes {
        let name = grammar[node].name.clone();
        let rule = &grammar[node].rule;

        eprintln!("Lowering name: {}", name);

        match lower_enum(grammar, rule) {
            Some(variants) => {
                let enum_src = AstEnumSrc {
                    doc: Vec::new(),
                    name,
                    variants,
                };
                res.enums.push(enum_src);
            }
            None => {
                let mut fields = Vec::new();
                lower_rule(&mut fields, grammar, None, rule);
                res.nodes.push(AstNodeSrc {
                    doc: Vec::new(),
                    name,
                    fields,
                });
            }
        }
    }

    res
}

fn lower_enum(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
    let alternatives = match rule {
        Rule::Alt(it) => it,
        _ => return None,
    };
    let mut variants = Vec::new();
    for alternative in alternatives {
        match alternative {
            Rule::Node(it) => variants.push(grammar[*it].name.clone()),
            // Rule::Token(it) if grammar[*it].name == ";" => (),
            _ => return None,
        }
    }
    Some(variants)
}

fn lower_rule(acc: &mut Vec<Field>, grammar: &Grammar, label: Option<&String>, rule: &Rule) {
    if lower_comma_list(acc, grammar, label, rule) {
        return;
    }

    match rule {
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let name = label.cloned().unwrap_or_else(|| ty.to_snake_case());
            let field = Field::Node {
                name,
                ty,
                cardinality: Cardinality::Optional,
            };
            acc.push(field);
        }
        Rule::Token(token) => {
            if label.is_some() {
                // make sure that we aren't doing things such as op:'!'
                panic!("Found label `{}` on token `{:?}`", label.unwrap(), token);
            }

            let mut name = grammar[*token].name.clone();
            if name != "string" {
                if "[]{}()".contains(&name) {
                    name = format!("'{}'", name);
                }
                let field = Field::Token(name);
                acc.push(field);
            }
        }
        Rule::Rep(inner) => {
            if let Rule::Node(node) = &**inner {
                let ty = grammar[*node].name.clone();
                let name = label
                    .cloned()
                    .unwrap_or_else(|| pluralize(&ty.to_snake_case()));
                let field = Field::Node {
                    name,
                    ty,
                    cardinality: Cardinality::Many,
                };
                acc.push(field);
                return;
            }
            todo!("{:?}", rule) // this will happen because we didn't do the comma list properly
        }
        Rule::Labeled { label: l, rule } => {
            assert!(label.is_none());

            // let manually_implemented = matches!(
            //     l.as_str(),
            //     "lhs"
            //         | "rhs"
            //         | "then_branch"
            //         | "else_branch"
            //         | "start"
            //         | "end"
            //         | "op"
            //         | "index"
            //         | "base"
            //         | "value"
            //         | "trait"
            //         | "self_ty"
            // );
            // if manually_implemented {
            //     return;
            // }
            lower_rule(acc, grammar, Some(l), rule);
        }
        Rule::Seq(rules) | Rule::Alt(rules) => {
            for rule in rules {
                lower_rule(acc, grammar, label, rule)
            }
        }
        Rule::Opt(rule) => lower_rule(acc, grammar, label, rule),
    }
}

fn lower_comma_list(
    acc: &mut Vec<Field>,
    grammar: &Grammar,
    label: Option<&String>,
    rule: &Rule,
) -> bool {
    let rule = match rule {
        Rule::Seq(it) => it,
        _ => return false,
    };
    let (node, repeat, trailing_comma, cardinality) = match rule.as_slice() {
        [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_comma)] => (
            node,
            repeat,
            Some(trailing_comma),
            Cardinality::ManyTrailing,
        ),
        [Rule::Node(node), Rule::Rep(repeat)] => (node, repeat, None, Cardinality::Many),
        _ => return false,
    };
    let repeat = match &**repeat {
        Rule::Seq(it) => it,
        _ => return false,
    };
    match repeat.as_slice() {
        [comma, Rule::Node(n)] if Some(comma) == trailing_comma.map(|t| &**t) && n == node => (),
        [_comma, Rule::Node(n)] if n == node => (),
        _ => return false,
    }
    let ty = grammar[*node].name.clone();
    let name = label
        .cloned()
        .unwrap_or_else(|| pluralize(&ty.to_snake_case()));
    let field = Field::Node {
        name,
        ty,
        cardinality,
    };
    acc.push(field);
    true
}

fn pluralize(s: &str) -> String {
    format!("{}s", s)
}
