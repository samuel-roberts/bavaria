
extern crate num;
#[macro_use]
extern crate num_derive;

use std::{io, path};
use clap::{App, Arg, ArgMatches};
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, RgbImage, Rgb, GrayImage, Luma};

fn main() {
    
    let mut app = App::new("Bavaria")
        .version("0.0.1")
        .author("Samuel Roberts")
        .about("TODO")
        .subcommand(App::new("encode")
            .arg(Arg::new("input")
                .value_name("FILE")
                .about("Input file name")
                .takes_value(true)
                .required(true)
                .index(1))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .about("Output file name")
                .required(false)))
        .subcommand(App::new("decode")
            .arg(Arg::new("input")
                .value_name("FILE")
                .about("Input file name")
                .takes_value(true)
                .required(true)
                .index(1))
            .arg(Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("VALUE")
                .about("Decoding mode")
                .required(false))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .about("Output file name")
                .required(false)));
    
    let opts = app.get_matches_mut();

    if let Some(ref opts) = opts.subcommand_matches("encode") {
        encode(*opts);
    } else if let Some(ref opts) = opts.subcommand_matches("decode") {
        decode(*opts);
    } else {
        app.write_help(&mut io::stdout()).expect("Failed to print help message");
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
    let mut output_image : GrayImage = ImageBuffer::new(input_image.width(), input_image.height());

    for y in 0..input_image.height() {
        for x in 0..input_image.width() {
            let pixel = input_image.get_pixel(x, y);

            if (x & 1 == 0) && (y & 1 == 0) {
                // X and Y are both even: blue filter
                output_image.put_pixel(x, y, Luma([pixel[2]]));
            } else if (x & 1 == 1) && (y & 1 == 1) {
                // X and Y are both odd: red filter
                output_image.put_pixel(x, y, Luma([pixel[0]]));
            } else {
                // X and Y are mixed: green filter
                output_image.put_pixel(x, y, Luma([pixel[1]]));
            }
        }
    }

    output_image.save(&output_filename).expect("Failed to write output");
}

#[derive(Eq, PartialEq, FromPrimitive)]
enum DecodeMode {
    Invalid = -1,
    None = 0,
    Simple = 1,
    Interpolate = 2
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
            Ok(enum_value) => num::FromPrimitive::from_u32(enum_value).unwrap_or(DecodeMode::Invalid),
            Err(_) => DecodeMode::Invalid
        },
        None => DecodeMode::None
    };

    if mode == DecodeMode::Invalid {
        // TODO Print help data
        println!("Mode not supported");
        return 
    }

    let input_image = Reader::open(&input_filename).unwrap().decode().unwrap();
    let mut output_image : RgbImage = ImageBuffer::new(input_image.width(), input_image.height());
    
    match mode {
        DecodeMode::None => decode_none(input_image, &mut output_image),
        DecodeMode::Simple => decode_simple(input_image, &mut output_image),
        _ => {
            println!("Decode mode {} is unsupported", mode as i32);
            return;
        }
    }

    output_image.save(&output_filename).expect("Failed to write output");
}

///
fn decode_none(input_image: DynamicImage, output_image: &mut RgbImage) {
    for y in 0..input_image.height() {
        for x in 0..input_image.width() {
            let pixel = input_image.get_pixel(x, y)[0];

            if (x & 1 == 0) && (y & 1 == 0) {
                // X and Y are both even: blue filter
                output_image.put_pixel(x, y, Rgb([0, 0, pixel]));
            } else if (x & 1 == 1) && (y & 1 == 1) {
                // X and Y are both odd: red filter
                output_image.put_pixel(x, y, Rgb([pixel, 0, 0]));
            } else {
                // X and Y are mixed: green filter
                output_image.put_pixel(x, y, Rgb([0, pixel, 0]));
            }
        }
    }
}

///
fn decode_simple(input_image: DynamicImage, output_image: &mut RgbImage) {
    for y in (0..input_image.height()).step_by(2) {
        for x in (0..input_image.width()).step_by(2) {
            
            let blue = input_image.get_pixel(x, y)[0];
            let green0 = input_image.get_pixel(x + 1, y)[0];
            let green1 = input_image.get_pixel(x, y + 1)[0];
            let red = input_image.get_pixel(x + 1, y + 1)[0];

            let pixel0 = Rgb([red, green0, blue]);
            let pixel1 = Rgb([red, green1, blue]);
            output_image.put_pixel(x, y, pixel0);
            output_image.put_pixel(x + 1, y, pixel0);
            output_image.put_pixel(x, y + 1, pixel1);
            output_image.put_pixel(x + 1, y + 1, pixel1);
        }
    }
}