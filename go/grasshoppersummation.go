// https://www.codewars.com/kata/55d24f55d7dd296eb9000030
package kata

func Summation(n int) int {
	res := 0
	for i := 1; i <= n; i++ {
		res += i
	}
	return res
}
