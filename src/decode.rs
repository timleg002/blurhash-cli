use image::{RgbaImage, Rgba, EncodableLayout};

use std::{process::exit, fs::File, io::Read};

pub fn blurhash_to_image_data(blurhash: &str, width: u32, height: u32, punch: f32) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    let raw_decoding = blurhash::decode(blurhash, width, height, punch);

    match RgbaImage::from_raw(width, height, raw_decoding) {
        Some(buffer) => {
            return buffer;
        },
        None => {
            eprintln!("Could not parse image!");
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

pub fn write_image_to_file(img: image::ImageBuffer<Rgba<u8>, Vec<u8>>, filename: &str) {
    if let Err(_) = img.save(filename) {
        eprintln!("Error saving the image");
        exit(-1);
    }
}

pub fn write_image_to_stdout(img: image::ImageBuffer<Rgba<u8>, Vec<u8>>) {
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