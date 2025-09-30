#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl From<i64> for Position {
    fn from(value: i64) -> Self {
        Self {
            x: (value >> 38) as i32,
            y: (value << 52 >> 52) as i32,
            z: (value << 26 >> 38) as i32,
        }
    }
}

impl From<Position> for i64 {
    fn from(pos: Position) -> Self {
        let x = pos.x as i64;
        let y = pos.y as i64;
        let z = pos.z as i64;
        ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF)
    }
}
