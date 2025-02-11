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
    pub file: Option<String>,
    pub html: Option<String>,
    pub inline_code: Option<bool>,
    pub italic: Option<bool>,
    pub items: Option<Value>,
    pub level: Option<u8>,
    pub marker: Option<bool>,
    pub message: Option<String>,
    pub r#type: Option<String>,
    pub style: Option<String>,
    pub text: Option<String>,
    pub title_type: Option<String>,
    pub title: Option<String>,
    pub underline: Option<bool>,
    pub url: Option<String>,
    pub with_headings: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListItem {
    pub content: String,
    pub items: Vec<ListItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChecklistItem {
    pub text: String,
    pub checked: bool,
}
