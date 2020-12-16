extern crate image;

use crate::image::GenericImageView;

#[derive(Debug)]
pub struct RgbData {
    r_avg: u32,
    g_avg: u32,
    b_avg: u32,
    pixels_number: u32,
}

impl RgbData {
    pub fn read_image_sizes(&self, filename: &str) -> (u32, u32) {
        let im = image::open(filename).unwrap();
        let (width, height) = im.dimensions();
        (width, height)
    }

    pub fn read_number_of_pixels(&self, filename: &str) -> u32 {
        let sizes = self.read_image_sizes(&filename);
        sizes.0 * sizes.1
    }

    pub fn count_avgs(&self, filename: &str) -> (u32, u32, u32) {
        let im = image::open(filename).unwrap();
        let (mut r_sum, mut g_sum, mut b_sum) = (0, 0, 0);
        let photo = im.to_rgba8();
        for px in photo.pixels() {
            r_sum += px[0] as u32;
            g_sum += px[1] as u32;
            b_sum += px[2] as u32;
        }
        let pixels = self.read_number_of_pixels(&filename);
        (r_sum / pixels, g_sum / pixels, b_sum / pixels)
    }
}

impl Default for RgbData {
    fn default() -> RgbData {
        RgbData {
            r_avg: 0,
            g_avg: 0,
            b_avg: 0,
            pixels_number: 0,
        }
    }
}