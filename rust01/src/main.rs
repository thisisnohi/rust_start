mod content;
mod text;

use crate::content::Content;
use clap::Parser;
use csscolorparser::Color;
use rusttype::Font;
use std::fs;

#[derive(Parser, Debug)]
pub struct Parameter {
    #[arg(long, default_value_t = 1920)]
    pub image_width: u32,
    #[arg(long, default_value_t = 1080)]
    pub image_height: u32,
    #[arg(long, default_value_t = 300)]
    pub padding: u32,
    #[arg(long, default_value = "#FFFFFFFF")]
    pub background_color: Color,
    #[arg(short, long)]
    pub input_path: String,
    #[arg(short, long)]
    pub output_path: String,
}

fn main() {
    println!("Hello, world!");

    let params = Parameter::parse();
    dbg!(&params);

    let font = load_font("assets/JetBrainsMono-Regular.ttf");
    let cotent = Content::new(&params.input_path);
    dbg!(&cotent);

    let canvas = ImageCanvas::new();
}

fn load_font(path: &str) -> Font {
    let bytes: Vec<u8> = fs::read(path).expect("File not found");
    Font::try_from_vec(bytes).expect("fail to create font")
}
