// https://www.codewars.com/kata/515e271a311df0350d00000f
package kata

func SquareSum(numbers []int) int {
	res := 0
	for i, _ := range numbers {
		res += numbers[i] * numbers[i]
	}
	return res
}
