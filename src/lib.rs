pub mod slides_parser {
    use serde_yaml::Value;
    use std::path::Path;

    pub enum ParseError {
        SerializationError,
        SyntaxError,
    }

    pub struct Slide {
        pub title: Option<String>,
        pub sub_titile: Option<String>,
        pub plain_text: Option<String>,
        pub images: Option<Vec<Box<Path>>>,
    }

    fn get_string_param_from_val_mapping(
        mapping: &serde_yaml::Mapping,
        param: String,
    ) -> Option<String> {
        let params_mappping = Value::String(param);
        match mapping.get(&params_mappping) {
            Some(title) => match title.as_str() {
                Some(str_title) => Some(String::from(str_title)),
                None => None,
            },
            None => None,
        }
    }

    impl Slide {
        fn new() -> Self {
            Slide {
                title: None,
                sub_titile: None,
                plain_text: None,
                images: None,
            }
        }

        pub fn from_yml_value(value: Value) -> Option<Self> {
            let slide_map = match value.as_mapping() {
                Some(map) => map,
                None => return None,
            };
            let params_map = {
                let slide_mapping = Value::String(String::from("slide"));
                match slide_map.get(&slide_mapping) {
                    Some(slide) => match slide.as_mapping() {
                        Some(_slide_map) => _slide_map,
                        None => return None,
                    },
                    None => return None,
                }
            };
            Some(Slide {
                title: get_string_param_from_val_mapping(params_map, String::from("title")),
                sub_titile: get_string_param_from_val_mapping(params_map, String::from("subtitle")),
                plain_text: get_string_param_from_val_mapping(
                    params_map,
                    String::from("plain_text"),
                ),
                images: None,
            })
        }

        pub fn from_str(yml_str: &str) -> Option<Self> {
            let val: Option<Value> = match serde_yaml::from_str(yml_str) {
                Ok(val) => val,
                Err(_) => None,
            };
            Slide::from_yml_value(match val {
                Some(_val) => _val,
                None => return None,
            })
        }
    }

    pub fn is_slide_valid(slide: &Slide) -> bool {
        true
    }

    pub fn pre_parse_slide(yaml_slide: String) -> Result<Value, serde_yaml::Error> {
        serde_yaml::from_str(&yaml_slide[..])
    }

    pub fn parse_slide(yaml_slide: String) -> Result<Slide, ParseError> {
        let slide = Slide::new();

        unimplemented!();
    }
}

pub mod presentation {
    use super::slides_parser::{ParseError, Slide};

    struct Presentation {
        slides: Option<Vec<Box<Slide>>>,
    }
}
