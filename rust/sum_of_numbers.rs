//https://www.codewars.com/kata/55f2b110f61eb01779000053
fn get_sum(a: i64, b: i64) -> i64 {
    if a > b {
        b..=a
    }
    else {
        a..=b
    }.sum()
}