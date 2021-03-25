#[cfg(test)]
mod tests {
    use pres_compiler::analyzer::generic_analyzer::YmlAnalyzer;
    use pres_compiler::analyzer::slide::{SlideYml, SlideYmlData};
    use pres_compiler::analyzer::presentation::{PresYml, PresYmlData};

    fn create_test_slide_yaml<'a>() -> &'a str {
        r#"
            slide:
              title: Test slide
              subtitle: Test subtitle
        "#
    }

    fn create_test_pres_yaml<'a>() -> &'a str {
        r#"
           pres: 
             slides:
               - slide:
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

    #[test]
    fn test_serialization_pres() {
        let test_slide_data: SlideYmlData = SlideYmlData {
            title: Some("Test slide".to_string()),
            subtitle: Some("Test subtitle".to_string()),
            plain_text: None,
            images: None,
        };

        let test_slide: SlideYml = SlideYml {
            slide: test_slide_data,
        };


        let test_serialized_pres: PresYml = *PresYml::new(create_test_pres_yaml()).unwrap();
        let test_pres_data: PresYmlData = PresYmlData {
            theme: None,
            slides: vec![test_slide]
        };

        let test_pres: PresYml = PresYml {
            pres: test_pres_data
        };

        assert_eq!(test_pres, test_serialized_pres);
    }

}
