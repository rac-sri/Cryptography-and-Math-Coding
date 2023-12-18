package sumcheck

import (
	"math"
	"strconv"
)

// Arity returns the number of arguments taken by a function.
// In Go, we need to define a specific function type.
type FuncType func(int, int, int) int


// returns n as a binary vector, front-padded to pad_to_len
func ToBits(n int, padToLen int) []int {
	binStr := strconv.FormatInt(int64(n),2)
	v := make([]int, len(binStr))
	for i,ch := range binStr {
		if ch == '1' {
			v[i] = 1
		} else {
			v[i] = 0
		}
	}

	diff := padToLen - len(v)
	paddedV := make([]int, diff)
	return append(paddedV, v...)
}

// DegJ returns the degree of the j'th variable in g
// Assumes a non-negative integer power less than 10
// Function is highly specific to the function signature and use case
func DegJ(g FuncType, j int ) int {
	exp := 1
	for {
		args := make([]int, 3)
		for i:= range args {
			if i==j {
				args[i] = 100
			}  else {
				args[i] = 1
			}
		}

		out1 := g(args[0], args[1], args[2])

		args[j] = 1000

		out2 := g(args[0], args[1], args[2])

		if math.Abs(float64(out1)/math.Pow(100, float64(exp))-float64(out2)/math.Pow(1000, float64(exp))) < 1 {
			return exp
		} else if exp > 10 {
			panic("exp grew larger than 10")
		} else {
			exp++
		}

	}
}