use super::generic_analyzer::YmlAnalyzer;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SlideYml {
    pub slide: SlideYmlData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SlideYmlData {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub plain_text: Option<String>,
    pub images: Option<Vec<String>>,
}
impl YmlAnalyzer for SlideYml {}
