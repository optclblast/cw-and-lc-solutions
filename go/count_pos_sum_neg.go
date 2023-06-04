// https://www.codewars.com/kata/576bb71bbbcf0951d5000044
package kata

func CountPositivesSumNegatives(numbers []int) []int {
	res := make([]int, 0, 2)
	negs := 0
	pos := 0
	for _, x := range numbers {
		if x > 0 {
			pos += 1
		} else if x < 0 {
			negs += x
		}
	}
	if negs > 0 || pos > 0 {
		res = append(res, pos)
		res = append(res, negs)
	}
	return res // your code here
}
