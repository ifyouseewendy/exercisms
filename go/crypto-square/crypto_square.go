package cryptosquare

import "math"

func Normalize(input string) string {
  ret := make([]byte, 0)

  for i := 0; i < len(input); i++ {
    s := input[i]

    if s >= 'A' && s <= 'Z' {
      s = s + 32
    }

    if (s >= 'a' && s <= 'z') || (s >= '0' && s <= '9') {
      ret = append(ret, s)
    }
  }

  return string(ret)
}

func CalculateRectangle(length int) (int, int) {
  if length <= 0 {
    return 0, 0
  } else if length == 1 {
    return 1, 1
  }

  x := int(math.Sqrt(float64(length)))

  if x * x >= length {
    return x, x
  } else if x * (x + 1) >= length {
    return x, x + 1
  } else {
    return x+1, x+1
  }
}

func Encode(input string) string {
  str := Normalize(input)

  r, c := CalculateRectangle(len(str))

  ret := make([]byte, 0)
  for i := 0; i < c; i++ {
    for j := 0; j < r; j++ {
      index := (j * c) + i
      var s byte
      if index >= len(str) {
        s = ' '
      } else {
        s = str[index]
      }

      ret = append(ret, s)
    }

    if i < c -1 {
      ret = append(ret, ' ')
    }
  }

  return string(ret)
}
