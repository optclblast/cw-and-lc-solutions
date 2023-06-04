// https://www.codewars.com/kata/554e4a2f232cdd87d9000038
package kata

func DNAStrand(dna string) string {
	nucls := []rune(dna)
	for index, char := range nucls {
		switch char {
		case 'A':
			nucls[index] = 'T'
		case 'T':
			nucls[index] = 'A'
		case 'G':
			nucls[index] = 'C'
		case 'C':
			nucls[index] = 'G'
		}
	}
	return string(nucls)
}
