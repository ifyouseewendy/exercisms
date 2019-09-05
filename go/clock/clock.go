package clock

import "fmt"

type Clock struct {
  hour int
  minute int
}

func New(h, m int) Clock {
  hour, minute := 0, 0
  offsetHour := 0

  if m > 0 {
    offsetHour += (m / 60)
    minute = m % 60
  } else if m == 0 {
    minute = 0
  } else {
    offsetHour += (m / 60) - 1
    minute = m % 60 + 60

    if minute == 60 {
      offsetHour += 1
      minute = 0
    }
  }

  ho := h + offsetHour
  if ho > 0 {
    hour = ho % 24
  } else if ho == 0 {
    hour = 0
  } else {
    hour = (ho % 24 + 24) % 24
  }

  return Clock{hour, minute}
}

func (clock Clock) String() string {
  return fmt.Sprintf("%02d:%02d", clock.hour, clock.minute)
}

func (clock Clock) Add(minutes int) Clock {
  return New(clock.hour, clock.minute + minutes)
}

func (clock Clock) Subtract(minutes int) Clock {
  return New(clock.hour, clock.minute - minutes)
}

// I don't really get what's the diff between Mod and Remainder
// https://play.golang.org/p/uHPeydec5q-
// fmt.Printf("%f", math.Mod(-24, 60))
// fmt.Println()
// fmt.Printf("%f", math.Mod(24, -60))
// fmt.Println()
// fmt.Printf("%f", math.Mod(24, 60))
// fmt.Println()
// fmt.Printf("%f", math.Remainder(-24, 60))
// fmt.Println()
// fmt.Printf("%f", math.Remainder(24, -60))
// fmt.Println()
// fmt.Printf("%f", math.Remainder(24, 60))
// fmt.Println()
// -24.000000
// 24.000000
// 24.000000
// -24.000000
// 24.000000
// 24.000000
