use crate::extended_background::ExtendedBackground;

pub struct Layer {
    text: Vec<String>,
    default_text_color: String,
    default_background_color: String,
    text_color: String,
    background_color: String
}

impl Layer {
    fn new() -> Layer {
        Layer {
            text: Vec::new(),
            default_text_color: String::from("\x1b[38;2;255;255;255m"),
            default_background_color: String::from("\x1b[48;2;0;0;0m"),
            text_color: String::from("\x1b[38;2;255;255;255m"),
            background_color: String::from("\x1b[48;2;0;0;0m")
        }
    }

    fn get_text<'a>(mut self) -> &'a mut Vec<String> {
        &mut self.text
    }

    fn set_text_color(mut self, color: String) -> Layer {
        self.text_color = color;
        self
    }

    fn set_background_color(mut self, color: String) -> Layer {
        self.background_color = color;
        self
    }
}

impl ExtendedBackground for Layer {
    fn get_dimensions(self) -> (u32, u32) {
        todo!()
    }

    fn set_offset(mut self, x: u32, y: u32) {
        todo!()
    }

    fn set_offset_inbounds(mut self, x: u32, y: u32) {
        todo!()
    }
}