use proc_macro::{Span, TokenStream};
use std::hash::{Hash, Hasher};
use std::{cmp::Ordering, collections::BTreeSet};

type Result<T> = std::result::Result<T, String>;

#[doc(hidden)]
#[proc_macro]
pub fn __deduplicate_enum(token_stream: TokenStream) -> TokenStream {
    try_deduplicate_enum(token_stream)
        .unwrap_or_else(|msg| parse_ts(&format!("compile_error!({:?})", msg)))
}

#[derive(Debug)]
struct Hashable {
    ident: String,
    span: Span,
}

impl Hash for Hashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}

impl PartialEq for Hashable {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Eq for Hashable {}

impl Into<proc_macro::Ident> for Hashable {
    fn into(self) -> proc_macro::Ident {
        proc_macro::Ident::new(&self.ident, self.span)
    }
}

impl PartialOrd for Hashable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ident.partial_cmp(&other.ident)
    }
}

impl Ord for Hashable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn try_deduplicate_enum(token_stream: TokenStream) -> Result<TokenStream> {
    let name = token_stream.clone().into_iter().next().unwrap();

    let deduplicated = token_stream
        .into_iter()
        .skip(1)
        .map(|token| {
            Hashable {
                ident: token.to_string(),
                span: token.span(),
            }
        })
        .collect::<BTreeSet<_>>();

    let mut res = TokenStream::new();

    res.extend(parse_ts("#[derive(Debug, Copy, Clone)]"));
    res.extend(parse_ts("pub"));
    res.extend(parse_ts("enum"));
    res.extend(Some(name));

    let mut s = String::from("{");
    for it in deduplicated {
        s.push_str(&it.ident);
        s.push(',');
    }
    s.push_str("__LAST");
    s.push('}');

    res.extend(parse_ts(&s));

    Ok(res)
}

fn parse_ts(s: &str) -> TokenStream {
    s.parse().unwrap()
}
