//https://www.codewars.com/kata/56269eb78ad2e4ced1000013
fn find_next_square(sq: u64) -> Option<u64> {
    if (sq as f64).sqrt() - (sq as f64).sqrt().floor() != 0.0 {
        return None
    }
    let r = ((sq as f64).sqrt()+1.0) as u64 *((sq as f64).sqrt()+1.0) as u64;
    Some(r)
}