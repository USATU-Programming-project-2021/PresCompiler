use super::generic_analyzer::{YmlAnalyzer, AnalysisErr};
use serde_yaml::Error;
use serde_yaml::Value;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SlideYml{
    pub slide: SlideYmlData
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SlideYmlData{
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub plain_text: Option<String>,
    pub images: Option<Vec<String>>
}
impl YmlAnalyzer for SlideYml{

    fn is_valid<'a>(&self) -> Result<&'a Value, AnalysisErr>{
        unimplemented!();
    }

    fn get_data(&self) -> Self{
        unimplemented!();
    }
}
