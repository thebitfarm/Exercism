extern crate pascals_triangle;
use pascals_triangle::*;


fn main() {
    let pt = PascalsTriangle::new(1);
    let expected: Vec<Vec<u32>> = vec![vec![1]];
    println!("One::: Expected={:?}, result={:?}", expected, pt.rows());

    let pt2 = PascalsTriangle::new(2);
    let expected2: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
    println!("Two::: Expected={:?}, result={:?}", expected2, pt2.rows());

    let pt3 = PascalsTriangle::new(3);
    let expected3: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
    println!("Three::: Expected={:?}, result={:?}", expected3, pt3.rows());
}