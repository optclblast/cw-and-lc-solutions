//https://www.codewars.com/kata/53af2b8861023f1d88000832
fn are_you_playing_banjo(name: &str) -> String {
    if name.chars().nth(0).unwrap() == 'R' || name.chars().nth(0).unwrap() == 'r'{
        return format!("{} plays banjo", name);
    }
    else {
        return format!("{} does not play banjo", name);
    }
}