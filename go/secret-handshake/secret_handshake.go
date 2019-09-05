package secret

func Reverse(slice []string) []string {
  newSlice := make([]string, 0)

  for i := len(slice) - 1; i >= 0; i-- {
    newSlice = append(newSlice, slice[i])
  }

  return newSlice
}

func Handshake(code uint) []string {
  shakes := make([]string, 0)

  if code <= 0 {
    return shakes
  }

  if code % 2 == 1 {
    shakes = append(shakes, "wink")
  }
  code = code >> 1
  if code == 0 {
    return shakes
  }

  if code % 2 == 1 {
    shakes = append(shakes, "double blink")
  }
  code = code >> 1
  if code == 0 {
    return shakes
  }

  if code % 2 == 1 {
    shakes = append(shakes, "close your eyes")
  }
  code = code >> 1
  if code == 0 {
    return shakes
  }

  if code % 2 == 1 {
    shakes = append(shakes, "jump")
  }
  code = code >> 1

  if code % 2 == 0 {
    return shakes
  }

  return Reverse(shakes)
}
