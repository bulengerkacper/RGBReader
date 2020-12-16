extern crate image;

use fltk::*;

pub mod analysis;
use analysis::RgbData;

use fltk::{app::*, button::*, dialog::*, output::*, window::*};
use std::path::Path;

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
    but.set_callback(move || {
        if let Some(file) = file_chooser("Choose a file", "*.jpg", ".", true) {
            let con = &file[..];
            if Path::new(&con).exists() {
                let rgb_data = RgbData::default();
                let rgbs = rgb_data.count_avgs(&con);
                let (out_r_string, out_g_string, out_b_string) = (
                    &rgbs.0.to_string(),
                    &rgbs.1.to_string(),
                    &rgbs.2.to_string(),
                );
                let (out_r_sliced, out_g_sliced, out_b_sliced) =
                    (&out_r_string, &out_g_string, &out_b_string);

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
