use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    /*
    let mut sum:u32 = 0;
    let mut uniq:HashSet<u32> = HashSet::new();

    for x in 1..limit {
        //println!("x[{}]", x);
        for f in factors.iter().filter(|&i| *i != 0u32 ) {
            if x % f == 0 && !uniq.contains(&x) {
                uniq.insert(x);
                sum += x;
                //println!("found x[{}] multiple of f[{}], running sum[{}]", x, f, sum);
            }
        }
    }

    sum
    */
    (1..limit).filter(|i| factors.iter().any(|f| *f != 0u32 && i % f == 0 )).sum()
}
