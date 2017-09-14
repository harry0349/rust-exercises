#[macro_use]
mod exercises;

extern crate chrono;

use exercises::gigasecond;
use exercises::leap;
use exercises::raindrops;
use exercises::nth_prime;
use exercises::bob;
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
}
