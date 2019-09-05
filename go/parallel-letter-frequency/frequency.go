package letter

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

func ConcurrentFrequency(words []string) FreqMap {
	m := FreqMap{}
	c := make(chan rune)
	stops := len(words)

	for i := 0; i < len(words); i++ {
		go func(word string, runeChannel chan<- rune) {
			for _, r := range word {
				runeChannel <- r
			}
			runeChannel <- -1
		}(words[i], c)
	}

	for stops != 0 {
		r := <-c
		if r == -1 {
			stops--
		} else {
			m[r]++
		}
	}

	return m
}
