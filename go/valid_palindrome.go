//https://leetcode.com/problems/valid-palindrome/
import "strings"

func isPalindrome(s string) bool {
	var chars []rune
	for _, c := range []rune(s) {
		if unicode.IsLetter(c) {
			chars = append(chars, []rune(strings.ToLower(string(c)))[0])
		} else if _, err := strconv.Atoi(string(c)); err == nil {
			chars = append(chars, []rune(strings.ToLower(string(c)))[0])
		}
	}

	for i, j := 0, len(chars)-1; i < j; i, j = i+1, j-1 {
		if chars[i] != chars[j] {
			return false
		}
	}
	return true
}