pub fn abbreviate(input: &str) -> String {
    let mut s = String::from(" ");
    s.push_str(input);

    s.chars().
        collect::<Vec<_>>().
        windows(2).
        filter(|pair|
            (!pair[0].is_alphanumeric() && pair[1].is_alphanumeric()) ||
            (!pair[0].is_uppercase() && pair[1].is_uppercase())
        ).
        map(|pair| pair[1]).
        collect::<String>().
        to_uppercase()
}