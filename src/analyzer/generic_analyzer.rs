use serde_yaml::Error;
use serde::de::DeserializeOwned;

#[derive(Debug, PartialEq)]
pub enum AnalysisErr {
    SyntaxErr(String),
    LogicErr(String),
    OsErr(String)
}

impl ToString for AnalysisErr{
    fn to_string(&self) -> String{
        match self{
            AnalysisErr::LogicErr(err) => err.clone(),
            AnalysisErr::SyntaxErr(err) => err.clone(),
            AnalysisErr::OsErr(err) => err.clone()
        }
    }
}

pub trait YmlAnalyzer {
    fn new<T: DeserializeOwned>(yml: &str) -> Result<Box<T>, AnalysisErr>{
        let slide: Result<T, Error> = serde_yaml::from_str(yml);

        match slide{
            Ok(slide) => Ok(Box::new(slide)),
            Err(err) => Err(AnalysisErr::LogicErr(err.to_string()))
        }
    }
}
