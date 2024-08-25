pub trait UI {
    fn handl_events(&mut self);
    fn clear(&mut self, color: crate::draw::color::Color);
    fn set_color(&mut self, color: crate::draw::color::Color);
    fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32);
    fn draw_pixel(&mut self, x: i32, y: i32);
    fn run(&self) -> bool;
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}
