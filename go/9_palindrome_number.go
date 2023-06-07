//https://leetcode.com/problems/palindrome-number/
import "strconv"

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}

	slOG := []rune(strconv.Itoa(x))
	sl := []rune(strconv.Itoa(x))

	for i, j := 0, len(sl)-1; i < j; i, j = i+1, j-1 {
		sl[i], sl[j] = sl[j], sl[i]
	}
	for k, v := range slOG {
		if v != sl[k] {
			return false
		}
	}
	return true
}