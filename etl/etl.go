package etl

import "strings"

func Transform(dict map[int][]string) map[string]int {
  newDict := make(map[string]int)

  for k, letters := range dict {
    for _, letter := range letters {
      newDict[strings.ToLower(letter)] = k
    }
  }

  return newDict
}
