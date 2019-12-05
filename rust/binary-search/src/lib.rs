use std::f32;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut currslice = array;
    let mut done = false;
    let mut accum : usize = 0;

    while !done {
        let midpoint : usize = match currslice.len() == 1 {
            true => 0,
            false => f32::floor((currslice.len() as f32) / 2 as f32) as usize - 1
        };

        if currslice[midpoint] == key {
            return Some(midpoint + accum);
        } else if currslice[midpoint] > key {
            if currslice.len() <= 1 {
                done = true;
            } else {
                currslice = &currslice[0..midpoint+1];
            }
        } else {
            if currslice.len() <= 1 {
                done = true;
            } else {
                currslice = &currslice[midpoint+1..currslice.len()];
                accum = accum + (midpoint+1);
            }
        }

    }

    None
}
