mod encode;
mod decode;

use std::{process::exit, io::Read};

use clap::{Parser, Subcommand};
use decode::write_image_to_file;
/// A CLI for the blurhash algoritm developed at Wolt, using a pure Rust implementation.
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Encodes a file or a base64 string.
    Encode {
        /// Name of input image file.
        #[clap(short, long)]
        input_file: Option<String>,

        /// Stdin input
        #[clap(short = 'S', long, takes_value = false)]
        stdin: bool,

        /// Use base64 as input, instead of a file.
        #[clap(short, long)]
        base_64: Option<String>,

        /// Name of a output file, else the output is printed directly to standard output.
        #[clap(short, long)]
        output_file: Option<String>,

        /// Width of the image (if you don't want to encode the entire image),
        #[clap(short = 'W', long)]
        width: Option<u32>,

        /// Height of the image
        #[clap(short = 'H', long)]
        height: Option<u32>,

        /// X components (read more on the woltapp/blurhash README.md repo)
        #[clap(short, long, default_value_t = 4)]
        x_components: u32,

        /// Y components
        #[clap(short, long, default_value_t = 3)]
        y_components: u32
    },
    Decode {
        /// Filename of the input blurhash string file.
        #[clap(short, long)]
        input_file: Option<String>,

        /// Whether to take intput from stdin
        #[clap(short = 'I', long, takes_value = false)]
        stdin: bool,

        /// Filename of the output image (if one is used).
        #[clap(short, long)]
        output_file: Option<String>,

        /// Whether to output raw data to stdout.
        #[clap(short = 'O', long, takes_value = false)]
        stdout: bool,

        /// Width of the image (if you don't want to decode the entire image),
        #[clap(short = 'W', long, default_value_t = 960)]
        width: u32,

        /// Height of the image
        #[clap(short = 'H', long, default_value_t = 320)]
        height: u32,

        /// Punch is a parameter that adjusts the contrast on the decoded image. 1 means normal, smaller values will make the effect more subtle, and larger values will make it stronger.
        #[clap(short, long, default_value_t = 1.0)]
        punch: f32,
    }
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Encode { 
            input_file, 
            output_file, 
            stdin,
            width, 
            height, 
            base_64,
            x_components,
            y_components
        } => {
            match input_file {
                Some(filename) => {                    
                    let blurhash = encode::file_img_to_blurhash(filename, *width, *height, *x_components, *y_components);
                    match output_file {
                        Some(file) => encode::write_blurhash_to_file(&blurhash, file),
                        None => encode::write_blurhash_to_stdout(&blurhash)
                    }
                },
                None if *stdin => {
                    let mut buffer = Vec::new();

                    match std::io::stdin().read_to_end(&mut buffer) {
                        Ok(_) => {
                            let blurhash = encode::raw_to_blurhash(&buffer, *width, *height, *x_components, *y_components);
                            match output_file {
                                Some(file) => encode::write_blurhash_to_file(&blurhash, file),
                                None => encode::write_blurhash_to_stdout(&blurhash)
                            }
                        },
                        Err(_) => {
                            eprintln!("Error while reading from stdin!");
                            exit(-1);
                        }
                    };
                }
                None => {
                  match base_64 {
                    Some(base64) => {
                        let blurhash = encode::base64_to_blurhash(base64, *width, *height, *x_components, *y_components);
                        match output_file {
                            Some(file) => encode::write_blurhash_to_file(&blurhash, file),
                            None => encode::write_blurhash_to_stdout(&blurhash)
                        }
                    },
                    None => {
                        eprintln!("Please select an input!");
                        exit(-1);
                    }
                  }  
                }
            }
        },
        Commands::Decode {
            input_file, 
            output_file, 
            width, 
            height, 
            punch,
            stdin,
            stdout,
        } => {
            match input_file {
                Some(input) => {
                    let blurhash = decode::read_blurhash_from_file(input);

                    let image = decode::blurhash_to_image_data(&blurhash, *width, *height, *punch);
                    
                    match output_file {
                        Some(file) => write_image_to_file(image, file),
                        None if *stdout => {
                            decode::write_image_to_stdout(image);
                        },
                        None => {
                            eprintln!("Please select an output!");
                            exit(-1);
                        }
                    }
                },
                None if *stdin => {
                    let mut buffer = Vec::new();

                    match std::io::stdin().read_to_end(&mut buffer) {
                        Ok(_) => {
                            match String::from_utf8(buffer) {
                                Ok(blurhash) => {
                                    let image = decode::blurhash_to_image_data(&blurhash, *width, *height, *punch);

                                    match output_file {
                                        Some(file) => {
                                            if let Err(_) = image.save(file) {
                                                eprintln!("Error while writing to image!");
                                                exit(-1);
                                            }
                                        },
                                        None if *stdout => {
                                            decode::write_image_to_stdout(image);
                                        },
                                        None => {
                                            eprintln!("Please select an output!");
                                            exit(-1);
                                        }
                                    }
                                },
                                Err(_) => {
                                    eprintln!("Please select an input!");
                                    exit(-1);
                                }
                            }
                        },
                        Err(_) => {
                            eprintln!("Error while reading from stdin!");
                            exit(-1);
                        }
                    };
                },
                None => {
                    eprintln!("Please select an input!");
                    exit(-1);
                }
            }
        },
        
    }
}
