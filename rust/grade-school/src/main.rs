extern crate grade_school;
use grade_school as school;

fn main () {

    println!("Hello {:?}", "world");

    let mut s = school::School::new();
    s.add(2, "Aimee");
    s.add(2, "Dillon");
    s.add(3, "Bobo");
    println!("School::new(), result: {:?}", s);
    println!("s.grades(), expected:: vec![2,3], result:: {:?}", s.grades());
    println!("s.grade(2), expected:: some_strings(&[\"Aimee\",\"Dillon\"], result:: {:?}", s.grade(2));
}