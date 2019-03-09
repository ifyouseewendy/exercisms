package luhn

import "strconv"

func Valid(str string) bool {
  if len(str) < 2 {
    return false
  }

  numbers := make([]int, 0)
  for i := 0; i < len(str); i++ {
    if str[i] == ' ' {
      continue
    } else if str[i] < '0' || str[i] > '9' {
      return false
    }

    num, _ := strconv.Atoi(string(str[i]))
    numbers = append(numbers, num)
  }

  if len(numbers) < 2 {
    return false
  }

  sum := 0
  second := false
  for i := len(numbers) - 1; i >= 0; i-- {
    if second == false {
      sum += numbers[i]
      second = true
    } else {
      num := 2 * numbers[i]
      if num > 9 { num -= 9 }
      sum += num
      second = false
    }
  }

  return sum % 10 == 0
}
