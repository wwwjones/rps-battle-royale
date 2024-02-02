// just need map to be something that holds boundaries
// needs to check validity of moves

use npc_engine_utils::{Coord2D, DirectionConverterYUp};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map{
    height: u32,
    width: u32,
}

impl Map {
    pub fn new(height: u32, width: u32) -> Self {
        // for an easier time right now, make the side lengths even
        let height = if height % 2 == 0 {height} else {height + 1};
        let width = if width % 2 == 0 {width} else {width + 1};
        Self{height, width}
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn out_of_bounds(&self, coord: Coord2D) -> bool {
        let quadrant_height = self.height/2;
        let quadrant_width = self.width/2;
        (coord.x.abs() as u32) < quadrant_width && (coord.y.abs() as u32) < quadrant_height
    }
}

// up is positive
pub type DirConv = DirectionConverterYUp;
