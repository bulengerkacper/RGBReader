extern crate image;

use crate::image::GenericImageView;
use std::path::Path;

use fltk::{app::*, button::*,menu::*, dialog::*, window::*};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "R G B Image Processor.");
    let mut but = Button::new(160, 210, 80, 40, "Process it!");
    wind.end();
    wind.show();
    but.set_callback(|| {
        if let Some(file) = file_chooser("Choose a file", "*.jpg", ".", true) {
            let con = &file[..];
            let popup_options = vec!["OK"];
            let mut menu_item = MenuItem::new(&popup_options);
            match menu_item.popup(100,100) {
                None => println!("None"),
                Some(val) => println!("{}", val.label().unwrap()),
            }
            if Path::new(con).exists() {
                let rgbs = read_pix_from_file(con);
            }
        }
    });
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
