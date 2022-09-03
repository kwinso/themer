use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type ThemeVars = BTreeMap<String, String>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BlockConfig {
    pub path: String,
    #[serde(default = "default_comment")]
    pub comment: String,
    pub closing_comment: Option<String>,

    #[serde(skip)]
    pub tag: Option<String>,

    #[serde(flatten)]
    pub block: BlockOptions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaggedConfig {
    pub path: String,
    #[serde(default = "default_comment")]
    pub comment: String,
    pub closing_comment: Option<String>,
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

impl FileConfig {
    pub fn get_path(&self) -> String {
        match self {
            FileConfig::Single(v) => v.path.clone(),
            FileConfig::Multi(v) => v.path.clone(),
        }
    }
    pub fn flatten(&self) -> Vec<BlockConfig> {
        match self {
            FileConfig::Single(v) => vec![v.clone()],
            FileConfig::Multi(mutli) => mutli
                .clone()
                .blocks
                .into_iter()
                .map(|(tag, block)| BlockConfig {
                    tag: Some(tag),
                    path: mutli.path.clone(),
                    comment: mutli.comment.clone(),
                    closing_comment: mutli.closing_comment.clone(),
                    block,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub themes: BTreeMap<String, ThemeVars>,
    pub files: BTreeMap<String, FileConfig>,
    pub reload: Option<String>,
}
