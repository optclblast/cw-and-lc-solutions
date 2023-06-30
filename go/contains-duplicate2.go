//https://leetcode.com/problems/contains-duplicate-ii/
func containsNearbyDuplicate(nums []int, k int) bool {
	m := make(map[int]int)
	for i, v := range nums {
		if v, ok := m[v]; ok {
			if abs(i-v) <= k {
				return true
			}
		}
		m[v] = i
	}
	return false
}

func abs(i int) int {
	if i < 0 {
		return i * (-1)
	}
	return i
}