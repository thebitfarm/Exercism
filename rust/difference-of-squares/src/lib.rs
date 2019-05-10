pub fn square_of_sum(n: u32) -> u32 {
    // (1 + 2 + ... + 10)² = 55² = 3025.
    //let ret:u32 = (1..n+1).sum();
    let ret:u32 = match n {
        a if a % 2 == 0 => (a+1) * (a/2),
        a  => (a+1) * ((a-1)/2) + ((a+1)/2)
    };
    ret * ret
}

pub fn sum_of_squares(n: u32) -> u32 {
    //  1² + 2² + ... + 10² = 385.
    let ret:u32 = (1..n+1).map(|x| x*x).sum();
    ret
}

pub fn difference(n: u32) -> u32 {

    println!("square_of_sum({}) = {}", n, square_of_sum(n));

    // 4 -> 10
    // 5 -> 15
    // 6 -> 21
    // 6 + 5 + 4 + 3 + 2 + 1
    // even (n+1) * (n/2)
    // 5 + 4 + 3 + 2 + 1
    // odd (n+1) * ((n-1)/2) + ((n+1)/2)


    square_of_sum(n) - sum_of_squares(n)
}
