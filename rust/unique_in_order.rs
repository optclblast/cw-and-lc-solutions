//https://www.codewars.com/kata/54e6533c92449cc251001667
fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut v = Vec::new();
    for i in sequence {
        if v.len() != 0 {
            if v[v.len()-1] != i {
                v.push(i);
            }
        }
        else {
            v.push(i);
        }
    }
    v
}