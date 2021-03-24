use super::generic_analyzer::{YmlAnalyzer, AnalysisErr};
use super::slide::{SlideYml};
use serde_yaml::Error;
use serde_yaml::Value;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PresYml{
    pub pres: PresYmlData
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PresYmlData{
    pub theme: Option<String>,
    pub slides: Vec<SlideYml>
}

impl YmlAnalyzer for PresYml{

    fn is_valid<'a>(&self) -> Result<&'a Value, AnalysisErr>{
        unimplemented!();
    }

    fn get_data(&self) -> Self{
        unimplemented!();
    }
}
