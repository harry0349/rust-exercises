#[macro_use]
mod exercises;

extern crate chrono;

use exercises::gigasecond;
use exercises::leap;
use exercises::raindrops;
use exercises::nth_prime;
use exercises::bob;
use exercises::beer_song;
use exercises::proverb;
use exercises::difference_of_squares;
use chrono::*;

fn main() {
    let start_date = Utc.ymd(2017, 9, 12).and_hms(0, 0, 0);
    let after = gigasecond::after(start_date);
    println!("{}", after);

    let result = leap::is_leap_year(2100);
    println!("{}", result);

    let drop = raindrops::raindrops(35);
    println!("{}", drop);

    println!("{:?}", nth_prime::nth(4));

    println!("{}", bob::reply(""));

    println!("{}", beer_song::sing(3, 0));
    //    let list: Vec<&str> = vec!['nail', 'shoe', 'horse', 'war'];
    let list: Vec<&str> = vec!["nail", "shoe", "horse", "war"];
    println!("{}", proverb::build_proverb(list));

    println!("{}", difference_of_squares::square_of_sum(100));
    println!("{}", difference_of_squares::sum_of_squares(100));
    println!("{}", difference_of_squares::difference(5));
}