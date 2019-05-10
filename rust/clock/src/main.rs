extern crate clock;
use clock::Clock;

fn main() {
    

    let clock = Clock::new( 5, 23 );

    println!("Debug clock {:?}", clock);
    println!("Format clock {}", clock);

    let clock2 = Clock::new(23, 59).add_minutes(2);
    assert_eq!(clock2.to_string(), "00:01");

    let clock3 = Clock::new(0, 3).add_minutes(-4);
    assert_eq!(clock3.to_string(), "23:59");

    println!("Clock Clock::new(25, 0) == 01:00 ===> {}", Clock::new(25, 0).to_string());
    println!("Clock Clock::new(100, 0) == 04:00 ===> {}", Clock::new(100, 0).to_string());
    println!("Clock Clock::new(0, 1723) == 04:43 ===> {}", Clock::new(0, 1723).to_string());
    println!("Clock Clock::new(-25, -160) == 20:20 ===> {}", Clock::new(-25, -160).to_string());

}