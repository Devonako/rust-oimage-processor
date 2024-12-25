use image::{GenericImageView, ImageFormat};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <command> <input_file> <output_file>", args[0]);
        return;
    }

    let command = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let img = image::open(input_file).unwrap();

    match command.as_str() {
        "resize" => {
            // Example: resize to 500x500
            let resized_img = img.resize(500, 500, image::imageops::FilterType::Lanczos3); 
            resized_img.save(output_file).unwrap();
        }
        "convert" => {
            // Example: convert to JPG
            let output_format = ImageFormat::Jpeg; 
            img.save_with_format(output_file, output_format).unwrap();
        }
        "grayscale" => {
            let grayscale_img = img.grayscale();
            grayscale_img.save(output_file).unwrap();
        }
        "blur" => {
            // Example: apply blur with sigma 1.0
            let blurred_img = img.blur(1.0); 
            blurred_img.save(output_file).unwrap();
        }
        _ => println!("Invalid command"),
    }
}