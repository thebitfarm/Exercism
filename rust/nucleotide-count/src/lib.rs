extern crate itertools;
use std::collections::HashMap;
use itertools::Itertools;

pub fn check_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false 
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match check_nucleotide(nucleotide) {
        false => Err(nucleotide),
        true => dna.chars().into_iter()
            .map(|c| 
                match check_nucleotide(c) {
                    true => Ok(c),
                    false => Err(c)
                })
            .fold_results(0, |accum, ele| {
                if nucleotide == ele {
                    accum + 1 
                } else {
                    accum
                }})
    }


}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    
    let mut count_map = HashMap::new();
    count_map.insert('A',0);
    count_map.insert('C',0);
    count_map.insert('G',0);
    count_map.insert('T',0);

    match dna.chars().into_iter()
        .map(|c| 
            match check_nucleotide(c) {
                true => Ok(c),
                false => Err(c)
            })
        .fold_results(0, |accum, c| {
            let count = count_map.entry(c).or_insert(0);
            *count += 1;
            accum + 1
    }) {
        Ok(_) => Ok(count_map),
        Err(whoops) => Err(whoops),
    }
    
}
