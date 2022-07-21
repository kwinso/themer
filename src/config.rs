use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type ThemeVars = BTreeMap<String, String>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileConfig {
    pub path: String,
    #[serde(default = "default_comment")]
    pub comment: String,

    #[serde(default)]
    pub only: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,

    pub aliases: Option<ThemeVars>,

    #[serde(default = "default_format")]
    pub format: String,
    pub custom: Option<String>,
}

fn default_comment() -> String {
    "#".to_owned()
}
fn default_format() -> String {
    "<key> = <value>".to_owned()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub themes: BTreeMap<String, ThemeVars>,
    pub files: BTreeMap<String, FileConfig>,
    pub reload: Option<String>,
}
