// https://www.codewars.com/kata/515decfd9dcfc23bb6000006
package kata

import (
	"strconv"
	"strings"
)

func Is_valid_ip(ip string) bool {
	nums := strings.Split(ip, ".")
	if len(nums) != 4 {
		return false
	}
	for _, v := range nums {
		val, e := strconv.Atoi(v)
		if ((val < 100 && len(v) >= 3) || e != nil) || val > 255 || val < 0 {
			return false
		}
	}
	return true
}
