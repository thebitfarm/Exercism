pub fn verse(n: i32) -> String {
    unimplemented!("emit verse {}", n)

    match n {
        x if x == 0 => "No more bottles of beer on the wall, no more bottles of beer."
    }

    m if m.len() == 0 => "Fine. Be that way!",
        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever."

}

/*
2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.

1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.

No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
*/

pub fn sing(start: i32, end: i32) -> String {


    unimplemented!("sing verses {} to {}, inclusive", start, end)
}
