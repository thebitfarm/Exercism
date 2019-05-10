pub fn reverse(input: &str) -> String {
    let mut ret : String = String::new();

    for c in input.chars() {
        println!("char is {}", c);
        ret.insert(0, c);
    }

    return ret;
}
