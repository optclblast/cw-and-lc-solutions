//https://www.codewars.com/kata/526571aae218b8ee490006f4
fn count_bits(i: i64) -> u32 {
    let mut num:u32 = 0;
    for x in format!("{:b}", i).chars() {
        match x {
            '1' => {
                num += 1 as u32;
            },
            _ => {
                num += 0 as u32;
            }
        }
    }
    num as u32
}