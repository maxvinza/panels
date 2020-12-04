use super::Point;


/// This emun contain angles
/// ```ignore
///      +-------------+--------------------------------+------------------------------+
///      | First point | Second point(relatively first) | Third point(relatively zero) |
///      +-------------+--------------------------------+------------------------------+
/// Left | 0x096       | 05A                            | 05A                          |
///      +-------------+--------------------------------+------------------------------+
/// ```
#[derive(Debug, Copy, Clone)]
pub enum PrimitiveType {
    Left = 0x09605A05A,
    Right = 0x01E05A05A,
    Flat = 0x09601E096
}


#[derive(Debug)]
pub struct Primitive {
    start_point: Point,
    height: usize,
    long: usize,
    primitive_type: PrimitiveType,
}


impl Primitive {
    pub fn new(start_point: Point, height: usize, long: usize, ptype: PrimitiveType) -> Self {
        Self {
            start_point: start_point,
            height: height,
            long: long,
            primitive_type: ptype,
        }
    }

    pub fn calc_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let mut next_point = self.calc_next_point(self.start_point, self.angle(2), self.long);
        
        points.push(self.start_point.clone());
        points.push(next_point);

        next_point = self.calc_next_point(next_point, self.angle(1), self.height);
        points.push(next_point);

        next_point = self.calc_next_point(self.start_point, self.angle(0), self.height);
        points.push(next_point);

        points
    }

    fn angle(&self, number: usize) -> u16 {
        let angles = self.primitive_type as usize;
        let shift = (angles >> (number * 24)) & 0xFFF;
        shift as u16
    } 

    // On input - old point, angle in usize (degrees for useful)
    fn calc_next_point(&self, point: Point, angle: u16, long: usize) -> Point {
        let long_f = long as f64;
        let angle_rad = (angle as f64).to_radians();
        let y = (long_f * angle_rad.sin()) as usize + point.y;
        let x = (long_f * angle_rad.cos()) as usize + point.x;

        Point::new(x, y)
    }
}
