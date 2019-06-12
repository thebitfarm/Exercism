//#[macro_use]
extern crate etl;



fn main() {

    let input = (&[(1, vec!['A', 'E', 'I', 'O', 'U'])]).iter().cloned().collect();



    println!("Result for &[(1, vec!['A', 'E', 'I', 'O', 'U'])]:: {:?}", etl::transform(&input));


}