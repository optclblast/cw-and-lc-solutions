//https://www.codewars.com/kata/576bb71bbbcf0951d5000044
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut res:Vec<i32> = Vec::new();
    let mut pos:i32 = 0;
    let mut neg:i32 = 0;
    for &num in &input {
        if num > 0 {
            pos += 1;
        }
        else if num < 0 {
            neg += num;
        }
    }
    if input.len() > 0 {
        res.push(pos);
        res.push(neg);
    }
    res
}