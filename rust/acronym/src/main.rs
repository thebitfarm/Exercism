extern crate acronym;

fn main() {
    println!("Hello {}", "world");
    println!("acronym::abbreviate(\"Portable Network Graphics\"), result:: {:?}", acronym::abbreviate("Portable Network Graphics"));;
    println!("emtpy call, result:: {:?}", acronym::abbreviate(""));
    println!("acronym::abbreviate(\"Ruby on Rails\"), result:: {:?}", acronym::abbreviate("Ruby on Rails"))
}