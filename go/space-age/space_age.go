package space

type Planet string

// The visibility of a name outside a package is determined
// by whether its first character is upper case.
const oneEarthYearSeconds = 31557600.0

var orbitalRate = map[Planet]float64{
  "Earth": 1,
  "Mercury": 0.2408467,
  "Venus": 0.61519726,
  "Mars": 1.8808158,
  "Jupiter": 11.862615,
  "Saturn": 29.447498,
  "Uranus": 84.016846,
  "Neptune": 164.79132,
}

func Age(seconds float64, planet Planet) float64 {
  rate := orbitalRate[planet]
  return seconds / oneEarthYearSeconds / rate
}
