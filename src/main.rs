use image::RgbImage;

use drawing::{
    Drawer,
    Primitive,
    PrimitiveType,
    Point
};

mod drawing;


fn main() {
    let mut img = RgbImage::new(800, 800);
    let point = Point::new(1, 1);

    let ptype = PrimitiveType::Left;
    let primitive = Primitive::new(point, 10, 10, ptype);
    let mut drawer = Drawer::new(primitive.calc_points());
    println!("{:?}", primitive.calc_points());

    let ptype = PrimitiveType::Right;
    let primitive = Primitive::new(point, 10, 10, ptype);
    println!("{:?}", primitive.calc_points());

    let ptype = PrimitiveType::Flat;
    let primitive = Primitive::new(point, 10, 10, ptype);
    println!("{:?}", primitive.calc_points());
    img.save("out.png").unwrap();
}
