//https://www.codewars.com/kata/53da3dbb4a5168369a0000fe
fn even_or_odd(i: i32) -> &'static str {
    if i%2 == 0 {
        return "Even";
    }
    else {
        return "Odd"
    }
}