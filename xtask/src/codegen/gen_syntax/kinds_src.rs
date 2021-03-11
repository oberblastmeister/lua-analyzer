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
    pub punct: Vec<(String, String)>,
    pub keywords: Vec<String>,
    pub literals: Vec<String>,
    pub tokens: Vec<String>,
    pub trivia: Vec<String>,
}

impl KindsSrc {
    pub fn get() -> Result<KindsSrc> {
        let s = fs::read_to_string(utils::xtask_root().join("assets/ast_src.toml"))?;
        let kinds: KindsSrc = toml::from_str(&s)?;
        Ok(kinds)
    }
}
