#[cfg(test)]
mod tests {
    use pres_compiler::slides_parser::Slide;
    use serde_yaml::Value;

    fn create_test_slide() -> Slide {
        Slide {
            title: Some("Test slide".to_string()),
            sub_titile: Some("Test subtitle".to_string()),
            plain_text: Option::None,
            images: Option::None,
        }
    }

    fn create_test_yaml<'a>() -> &'a str {
        r#"
            slide:
              title: Test slide
              subtitle: Test subtitle
        "#
    }


    #[test]
    fn test_parse() {
        assert_eq!(Slide::from_str(create_test_yaml()).unwrap(), create_test_slide());
    }
}
