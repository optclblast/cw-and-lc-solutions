//https://www.codewars.com/kata/5963c18ecb97be020b0000a2
fn derive(coefficient: u32, exponent: u32) -> String {
    format!("{0}x^{1}", coefficient*exponent, exponent-1)
}