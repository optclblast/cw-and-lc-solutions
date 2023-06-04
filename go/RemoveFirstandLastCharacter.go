// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0
package kata

import (
	"strings"
)

func RemoveChar(word string) string {
	runes := []rune(word)
	runes[0] = ' '
	runes[len(runes)-1] = ' '
	return strings.Trim(string(runes), " ")
}
