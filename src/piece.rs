use crate::tile::Tile;

pub fn get_shape(x: u8, y: u8, template: Vec<u8>) -> Vec<Option<Tile>> {
    return match template.len() {
        9 => template
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                if *val == 0 {
                    return None;
                }
                return match idx {
                    0 => Some(Tile::new(x - 1, y - 1, *val)),
                    1 => Some(Tile::new(x, y - 1, *val)),
                    2 => Some(Tile::new(x + 1, y - 1, *val)),
                    3 => Some(Tile::new(x - 1, y, *val)),
                    4 => Some(Tile::new(x, y, *val)),
                    5 => Some(Tile::new(x + 1, y, *val)),
                    6 => Some(Tile::new(x - 1, y + 1, *val)),
                    7 => Some(Tile::new(x, y + 1, *val)),
                    8 => Some(Tile::new(x + 1, y + 1, *val)),
                    _ => None,
                };
            })
            .collect(),
        16 => template
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                if *val == 0 {
                    return None;
                }
                return match idx {
                    0 => Some(Tile::new(x - 1, y - 1, *val)),
                    1 => Some(Tile::new(x, y - 1, *val)),
                    2 => Some(Tile::new(x + 1, y - 1, *val)),
                    3 => Some(Tile::new(x + 2, y - 1, *val)),
                    4 => Some(Tile::new(x - 1, y, *val)),
                    5 => Some(Tile::new(x, y, *val)),
                    6 => Some(Tile::new(x + 1, y, *val)),
                    7 => Some(Tile::new(x + 2, y, *val)),
                    8 => Some(Tile::new(x - 1, y + 1, *val)),
                    9 => Some(Tile::new(x, y + 1, *val)),
                    10 => Some(Tile::new(x + 1, y + 1, *val)),
                    11 => Some(Tile::new(x + 2, y + 1, *val)),
                    12 => Some(Tile::new(x - 1, y + 2, *val)),
                    13 => Some(Tile::new(x, y + 2, *val)),
                    14 => Some(Tile::new(x + 1, y + 2, *val)),
                    15 => Some(Tile::new(x + 2, y + 2, *val)),
                    _ => None,
                };
            })
            .collect(),
        _ => panic!("Unhandled shape template"),
    };
}

pub enum Direction {
    RIGHT,
    LEFT,
    BOTTOM,
}
