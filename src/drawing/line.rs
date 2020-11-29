use super::Point;


#[derive(Default)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}


impl Line {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        let start = Point::new(x1, y1);
        let end = Point::new(x2, y2);

        Self {
            start: start,
            end: end,
        }
    }
}