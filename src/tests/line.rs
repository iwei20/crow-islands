use crate::color;

fn rotate((x, y): (i32, i32)) {

}

fn amongus(&mut img: Image, (x, y): (i32, i32), (width, height): (i32, i32), angle_rads: f32, iters: usize, total_iters: usize) {
    if iters == 0 {return}
    let color = color!(iters * 255 / total_iters, 0, 255 - iters * 255 / total_iters);
    // Upper shell of the astronaut
    img.draw_line(
        (x - height / (2 * angle_rads.acos()), y - width / (2 * angle_rads.cos())), 
        (x + height / (2 * angle_rads.acos()), y - width / (2 * angle_rads.cos())), 
        color
    );
    img.draw_line(
        (x - height / (2 * angle_rads.acos()), y - width / (2 * angle_rads.asin())), 
        (x + height / (2 * angle_rads.acos()), y - width / (2 * angle_rads.asin())), 
        color
    );
    img.draw_line(
        (x - height / (2 * angle_rads.sin()), y - width / (2 * angle_rads.cos())), 
        (x + height / (2 * angle_rads.sin()), y - width / (2 * angle_rads.cos())), 
        color
    );
    img.draw_line(
        (x - height / (2 * angle_rads.sin()), y - width / (2 * angle_rads.cos())), 
        (x + height / (2 * angle_rads.sin()), y - width / (2 * angle_rads.cos())), 
        color
    );
    img.draw_line(());
    amongus(img, (x, y), (width * 3/4, height * 3/4), angle_rads + f32::consts::PI / 6, iters - 1, total_iters);
}

#[test]
fn spiral_amongla() {
    let mut image = Image::new(500, 500);
    amongus(image, (250, 250), (200, 400), f32::consts::PI, 12, 12);
    image.write_file_test("spiral_amongla");
}