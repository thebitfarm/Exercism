extern crate sum_of_multiples;
use sum_of_multiples::*;


fn main() {

    //println!("sum_of_multiples(1, &[3, 5]) = {}", sum_of_multiples(1, &[3, 5]));
    //println!("sum_of_multiples(4, &[3, 5]) = {}", sum_of_multiples(4, &[3, 5]));
    println!("sum_of_multiples(10, &[3, 5]) = {}", sum_of_multiples(10, &[3, 5]));  // 23
    println!("sum_of_multiples(1000, &[3, 5]) = {}", sum_of_multiples(1000, &[3, 5]));  // 233168
    println!("sum_of_multiples(4, &[3, 0]) = {}", sum_of_multiples(4, &[3, 0])); // 3
    

}