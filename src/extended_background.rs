pub trait ExtendedBackground {
    fn get_dimensions(self) -> (u32, u32);
    fn set_offset(mut self, x: u32, y: u32);
    fn set_offset_inbounds(mut self, x: u32, y: u32);
}