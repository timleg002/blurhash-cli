use std::{process::exit, fs::File, io::Write};

use image::{GenericImageView, ImageError, EncodableLayout, DynamicImage};

pub fn file_img_to_blurhash(
    filename: &str, 
    width: Option<u32>, 
    height: Option<u32>,
    x_components: u32, 
    y_components: u32
) -> String {
    match image::open(filename) {
        Ok(img) => {
            return img_to_blurhash(img, width, height, x_components, y_components);
        },
        Err(err) => {
            match err {
                ImageError::Decoding(_) => {
                    eprintln!("Couldn't decode the image!");
                    exit(-1);
                },
                ImageError::IoError(_) => {
                    eprintln!("Couldn't load the image!");
                    exit(-1);
                },
                _ => {
                    eprintln!("An unknown error occured while loading the image.");
                    exit(-1);
                }
            }
        } 
    };
}

pub fn img_to_blurhash(image: DynamicImage,width: Option<u32>, height: Option<u32>,x_components: u32, y_components: u32) -> String {
    let (w, h) = match (width, height) {
        (Some(w), Some(h)) => (w, h),
        _ => image.dimensions(),
    };

    let blurhash = blurhash::encode(
        x_components,
        y_components, 
        w,
        h,
        &image.to_rgba8().into_vec()
    );

    blurhash
}

pub fn raw_to_blurhash(raw: &[u8], width: Option<u32>, height: Option<u32>,x_components: u32, y_components: u32) -> String {
    match image::load_from_memory(raw) {
        Ok(img) => {
            return img_to_blurhash(img, width, height, x_components, y_components);
        },
        Err(err) => { // TODO remove code duplication
            match err {
                ImageError::Decoding(_) => {
                    eprintln!("Couldn't decode the image!");
                    exit(-1);
                },
                ImageError::IoError(_) => {
                    eprintln!("Couldn't load the image!");
                    exit(-1);
                },
                _ => {
                    eprintln!("An unknown error occured while loading the image.");
                    exit(-1);
                }
            }
        } 
    };
}

pub fn base64_to_blurhash(
    base64: &str,
    width: Option<u32>,
    height: Option<u32>,
    x_components: u32,
    y_components: u32
) -> String {
    match base64::decode(base64) {
        Ok(decoded_base64) => {
            match image::load_from_memory(decoded_base64.as_bytes()) {
                Ok(image) => {
                    let (w, h) = match (width, height) {
                        (Some(w), Some(h)) => (w, h),
                        _ => image.dimensions(),
                    };

                    return blurhash::encode(
                        x_components, 
                        y_components, 
                        w, 
                        h, 
                        &image.to_rgba8().to_vec()
                    );
                }, 
                Err(err) => {
                    match err {
                        ImageError::Decoding(_) => {
                            eprintln!("Couldn't decode the image!");
                            exit(-1);
                        },
                        ImageError::IoError(_) => {
                            eprintln!("Couldn't load the image!");
                            exit(-1);
                        },
                        _ => {
                            eprintln!("An unknown error occured while loading the image.");
                            exit(-1);
                        }
                    }
                }
            }
        },
        Err(_) => {
            eprintln!("Invalid base64 value!");
            exit(-1);
        }
    };
}

pub fn write_blurhash_to_file(blurhash: &str, output_file: &str) {
    match File::create(output_file) {
        Ok(mut file) => {
            if let Err(_) = file.write(blurhash.as_bytes()) {
                eprintln!("An error occured while writing to file");
                exit(-1);
            }
        },
        Err(_) => {
            eprintln!("Unable to open file {output_file}!");
            exit(-1);
        }
    }
}

pub fn write_blurhash_to_stdout(blurhash: &str) {
    println!("{blurhash}");
    exit(0);
}

