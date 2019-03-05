package accumulate

func Accumulate(collection []string, converter func(string) string) []string {
  newCollection := make([]string, len(collection))
  for i, ele := range collection {
    newCollection[i] = converter(ele)
  }
  return newCollection
}
