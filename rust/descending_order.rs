//https://www.codewars.com/kata/5467e4d82edf8bbf40000155
//solution 1
use std::iter::FromIterator;
fn descending_order(x: u64) -> u64 {
    let mut res = x.to_string()
                    .chars()
                    .collect::<Vec<char>>();
    res.sort_by(|a, b| b.cmp(a));
    String::from_iter(res).parse::<u64>().unwrap()      
}

//solution 2
fn descending_order(x: u64) -> u64 {
    let mut corevec:Vec<_> = x.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut counter:u32 = corevec.len() as u32;
    for _i in 0..corevec.len() {
        for (i, e) in corevec.clone().iter().enumerate() {
            if (i as u32) == counter {
                break;
            }
            if i == 0 {
                continue;
            }
            if e > &corevec[i-1] {
                let amogus:u32 = corevec[i-1];
                corevec[i-1] = *e;
                corevec[i] = amogus;
            }
        }
        counter -= 1;
    }
    corevec.iter().map(|x| x.to_string()).collect::<String>().parse::<u64>().unwrap()
}