#[derive(Copy, Clone)]
pub enum Vec2 {
    Screen(i32, i32),
}

pub fn to_screen2(coord: Vec2) -> Option<(i32, i32)> {
    return match coord {
        Vec2::Screen(x, y) => Some((x, y)),
    };
}
