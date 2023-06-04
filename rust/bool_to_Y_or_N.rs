//https://www.codewars.com/kata/53369039d7ab3ac506000467
fn bool_to_word(value: bool) -> &'static str {
    let mut yesno:&str = "";  
    match value {
        true => {
            yesno = "Yes";
        }
        false => {
            yesno = "No"
        }
    }
    yesno
}