////https://www.codewars.com/kata/5266876b8f4bf2da9b000362
fn likes(names: &[&str]) -> String {
    if names.len() > 3 {
        return format!("{0}, {1} and {2} others like this", names[0], names[1], names.len()-2)
    }
    else if names.len() == 3 {
        return format!("{0}, {1} and {2} like this", names[0], names[1], names[2])
    }
    else if names.len() == 2 {
        return format!("{0} and {1} like this", names[0], names[1])
    }
    else if names.len() == 1 {
        return format!("{0} likes this", names[0])
    }
    format!("no one likes this")
}