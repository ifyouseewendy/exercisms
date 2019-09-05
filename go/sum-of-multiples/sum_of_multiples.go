package summultiples

func SumMultiples(limit int, divisors... int) int {
  sum := 0
  dict := make(map[int]bool)

  for i := 0; i < len(divisors); i++ {
    num := 0

    if divisors[i] <= 0 {
      continue
    }

    for num < limit {
      if dict[num] != true {
        dict[num] = true
        sum += num
      }

      num += divisors[i]
    }
  }

  return sum
}
