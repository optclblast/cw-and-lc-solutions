//https://www.codewars.com/kata/5715eaedb436cf5606000381
fn positive_sum(slice: &[i32]) -> i32 {
    let mut sum:i32 = 0;
    for x in slice.iter() {
        if *x > 0 {
            sum += *x;
        }
    }
    sum
}