use crate::animation::Animation;
use crate::animation::Rotation;

pub struct Sprite {
    text: Vec<String>,
    default_text_color: String,
    default_background_color: String,
    text_color: String,
    background_color: String,
    x: u32,
    y: u32,
    z: u32,
    rotation: Rotation
}

impl Sprite {
    fn new() -> Sprite {
        Sprite {
            text: Vec::new(),
            default_text_color: String::from("\x1b[38;2;255;255;255m"),
            default_background_color: String::from("\x1b[48;2;0;0;0m"),
            text_color: String::from("\x1b[38;2;255;255;255m"),
            background_color: String::from("\x1b[48;2;0;0;0m"),
            x: 0,
            y: 0,
            z: 0,
            rotation: Rotation::First
        }
    }

    fn get_text<'a>(mut self) -> &'a mut Vec<String> {
        &mut self.text
    }

    fn set_text_color(mut self, color: String) -> Sprite {
        self.text_color = color;
        self
    }

    fn set_background_color(mut self, color: String) -> Sprite {
        self.background_color = color;
        self
    }

    fn get_position(self) -> (u32, u32, u32) {
        (self.x, self.y, self.z)
    }
}

impl Animation for Sprite {
    fn next_cell(mut self) {
        todo!()
    }

    fn prev_cell(mut self) {
        todo!()
    }

    fn get_num_cells(self) -> u32 {
        todo!()
    }

    fn set_current_cell(mut self, cell: u32) {
        todo!()
    }

    fn set_rotation(mut self, rot: Rotation) {
        self.rotation = rot;
    }
}