use pres_compiler::analyzer::slide::SlideYml;
use pres_compiler::analyzer::generic_analyzer::YmlAnalyzer;

use pres_compiler::validators::slide;

fn main() {
    let st = r#"
            slide:
              title: Test slide
              subtitle: Test subtitle
              images:
                - ./lib.rs
                - ./main.rs
        "#;
    let test_serialized_slide: SlideYml = *SlideYml::new(st).unwrap();
    match slide::is_valid(&test_serialized_slide){
        Ok(_) => println!("{:?}", test_serialized_slide),
        Err(err) => println!("{}", err),
    };
}
