//https://www.codewars.com/kata/5266876b8f4bf2da9b000362
fn likes(names: &[&str]) -> String {
    match names {
        [] => {return format!("no one likes this")},
        [a] => {return format!("{0} likes this", names[0])},
        [a, b] => {return format!("{0} and {1} like this", names[0], names[1])},
        [a, b, c] => {return format!("{0}, {1} and {2} like this", names[0], names[1], names[2])},
        [a, b, rest @ ..] => {return format!("{0}, {1} and {2} others like this", names[0], names[1], rest.len())}, 
    }
}