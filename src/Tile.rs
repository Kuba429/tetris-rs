pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub val: u8,
}
impl Tile {
    pub fn new(x: u8, y: u8, val: u8) -> Self {
        Self {
            x: x,
            y: y,
            val: val,
        }
    }
}
