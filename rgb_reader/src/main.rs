extern crate image;

use crate::image::GenericImageView;
use std::path::Path;

use walkdir::WalkDir;
use iced::{button, Button, Column,Element, Text,Sandbox, Settings};

fn main()  ->iced::Result {
    Processor::run(Settings::default())
}

pub fn perform_on_all_paths() {
    let images_paths = paths_of_images_from_resources();
    for path in images_paths {
        let con = &path[..];
        if Path::new(&con).exists() {
            read_pix_from_file(&con);
        }
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
    println!("{} ", filename);
    println!("{} {} {} ",r_sum / pixels, g_sum / pixels, b_sum / pixels );
    (r_sum / pixels, g_sum / pixels, b_sum / pixels)
}

fn paths_of_images_from_resources() -> Vec<String> {
    let mut jpegs: Vec<String> = vec![];
    for entry in WalkDir::new("resources//").contents_first(true) {
        let entry = entry.unwrap();
        if entry
            .path()
            .display()
            .to_string()
            .to_string()
            .contains("jpeg")
            || entry
                .path()
                .display()
                .to_string()
                .to_string()
                .contains("jpg")
            || entry
                .path()
                .display()
                .to_string()
                .to_string()
                .contains("JPEG")
            || entry
                .path()
                .display()
                .to_string()
                .to_string()
                .contains("JPG")
        {
            jpegs.push(entry.path().display().to_string());
        }
    }
    jpegs
}

#[derive(Default)]
struct Processor {
    value: String,
    processing_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PerformProcessing,
}

impl Sandbox for Processor {
    type Message = Message;
    fn new() -> Self {
        Self::default()
    }
    
    fn title(&self) -> String {
        String::from("RGB Image Processor.")
    }

    fn view(&mut self) -> Element<Message> {
        Column::new().push(Button::new(&mut self.processing_button, Text::new("Process")).on_press(Message::PerformProcessing),).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PerformProcessing => {
                perform_on_all_paths();
            }
        }
    }
}