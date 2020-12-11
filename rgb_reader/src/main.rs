extern crate image;

use crate::image::GenericImageView;
use std::path::Path;

use orbtk::prelude::*;
use walkdir::WalkDir;

fn main() {
    gui_handle();
    walk_through_dirs();
    let path = "resources\\nosacz.jpg";
    if Path::new(path).exists() {
        read_pix_from_file(path);
    }
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
    // println!("{} {} {} ",r_sum / pixels, g_sum / pixels, b_sum / pixels );
    (r_sum / pixels, g_sum / pixels, b_sum / pixels)
}

pub fn walk_through_dirs() {
    for entry in WalkDir::new("resources\\") {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
}
pub fn gui_handle() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::new().text("OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
