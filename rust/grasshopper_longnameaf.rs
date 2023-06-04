// https://www.codewars.com/kata/586c1cf4b98de0399300001d
fn combat(health: f32, damage: f32) -> f32 {
    if health - damage >= 0.0 {
        return health - damage;
    }
    return 0.0
}
