extern crate nucleotide_count;
use nucleotide_count as dna;


fn main() {
    //let pt = PascalsTriangle::new(1);
    //let expected: Vec<Vec<u32>> = vec![vec![1]];
    //println!("One::: Expected={:?}, result={:?}", expected, pt.rows());

    println!("check_nucleotide('A'), result={:?}", dna::check_nucleotide('A'));
    println!("check_nucleotide('X'), result={:?}", dna::check_nucleotide('X'));

    println!("dna::count('A', \"\"), Expected={:?}, result={:?}", 0, dna::count('A', ""));
    println!("dna::count('C', \"CCCCC\"), Expected={:?}, result={:?}", 5, dna::count('C', "CCCCC").unwrap());
    println!("dna::count('C', \"CCACCC\"), Expected={:?}, result={:?}", 5, dna::count('C', "CCACCC").unwrap());
    println!("dna::count('A', \"CCACCC\"), Expected={:?}, result={:?}", 1, dna::count('A', "CCACCC").unwrap());
    println!("dna::count('X', \"\"), Expected={:?}, result={:?}", 0, dna::count('A', "AX"));
    println!("dna::count('X', \"A\"), Expected={:?}, result={:?}", Err::<(),char>('X'), dna::count('X', "A"));

    println!("dna::nucleotide_counts(\"ACGT\"), result={:?}", dna::nucleotide_counts("ACGT").unwrap());
    println!("dna::nucleotide_counts(\"ACXGT\"), result={:?}", dna::nucleotide_counts("ACXGT"));
    assert!(dna::count('A', "").is_ok());

    println!("Hello World");
}