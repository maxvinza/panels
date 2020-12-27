use image::RgbImage;

mod point;
mod primitive;


pub use {
    point::Point,
    primitive::{
        PrimitiveType,
        Primitive,
    }
};


pub struct Drawer {
    points: Vec<Point>,
}


impl Drawer {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            points: points
        }
    }

    pub fn draw(&self, image: &mut RgbImage) {
        //
    }
}
