//https://www.codewars.com/kata/554e4a2f232cdd87d9000038
fn dna_strand(dna: &str) -> String {
    let mut dnamirror = String::new();
    for c in dna.chars() {
        match c {
            'A' => {
               dnamirror.push('T');
            },
            'T' => {
                dnamirror.push('A');
            },
            'C' => {
                dnamirror.push('G');
            },
            'G' => {
                dnamirror.push('C');
            }
            _ => {}
        }
    }
    dnamirror
}