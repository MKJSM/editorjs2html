// Models

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Block {
    pub r#type: String,
    pub data: Data,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub align_text: Option<String>,
    pub align: Option<String>,
    pub alignment: Option<String>,
    pub bold: Option<bool>,
    pub caption: Option<String>,
    pub code: Option<String>,
    pub color: Option<String>,
    pub content: Option<Value>,
    pub embed: Option<String>,
    pub file: Option<File>,
    pub height: Option<i64>,
    pub html: Option<String>,
    pub inline_code: Option<bool>,
    pub italic: Option<bool>,
    pub items: Option<Value>,
    pub level: Option<u8>,
    pub marker: Option<bool>,
    pub message: Option<String>,
    pub r#type: Option<String>,
    pub stretched: Option<bool>,
    pub style: Option<String>,
    pub text: Option<String>,
    pub title_type: Option<String>,
    pub title: Option<String>,
    pub underline: Option<bool>,
    pub url: Option<String>,
    pub width: Option<i64>,
    pub with_background: Option<bool>,
    pub with_border: Option<bool>,
    pub with_headings: Option<bool>,
    pub line_width: Option<u32>,
    pub line_thickness: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListItem {
    pub content: String,
    pub items: Vec<ListItem>,
    pub meta: Meta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChecklistItem {
    pub text: String,
    pub checked: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct File {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    #[serde(default)]
    pub checked: bool,
}
