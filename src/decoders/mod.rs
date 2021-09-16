
// TODO Split this further

use image::{DynamicImage, GenericImageView, RgbImage, Rgb};

///
#[derive(Eq, PartialEq, FromPrimitive)]
pub enum DecodeMode {
    Invalid = -1,
    None = 0,
    Simple = 1,
    Interpolate = 2
}

///
pub fn decode_none(input_image: DynamicImage, output_image: &mut RgbImage) {
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
pub fn decode_simple(input_image: DynamicImage, output_image: &mut RgbImage) {
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