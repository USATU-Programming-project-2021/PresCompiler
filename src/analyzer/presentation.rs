use super::generic_analyzer::YmlAnalyzer;
use super::slide::SlideYml;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PresYml {
    pub pres: PresYmlData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PresYmlData {
    pub theme: Option<String>,
    pub slides: Vec<SlideYml>,
}
impl YmlAnalyzer for PresYml {}
