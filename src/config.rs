use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type ThemeVars = BTreeMap<String, String>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BlockConfig {
    pub path: String,
    #[serde(default = "default_comment")]
    pub comment: String,
    #[serde(flatten)]
    pub block: BlockOptions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaggedConfig {
    pub path: String,
    #[serde(default = "default_comment")]
    pub comment: String,
    pub blocks: BTreeMap<String, BlockOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BlockOptions {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FileConfig {
    Multi(TaggedConfig),
    Single(BlockConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub themes: BTreeMap<String, ThemeVars>,
    pub files: BTreeMap<String, FileConfig>,
    pub reload: Option<String>,
}
