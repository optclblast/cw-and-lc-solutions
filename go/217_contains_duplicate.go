//https://leetcode.com/problems/contains-duplicate/
func containsDuplicate(nums []int) bool {
	container := make(map[int]int)
	for _, v := range nums {
		if _, ok := container[v]; ok {
			return true
		}
		container[v] = v
	}
	return false
}