#[cfg(test)]
mod tests {
    use pres_compiler::analyzer::generic_analyzer::{AnalysisErr, YmlAnalyzer};
    use pres_compiler::analyzer::slide::{SlideYml, SlideYmlData};
    use pres_compiler::analyzer::presentation::{PresYml, PresYmlData};

    fn create_test_slide_yaml<'a>() -> &'a str {
        r#"
            slide:
              title: Test slide
              subtitle: Test subtitle
        "#
    }

    #[test]
    fn test_serialization_slide() {
        let test_serialized_slide: SlideYml = *SlideYml::new(create_test_slide_yaml()).unwrap();
        let test_slide_data: SlideYmlData = SlideYmlData {
            title: Some("Test slide".to_string()),
            subtitle: Some("Test subtitle".to_string()),
            plain_text: None,
            images: None,
        };

        let test_slide: SlideYml = SlideYml {
            slide: test_slide_data,
        };

        assert_eq!(test_slide, test_serialized_slide);
    }
}
