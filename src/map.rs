// just need map to be something that holds boundaries
// needs to check validity of moves

use npc_engine_utils::{Coord2D, DirectionConverterYDown};


#[derive(Debug, Clone, PartialEq)]
pub struct Map{
    height: u32,
    width: u32,
    center: Coord2D,
    longest_dist: f32,
}

impl Map {
    pub fn new(height: u32, width: u32) -> Self {
        // for an easier time right now, make the side lengths even
        let height = if height % 2 == 1 {height} else {height + 1};
        let width = if width % 2 == 1 {width} else {width + 1};
        let center = Coord2D::new((width/2) as i32, (height/2) as i32);
        let longest_dist = ((center.x.pow(2) + center.y.pow(2)) as f32).sqrt();
        Self{height, width, center, longest_dist}
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn longest_dist(&self) -> f32 {
        self.longest_dist
    }
    pub fn distance_from_center(&self, location: Coord2D) -> f32 {
        let dist = self.center.abs_diff(&location);
        ((dist.x.pow(2) + dist.y.pow(2)) as f32).sqrt()
    }
    pub fn distance_points(&self, location: Coord2D) -> f32 {
        // want to flip the distance from center so that being in the center gives you points
        // y = mx + q where q is farthest distance possible on map and m is -1
        -self.distance_from_center(location) + self.longest_dist
    }
    pub fn size(&self) -> Coord2D {
        Coord2D { x: self.width as i32, y: self.height as i32}
    }
    pub fn out_of_bounds(&self, coord: Coord2D) -> bool {
        coord.x < 0 || coord.x >= (self.width as i32) || coord.y < 0 || coord.y >= (self.height as i32)
    }
}

// up is positive
pub type DirConv = DirectionConverterYDown;
