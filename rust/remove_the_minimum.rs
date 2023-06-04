//https://www.codewars.com/kata/563cf89eb4747c5fb100001b
fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut v:Vec<u32> = Vec::new();//vec![0 as u32; numbers.len()-1];
    if numbers.len() == 0 {
        return v
    }
    for x in numbers.clone().iter() {
        v.push(*x);
    }
    let mut ind:usize = 0;
    let mut val:u32 = v[0];
    for (i, value) in v.iter().enumerate() {
        
        if *value < val {
            ind = i;
            val = *value;
        } 
    }
    v.remove(ind);
    v
}