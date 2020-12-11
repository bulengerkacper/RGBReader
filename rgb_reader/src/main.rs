extern crate image;

use crate::image::GenericImageView;

fn main() {
    read_pix_from_file("C:\\Users\\Hyperbook\\Desktop\\nosa.jpg");
}

pub fn read_pix_from_file(filename: &str) -> (u32,u32,u32) {
    let im = image::open(filename).expect("File not found.");
    let (width, height) = im.dimensions();
    let pixels = width * height;
    let photo = im.to_rgba8();
    let mut r_sum = 0;
    let mut g_sum = 0;
    let mut b_sum = 0;
    for px in photo.pixels() {
        r_sum = r_sum + px[0] as u32;
        g_sum = g_sum + px[1] as u32;
        b_sum = b_sum + px[2] as u32;
    }
    (r_sum / pixels, g_sum / pixels, b_sum / pixels)
}
