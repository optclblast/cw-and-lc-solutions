// https://www.codewars.com/kata/59ca8246d751df55cc00014c
package kata

func Hero(bullets, dragons int) bool {
	if bullets-dragons*2 >= 0 {
		return true
	}
	return false
}
