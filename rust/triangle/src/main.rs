extern crate triangle;
use triangle::*;

pub fn main() {
    println!("Triangle::build([4, 3, 4]), result: {:?}", Triangle::build([4, 3, 4]));
    let triangle = Triangle::build([10, 10, 10]).unwrap();
    println!("Triangle::build([10, 10, 10]), result: {:?}", triangle);
    println!("Triangle::build([10, 10, 10]), is_equilateral() result: {:?}", triangle.is_equilateral());
}