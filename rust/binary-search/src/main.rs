extern crate binary_search;
use binary_search as bs;

fn main () {
    println!("Hello {:?}", "world");

    //println!("bs::find(&[6], 6), result:: {:?}", bs::find(&[6], 6));
    //println!("bs::find(&[1, 2], 1), result:: {:?}", bs::find(&[1, 2], 1));
    //println!("bs::find(&[1, 2], 2), result:: {:?}", bs::find(&[1, 2], 2));
    //println!("bs::find(&[1, 3, 4, 6, 8, 9, 11], 6), expected::Some(3), result::{:?}", bs::find(&[1, 3, 4, 6, 8, 9, 11], 6));
    //println!("bs::find(&[1, 3, 4, 6, 8, 9, 11], 1), expected::Some(0), result::{:?}", bs::find(&[1, 3, 4, 6, 8, 9, 11], 1));
    println!("bs::find(&[1, 3, 4, 6, 8, 9, 11], 0), expected::None, result::{:?}", bs::find(&[1, 3, 4, 6, 8, 9, 11], 0));
    //println!("bs::find(&[1, 3, 4, 6, 8, 9, 11], 11), expected::Some(6), result::{:?}", bs::find(&[1, 3, 4, 6, 8, 9, 11], 11));
    //println!("bs::find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144), expected::Some(9), result::{:?}", 
    //        bs::find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144));
    
}