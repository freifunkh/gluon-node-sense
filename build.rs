use grass::{from_path, OutputStyle};
use std::fs;
use std::path::Path;

fn main() {
    let input_scss = "web/sass/style.scss";
    let output_css = "web/static/css/style.css";

    println!("cargo:rerun-if-changed={}", input_scss);

    if Path::new(input_scss).exists() {
        match from_path(
            input_scss,
            &grass::Options::default().style(OutputStyle::Compressed),
        ) {
            Ok(css) => {
                fs::write(output_css, css).expect("Failed to write CSS file");
                println!("SCSS compiled successfully: {}", output_css);
            }
            Err(e) => panic!("Error compiling SCSS: {}", e),
        }
    } else {
        panic!("SCSS file not found: {}", input_scss);
    }
}
