#[allow(dead_code)]
pub fn verse(n: u8) -> String {
    format!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.", n, n, n - 1)
}

pub fn verse_origin(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
              Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
              Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        n if n > 2 && n <= 99 =>
            format!(
                "{n} bottles of beer on the wall, {n} bottles of beer.\n\
                 Take one down and pass it around, {n_minus_1} bottles of beer on the wall.\n",
                n = n,
                n_minus_1 = n -1),
        _ => panic!(),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end .. start + 1).rev().map(|n| verse_origin(n)).collect::<Vec<_>>().join("\n")
}