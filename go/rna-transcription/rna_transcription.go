package strand

import "strings"

func replace(r rune) rune {
  switch r {
    case 'C':
      return 'G'
    case 'G':
      return 'C'
    case 'T':
      return 'A'
    case 'A':
      return 'U'
    default:
      return -1
  }
}

func ToRNA(dna string) string {
  return strings.Map(replace, dna)
}
