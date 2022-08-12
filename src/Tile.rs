pub struct Tile {
    pub x: i8,
    pub y: i8,
    pub val: u8,
}
impl Tile {
    pub fn new(x: i8, y: i8, val: u8) -> Self {
        Self { x, y, val }
    }
}
