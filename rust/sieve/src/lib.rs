use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {

    // for to mark non-prime into hashset
    let limit_p = (upper_bound as f64).sqrt().round() as u64;

    let sieve : HashSet<u64> = (2..(1+limit_p)).flat_map(|n| {
        (n..).map(|x| x*n).take_while(|&m| m <= upper_bound).collect::<HashSet<u64>>()
    }).collect::<HashSet<u64>>();

    // for to loop to upper_bound and any not in hashet gets pushed into result
    (2..(1+upper_bound)).filter(|x| !sieve.contains(x)).collect::<Vec<u64>>()
}
