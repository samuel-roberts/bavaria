extern crate num;
#[macro_use]
extern crate num_derive;

mod decoders;
mod encoders;

use clap::{App, Arg, ArgMatches};
use image::{io::Reader, GenericImageView, GrayImage, ImageBuffer, RgbImage};
use std::{io, path};

use decoders::{decode_none, decode_simple, DecodeMode};
use encoders::encode_basic;

fn main() {
    let mut app = App::new("Bavaria")
        .version("0.0.1")
        .author("Samuel Roberts")
        .about("TODO")
        .subcommand(
            App::new("encode")
                .arg(
                    Arg::new("input")
                        .value_name("FILE")
                        .about("Input file name")
                        .takes_value(true)
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .about("Output file name")
                        .required(false),
                ),
        )
        .subcommand(
            App::new("decode")
                .arg(
                    Arg::new("input")
                        .value_name("FILE")
                        .about("Input file name")
                        .takes_value(true)
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("mode")
                        .short('m')
                        .long("mode")
                        .value_name("VALUE")
                        .about("Decoding mode")
                        .required(false),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .about("Output file name")
                        .required(false),
                ),
        );

    let opts = app.get_matches_mut();

    if let Some(ref opts) = opts.subcommand_matches("encode") {
        encode(*opts);
    } else if let Some(ref opts) = opts.subcommand_matches("decode") {
        decode(*opts);
    } else {
        app.write_help(&mut io::stdout())
            .expect("Failed to print help message");
    }
}

/// Encode an image
fn encode(opts: &ArgMatches) {
    let input_filename = opts.value_of("input").unwrap();
    let output_filename = match opts.value_of("output") {
        Some(value) => value.to_owned(),
        None => {
            let mut path = path::PathBuf::from(&input_filename);
            path.set_extension("bayered.png");
            path.to_string_lossy().to_string()
        }
    };

    let input_image = Reader::open(&input_filename).unwrap().decode().unwrap();
    let mut output_image: GrayImage = ImageBuffer::new(input_image.width(), input_image.height());

    encode_basic(input_image, &mut output_image);

    output_image
        .save(&output_filename)
        .expect("Failed to write output");
}

/// Decode an image
fn decode(opts: &ArgMatches) {
    let input_filename = opts.value_of("input").unwrap();
    let output_filename = match opts.value_of("output") {
        Some(value) => value.to_owned(),
        None => {
            let mut path = path::PathBuf::from(&input_filename);
            path.set_extension("debayered.png");
            path.to_string_lossy().to_string()
        }
    };
    let mode = match opts.value_of("mode") {
        Some(value) => match value.parse::<u32>() {
            Ok(enum_value) => {
                num::FromPrimitive::from_u32(enum_value).unwrap_or(DecodeMode::Invalid)
            }
            Err(_) => DecodeMode::Invalid,
        },
        None => DecodeMode::Simple,
    };

    if mode == DecodeMode::Invalid {
        // TODO Print help data
        println!("Mode not supported");
        return;
    }

    let input_image = Reader::open(&input_filename).unwrap().decode().unwrap();
    let mut output_image: RgbImage = ImageBuffer::new(input_image.width(), input_image.height());

    match mode {
        DecodeMode::None => decode_none(input_image, &mut output_image),
        DecodeMode::Simple => decode_simple(input_image, &mut output_image),
        _ => {
            println!("Decode mode {} is unsupported", mode as i32);
            return;
        }
    }

    output_image
        .save(&output_filename)
        .expect("Failed to write output");
}
