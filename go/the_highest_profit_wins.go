// https://www.codewars.com/kata/559590633066759614000063

package kata

func MinMax(arr []int) [2]int {
	return [2]int{min(arr), max(arr)}
}

func min(arr []int) int {
	min := arr[0]
	for _, i := range arr {
		if i < min {
			min = i
		}
	}
	return min
}

func max(arr []int) int {
	max := arr[0]
	for _, i := range arr {
		if i > max {
			max = i
		}
	}
	return max
}
