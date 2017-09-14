//pub fn square_of_sum(mut n: i32) -> i32 {
//    let mut sum = 0;
//    while n > 0 {
//        sum += n;
//        n -= 1;
//    }
//    return sum * sum;
//}
//
//pub fn sum_of_squares(mut n: i32) -> i32 {
//    let mut sum = 0;
//    while n > 0 {
//        sum += n * n;
//        n -= 1;
//    }
//    sum
//}
//
//pub fn difference(n: i32) -> i32 {
//    square_of_sum(n) - sum_of_squares(n)
//}

pub fn square_of_sum(n: usize) -> usize {
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn sum_of_squares(n: usize) -> usize {
    (0..n + 1).map(|x| x * x).fold(0, |accum, x| accum + x)
}

pub fn difference(n: usize) ->usize{
    square_of_sum(n) - sum_of_squares(n)
}