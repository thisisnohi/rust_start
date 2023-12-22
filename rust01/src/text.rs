use csscolorparser::Color;
use imageproc::drawing::text_size;
use rusttype::{Font, Scale};

#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub font_color: Color,
    pub background_color: Color,
}

impl Text {
    pub(crate) fn new() -> Text {
        Text {
            content: "".to_string(),
            font_size: 120f32,
            font_color: Color::from([0, 0, 0]),
            background_color: Color::from([255, 255, 255]),
        }
    }

    pub fn scale(&self) -> Scale {
        Scale::uniform(self.font_size)
    }

    pub fn size(&self, font:&Font) -> (u32,u32) {
        let (w,h) = text_size(self.scale(), &font, self.content.replace(" ","0").as_str());
            (w as u32,h as u32)
    }

}
}
