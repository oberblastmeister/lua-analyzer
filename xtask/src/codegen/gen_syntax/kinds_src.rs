use std::{collections::BTreeMap, fs};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct KindsSrc {
    pub punct: PunctMap,
    pub keywords: Vec<String>,
    pub literals: Vec<String>,
    pub tokens: Vec<String>,
    pub trivia: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PunctMap(BTreeMap<String, String>);

impl PunctMap {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|v| &**v)
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, String> {
        self.0.iter()
    }
}

impl KindsSrc {
    pub fn get() -> Result<KindsSrc> {
        let s = fs::read_to_string(utils::xtask_root().join("assets/ast_src.toml"))?;
        let kinds: KindsSrc = toml::from_str(&s)?;
        Ok(kinds)
    }
}
