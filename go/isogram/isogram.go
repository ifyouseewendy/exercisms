package isogram

import "strings"

const Alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

func isAlphabet(s string) bool {
  return strings.ContainsAny(s, Alphabet)
}

func IsIsogram(sentence string) bool {
  dict := make(map[string]int)

  for _, letter := range sentence {
    if !isAlphabet(string(letter)) {
      continue
    }

    if dict[strings.ToLower(string(letter))] == 1 {
      return false
    }
    dict[strings.ToLower(string(letter))]++
  }

  return true
}
