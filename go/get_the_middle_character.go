// https://www.codewars.com/kata/56747fd5cb988479af000028
package kata

func GetMiddle(s string) string {
	runes := []rune(s)
	var str []rune
	if len(runes)%2 == 0 {
		str = append(str, runes[len(runes)/2-1], runes[len(runes)/2])
		return string(str)
	}
	str = append(str, runes[len(runes)/2])
	return string(str)
}
