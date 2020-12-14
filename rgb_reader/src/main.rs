extern crate image;

use crate::image::GenericImageView;
use fltk::*;
use std::path::Path;

use fltk::{app::*, button::*, dialog::*, output::*, window::*};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "R G B Image Processor.");
    let mut but = Button::new(100, 100, 80, 40, "Process it!");
    let mut out_r = Output::new(100, 200, 30, 30, "");
    let mut out_g = Output::new(150, 200, 30, 30, "");
    let mut out_b = Output::new(200, 200, 30, 30, "");

    out_r.set_text_size(14);
    out_g.set_text_size(14);
    out_b.set_text_size(14);

    wind.show();
    but.set_callback(move|| {
        if let Some(file) = file_chooser("Choose a file", "*.jpg", ".", true) {
            let con = &file[..];
            if Path::new(con).exists() {
                let rgbs = read_pix_from_file(con);
                println!("{} {} {}", rgbs.0, rgbs.1, rgbs.2);


                let out_r_string = rgbs.0.to_string();
                let out_g_string = rgbs.1.to_string();
                let out_b_string = rgbs.2.to_string();
                let out_r_sliced=&out_r_string;
                let out_g_sliced=&out_g_string;
                let out_b_sliced=&out_b_string;
                out_r.set_value(out_r_sliced);
                out_g.set_value(out_g_sliced);
                out_b.set_value(out_b_sliced);
            }
        }
    });
    wind.redraw();
    wind.end();

    app.run().unwrap();
}

pub fn read_pix_from_file(filename: &str) -> (u32, u32, u32) {
    let im = image::open(filename).unwrap();
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
    println!("{} ", filename);
    (r_sum / pixels, g_sum / pixels, b_sum / pixels)
}
