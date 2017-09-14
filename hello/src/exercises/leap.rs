pub fn is_leap_year(year: u32) -> bool {
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn is_leap_year_origin(year: i32) -> bool {
    let has_factor = |n| year % n == 0;
    has_factor(4) && (!has_factor(100) || has_factor(400))
}
