// https://www.codewars.com/kata/5963c18ecb97be020b0000a2
package kata

import (
	"fmt"
	"strconv"
)

func Derive(coefficient, exponent int) string {
	return fmt.Sprintf("%sx^%s", strconv.Itoa(coefficient*exponent), strconv.Itoa(exponent-1))
}
