use crate::analyzer::slide::SlideYml;
use std::path::Path;
use std::fmt;

pub enum ValidateError {
    ImageDoesntExists(String),
}

impl fmt::Display for ValidateError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            ValidateError::ImageDoesntExists(err_msg) => write!(f, "{}", err_msg),
        }
    }    
}

pub enum ValidateSucces {
    Success,
    Unchanged,
}

fn validate_images(slide_images: &Option<Vec<String>>) -> Result<ValidateSucces, ValidateError> {
    let images: &Vec<String> = match &slide_images {
        Some(image) => image,
        None => return Ok(ValidateSucces::Unchanged),
    };

    for image in images {
        if !Path::new(image).is_file() {
            return Err(ValidateError::ImageDoesntExists(format!(
                "File '{}' doesn't exists",
                image
            )));
        };
    };

    Ok(ValidateSucces::Success)
}

pub fn is_valid(slide_yml: &SlideYml) -> Result<ValidateSucces, ValidateError> {
    validate_images(&slide_yml.slide.images)?;
    Ok(ValidateSucces::Success)
}
