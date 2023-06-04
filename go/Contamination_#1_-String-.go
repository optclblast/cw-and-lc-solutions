// https://www.codewars.com/kata/596fba44963025c878000039
package kata

import "fmt"

func Contamination(text, char string) string {
	res := ""
	for x := 0; x < len(text); x++ {
		res += fmt.Sprintf("%s", char)
	}
	return res
}
