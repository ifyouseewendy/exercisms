package raindrops

import "strconv"

func Convert(num int) string {
  if num % 3 != 0 && num % 5 != 0 && num % 7 != 0 {
    return strconv.Itoa(num)
  }

  raindrop := []byte("")
  if num % 3 == 0 {
    raindrop = append(raindrop, "Pling"...)
  }
  if num % 5 == 0 {
    raindrop = append(raindrop, "Plang"...)
  }
  if num % 7 == 0 {
    raindrop = append(raindrop, "Plong"...)
  }

  return string(raindrop)
}
