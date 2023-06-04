// https://www.codewars.com/kata/5a6663e9fd56cb5ab800008b
package kata

func CalculateYears(years int) (result [3]int) {
	CatYears := 0
	DogYears := 0
	if years >= 2 {
		CatYears = 24
		DogYears = 24
		for i := 0; i < years-2; i++ {
			CatYears += 4
			DogYears += 5
		}
	} else {
		CatYears = 15
		DogYears = 15
	}
	res := [3]int{years, CatYears, DogYears}
	return res
}
