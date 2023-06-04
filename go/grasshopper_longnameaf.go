// https://www.codewars.com/kata/586c1cf4b98de0399300001d
package kata

func combat(health, damage float64) float64 {
	if float64(health-damage) > 0 {
		return float64(health - damage)
	}
	return 0.0
}
