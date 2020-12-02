use super::Point;


#[derive(Debug)]
pub struct Primitive {
    start_point: Point,
    height: usize,
    long: usize,
    rigth: bool,
}


impl Primitive {
    pub fn new(start_point: Point, height: usize, long: usize, rigth: bool) -> Self {
        Self {
            start_point: start_point,
            height: height,
            long: long,
            rigth: rigth,
        }
    }

    pub fn calc_points(&self) -> Vec<Point> {
        let angle = if self.rigth { 30 } else { 150 };
        let mut points = Vec::new();

        let mut next_point = self.calc_next_point(self.start_point, angle, self.long);
        
        points.push(self.start_point.clone());
        points.push(next_point);

        next_point = self.calc_next_point(next_point, 90, self.height);
        points.push(next_point);

        next_point = self.calc_next_point(self.start_point, 90, self.height);
        points.push(next_point);

        points
    }

    // On input - old point, angle in usize (degrees for useful)
    fn calc_next_point(&self, point: Point, angle: usize, long: usize) -> Point {
        let long_f = long as f64;
        let angle_rad = (angle as f64).to_radians();
        let y = (long_f * angle_rad.sin()) as usize + point.y;
        let x = (long_f * angle_rad.cos()) as usize + point.x;

        Point::new(x, y)
    }
}
