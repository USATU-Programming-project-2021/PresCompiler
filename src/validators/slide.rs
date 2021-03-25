use crate::analyzer::slide::SlideYml;
use std::path::Path;

pub enum ValidateError {
    ImageDoesntExists(String),
}

impl ToString for ValidateError {
    fn to_string(&self) -> String {
        match &self {
            &ValidateError::ImageDoesntExists(err_msg) => err_msg.clone(),
        }
    }
}

enum ValidateSucces {
    Success,
    Unchanged,
}

#[derive(Debug)]
pub struct Slide {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub plain_text: Option<String>,
    pub images: Option<Vec<String>>,
}

impl Slide {
    fn validate_images(
        slide_images: &Option<Vec<String>>,
    ) -> Result<ValidateSucces, ValidateError> {
        let images: &Vec<String> = match &slide_images {
            Some(image) => image,
            None => return Ok(ValidateSucces::Unchanged),
        };
        for image in images.iter() {
            let image_path: &Path = Path::new(image);
            if !image_path.is_file() {
                return Err(ValidateError::ImageDoesntExists(format!(
                    "Filename '{}' doesn't exists",
                    image_path.display()
                )));
            };
        }
        Ok(ValidateSucces::Success)
    }

    fn is_valid(slide_yml: &SlideYml) -> Result<ValidateSucces, ValidateError> {
        Slide::validate_images(&slide_yml.slide.images)?;
        Ok(ValidateSucces::Success)
    }

    pub fn new(slide_yml: SlideYml) -> Result<Self, ValidateError> {
        match Slide::is_valid(&slide_yml) {
            Ok(_) => Ok(Slide {
                title: slide_yml.slide.title,
                subtitle: slide_yml.slide.subtitle,
                plain_text: slide_yml.slide.plain_text,
                images: slide_yml.slide.images,
            }),
            Err(validate_err) => return Err(validate_err),
        }
    }
}
