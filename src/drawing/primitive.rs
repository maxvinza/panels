use super::Point;


pub struct Primitive {
    start_point: Point,
    height: usize,
    long: usize,
}


impl Primitive {
    pub fn new(start_point: Point, height: usize, long: usize) -> Self {
        Self {
            start_point: start_point,
            height: height,
            long: long,
        }
    }

    pub fn calc_points(self) -> Vec<Point> {
        let mut points = Vec::new();
        points.push(self.start_point);

        points
    }
}
