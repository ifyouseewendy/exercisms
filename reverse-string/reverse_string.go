package reverse

func String(str string) string {
  if (len(str) <= 1) {
    return str
  }

  reversedStr := make([]rune, 0)
  runes := []rune(str)
  for i := (len(runes) - 1); i >= 0; i-- {
    reversedStr = append(reversedStr, runes[i])
  }

  return string(reversedStr)
}
