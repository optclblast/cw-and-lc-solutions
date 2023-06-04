// https://www.codewars.com/kata/544675c6f971f7399a000e79
package kata

import (
	"fmt"
	"strconv"
)

func StringToNumber(str string) int {
	i, err := strconv.Atoi(str)
	if err != nil {
		fmt.Print(err)
	}
	return i
}
