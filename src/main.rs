mod color;
mod image;
mod tests;

use color::color_constants;
use image::Image;
const XRES: usize = 500;
const YRES: usize = 500;
fn main() {
    let mut img = Image::new(XRES, YRES);

    // img.set_y_invert(true);
    
    let xresint = XRES as i32;
    let yresint = YRES as i32;
    // 1 and 5
    img.draw_line((0, 0), (xresint - 1, yresint - 1), color_constants::GREEN);
    img.draw_line((0, 0), (xresint - 1, yresint / 2), color_constants::GREEN);
    img.draw_line((xresint - 1, yresint - 1), (0, yresint / 2), color_constants::GREEN);

    // 8 and 4
    img.draw_line((0, yresint - 1), (xresint - 1, 0), color_constants::CYAN);
    img.draw_line((0, yresint - 1), (xresint - 1, yresint / 2), color_constants::CYAN);
    img.draw_line((xresint - 1, 0), (0, yresint / 2), color_constants::CYAN);

    // 2 and 6
    img.draw_line((0, 0), (xresint / 2, yresint - 1), color_constants::RED);
    img.draw_line((xresint - 1, yresint - 1), (xresint / 2, 0), color_constants::RED);

    // 7 and 3
    img.draw_line((0, yresint - 1), (xresint / 2, 0), color_constants::PURPLE);
    img.draw_line((xresint - 1, 0), (xresint / 2, yresint - 1), color_constants::PURPLE);

    // horizontal and vertical
    img.draw_line((0, yresint / 2), (xresint - 1, yresint / 2), color_constants::YELLOW);
    img.draw_line((xresint / 2, 0), (xresint / 2, yresint - 1), color_constants::YELLOW);

    img.write_file("result").expect("Image write to file failed");
}