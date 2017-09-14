//todo Convert a number to a string, the contents of which depend on the number's factors.
pub fn raindrops(rain: u32) -> String {
    let mut drops = String::new();
    if rain % 3 == 0 {
        drops.push_str("Pling");
    } else if rain % 5 == 0 {
        drops.push_str("Plang");
    } else if rain % 7 == 0 {
        drops.push_str("Plong");
    } else {
        let rain = format!("{}", rain);
        drops.push_str(&rain);
    }
    drops
}

#[allow(dead_code)]
pub fn raindrops_origin(n: usize) -> String {
    let is_pling = |n| n % 3 == 0;
    let is_plang = |n| n % 5 == 0;
    let is_plong = |n| n % 7 == 0;
    let mut drops = String::new();
    if is_pling(n) {
        drops.push_str("Pling");
    }
    if is_plang(n) {
        drops.push_str("Plang");
    }
    if is_plong(n) {
        drops.push_str("Plong");
    }
    if drops.is_empty() {
        let s = format!("{}", n);
        drops.push_str(&s);
    }
    drops
}
