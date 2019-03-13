package grains

import "errors"

func Square(num int) (uint64, error) {
  if num < 1 || num > 64 {
    return 0, errors.New("There is no such square")
  }

  return uint64(1) << uint64(num -1), nil
}

func Total() uint64 {
  return 1 << 64 - 1
}
