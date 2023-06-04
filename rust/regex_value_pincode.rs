//https://www.codewars.com/kata/55f8a9c06c018a0d6e000132
fn validate_pin(pin: &str) -> bool {
    if pin.len() == 4 as usize || pin.len() == 6 as usize {
        for c in pin.chars() {
            match c {
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {},
                _ => {
                    return false
                }
            }
        }
        return true
    }
    else {
        return false
    }
}