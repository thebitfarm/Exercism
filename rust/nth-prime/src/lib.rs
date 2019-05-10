pub fn nth(n: u32) -> Option<u32> {
    let mut ret:Option<u32> = None;
    let target:i32 = n as i32;
    let mut count:i32 = 0;
    let mut curr:i32 = 1;

    println!("Result of isprime: {}", is_prime( n as i32 ) );
    
    let mut done = false;

    if target == 0 {
        return None;
    }
    if target == 1 {
        return None;
    }

    while !done {
        //println!("checking {}", curr);

        if is_prime( curr ) {
            //println!("\t found prime!");
            count += 1;

            if count >= target {
                done = true;

            } 
        }
        
        if !done {
            curr += 1;
        }

    }
    if done {
        ret = Some(curr as u32)
    }

    return ret;
}

pub fn is_prime(n: i32) -> bool {
    let mut ret:bool = true;
    if n == 1 || n == 0 {
        return false;
    }
    if n == 2 {
        return ret;
    }

    let upper:i32 = 1 + (n / 2);

    for x in 2..upper {
        if n % x == 0 {
            ret = false;
        }
    }

    return ret;
}
