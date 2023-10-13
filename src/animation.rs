pub trait Animation {
    fn next_cell(mut self);
    fn prev_cell(mut self);
    fn get_num_cells(self) -> u32;
    fn set_current_cell(mut self, cell: u32);
    fn set_rotation(mut self, rot: Rotation);
}

pub enum Rotation {
    First,
    Second,
    Third,
    Fourth
}