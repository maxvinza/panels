use drawing::{
    Primitive,
    Point
};

mod drawing;


fn main() {
    let point = Point::new(1, 1);
    let primitive = Primitive::new(point, 10, 10, true);
    println!("{:?}", primitive.calc_points());
}
