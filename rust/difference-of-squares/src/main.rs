extern crate difference_of_squares;
use difference_of_squares as squares;


fn main() {
    let n:u32 = 3;
    println!("square_of_sum({}) = {}", n, squares::square_of_sum(n));
    println!("sum_of_squares({}) = {}", n, squares::sum_of_squares(n));
}