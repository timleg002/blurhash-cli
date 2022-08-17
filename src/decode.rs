use image::EncodableLayout;

use std::{process::exit, fs::File, io::Read};

pub fn blurhash_to_image_data(blurhash: &str, width: u32, height: u32, punch: f32) -> image::DynamicImage {
    let raw_decoding = blurhash::decode(blurhash, width, height, punch);

    match image::load_from_memory(raw_decoding.as_bytes()) {
        Ok(image) => {
            return image;
        },
        Err(_) => {
            eprintln!("Error decoding the blurhash");
            exit(-1); 
        }
    }
}

pub fn read_blurhash_from_file(filename: &str) -> String {
    match File::open(filename) {
        Ok(mut file) => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Ok(_) => {
                    return buf;
                },
                Err(_) => {
                    eprintln!("Could not read file {filename}!");
                    exit(-1);
                } 
            };
        },
        Err(_) => {
            eprintln!("Error when reading input blurhash file! (filename: {filename})");
            exit(-1);
        }
    };
}

pub fn write_image_to_file(img: image::DynamicImage, filename: &str) {
    if let Err(_) = img.save(filename) {
        eprintln!("Error saving the image");
        exit(-1);
    }
}

pub fn write_image_to_stdout(img: image::DynamicImage) {
    println!(
        "{}",
        img
            .as_bytes()
            .iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}