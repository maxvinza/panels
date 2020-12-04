use drawing::{
    Primitive,
    PrimitiveType,
    Point
};

mod drawing;


fn main() {
    let point = Point::new(1, 1);

    let ptype = PrimitiveType::Left;
    let primitive = Primitive::new(point, 10, 10, ptype);
    println!("{:?}", primitive.calc_points());

    let ptype = PrimitiveType::Right;
    let primitive = Primitive::new(point, 10, 10, ptype);
    println!("{:?}", primitive.calc_points());

    let ptype = PrimitiveType::Flat;
    let primitive = Primitive::new(point, 10, 10, ptype);
    println!("{:?}", primitive.calc_points());
}
