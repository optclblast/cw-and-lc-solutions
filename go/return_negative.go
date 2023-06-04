// https://www.codewars.com/kata/55685cd7ad70877c23000102
package kata

func MakeNegative(x int) int {
	if x < 0 {
		return x
	}
	return x - x*2
}
