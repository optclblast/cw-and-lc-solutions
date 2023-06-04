//https://www.codewars.com/kata/559590633066759614000063
fn min_max(lst: &[i32]) -> (i32, i32) {
    (min(lst), max(lst))
}

fn min(arr: &[i32]) -> i32 {
    let mut min = arr[0];
    for i in arr {
        if *i < min {
            min = *i;
        }
    }
    min
}
fn max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for i in arr {
        if *i > max {
            max = *i;
        }
    }
    max
}