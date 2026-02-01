use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct PdxFile {
    pub version: u32,
    pub meta: PdxMeta,
    pub content: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PdxMeta {
    pub title: String,
    pub lang: String,
}
