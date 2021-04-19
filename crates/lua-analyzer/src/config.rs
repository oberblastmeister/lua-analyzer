use std::iter;

use lsp_types::ClientCapabilities;
use serde::de::DeserializeOwned;
use stdx::paths::AbsPathBuf;

config_data! {
    struct ConfigData {
        workspace_library: Vec<String> = "[]",
    }
}

impl Default for ConfigData {
    fn default() -> Self {
        ConfigData::from_json(serde_json::Value::Null)
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    caps: lsp_types::ClientCapabilities,
    data: ConfigData,
    root_path: AbsPathBuf,
}

impl Config {
    pub fn new(root_path: AbsPathBuf, caps: ClientCapabilities) -> Self {
        Config { caps, data: ConfigData::default(), root_path }
    }
    pub fn update(&mut self, json: serde_json::Value) {
        log::info!("updating config from JSON: {:#}", json);
        if json.is_null() || json.as_object().map_or(false, |it| it.is_empty()) {
            return;
        }
        self.data = ConfigData::from_json(json);
    }
}

macro_rules! _config_data {
    (struct $name:ident {
        $(
            $(#[doc=$doc:literal])*
            $field:ident $(| $alias:ident)?: $ty:ty = $default:expr,
        )*
    }) => {
        #[allow(non_snake_case)]
        #[derive(Debug, Clone)]
        struct $name { $($field: $ty,)* }
        impl $name {
            fn from_json(mut json: serde_json::Value) -> $name {
                $name {$(
                    $field: get_field(
                        &mut json,
                        stringify!($field),
                        None$(.or(Some(stringify!($alias))))?,
                        $default,
                    ),
                )*}
            }

            #[cfg(test)]
            fn manual() -> String {
                manual(&[
                    $({
                        let field = stringify!($field);
                        let ty = stringify!($ty);
                        (field, ty, &[$($doc),*], $default)
                    },)*
                ])
            }
        }
    };
}
use _config_data as config_data;

fn get_field<T: DeserializeOwned>(
    json: &mut serde_json::Value,
    field: &'static str,
    alias: Option<&'static str>,
    default: &str,
) -> T {
    let default = serde_json::from_str(default).unwrap();

    // XXX: check alias first, to work-around the VS Code where it pre-fills the
    // defaults instead of sending an empty object.
    alias
        .into_iter()
        .chain(iter::once(field))
        .find_map(move |field| {
            let mut pointer = field.replace('_', "/");
            pointer.insert(0, '/');
            json.pointer_mut(&pointer)
                .and_then(|it| serde_json::from_value(it.take()).ok())
        })
        .unwrap_or(default)
}

#[cfg(test)]
fn manual(fields: &[(&'static str, &'static str, &[&str], &str)]) -> String {
    fields
        .iter()
        .map(|(field, _ty, doc, default)| {
            let name = format!("lua-analyzer.{}", field.replace("_", "."));
            let doc = doc_comment_to_string(*doc);
            format!(
                "[[{}]]{} (default: `{}`)::\n+\n--\n{}--\n",
                name, name, default, doc
            )
        })
        .collect::<String>()
}

fn doc_comment_to_string(doc: &[&str]) -> String {
    doc.iter()
        .map(|it| it.strip_prefix(' ').unwrap_or(it))
        .map(|it| format!("{}\n", it))
        .collect()
}
