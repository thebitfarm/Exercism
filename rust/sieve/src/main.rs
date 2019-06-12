extern crate sieve;

fn main() {
    println!("Hello {}", "world");

    println!("sieve::primes_up_to(2)=[2], result: {:?}", sieve::primes_up_to(2));
    println!("sieve::primes_up_to(10)=[2, 3, 5, 7], result: {:?}", sieve::primes_up_to(10));
    println!("sieve::primes_up_to(13)=[2, 3, 5, 7, 11, 13], result: {:?}", sieve::primes_up_to(13));
    


}