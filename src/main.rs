
use std::{io, path};
use clap::{App, Arg, ArgMatches};
use image::{io::Reader, GenericImageView, ImageBuffer, GrayImage, Luma};

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
                .required(false))
        )
        .subcommand(App::new("decode"));
    
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

/// Decode an image
fn decode(_opts: &ArgMatches) {
    // TODO
}
