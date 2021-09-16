
use image::{DynamicImage, GenericImageView, GrayImage, Luma};

///
pub fn encode_basic(input_image: DynamicImage, output_image: &mut GrayImage) {
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
}