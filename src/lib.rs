
pub mod slides_parser {
    use std::path::Path;

    pub struct ParseError(String);

    pub struct Slide {
        title: String,
        sub_titile: Option<String>,
        plain_text: Option<String>,
        images: Option<Vec<Box<Path>>>,
    }

    pub fn parse_slide(yaml_slide: String) -> Result<Slide, ParseError> {
        unimplemented!();
    }

}
