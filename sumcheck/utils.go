package sumcheck

import (
	"math"
	"reflect"
	"strconv"
)

// Arity returns the number of arguments taken by a function.
// In Go, we need to define a specific function type.
type FuncType func(...int) int

func Arity(f interface{}) int {
	// Get the type of f, which should be a function.
	fType := reflect.TypeOf(f)
	if fType.Kind() != reflect.Func {
		// Optionally, handle the case where f is not a function.
		return -1
	}
	// Return the number of input arguments.
	return fType.NumIn()
}

// returns n as a binary vector, front-padded to pad_to_len
// The use of ToBits is crucial for generating the necessary input patterns to
// systematically evaluate the function g over all possible binary input
// combinations, which is a common requirement in various computational and cryptographic algorithms.
func ToBits(n int, padToLen int) []int {
	binStr := strconv.FormatInt(int64(n), 2)

	if len(binStr) > padToLen {
		padToLen = len(binStr)
	}

	v := make([]int, len(binStr))
	for i, ch := range binStr {
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
func DegJ(g FuncType, j int) int {
	exp := 1
	for {
		args := make([]int, 1)
		for i := range args {
			if i == j {
				args[i] = 100
			} else {
				args[i] = 1
			}
		}

		out1 := g(args[0])

		args[0] = 1000

		out2 := g(args[0])

		// Consider a function f(x) = x²
		// To find the degree of x (assuming it's the second variable, so x = 1), the function would compare f(100) with f(1000).
		// If x is cubed (x³), the output should scale by 1000^ 3 / 100 ^ 3
		// when x changes from 100 to 1000. The function checks if this scaling holds to estimate the degree.
		if math.Abs(float64(out1)/math.Pow(100, float64(exp))-float64(out2)/math.Pow(1000, float64(exp))) < 1 {
			return exp
		} else if exp > 10 {
			panic("exp grew larger than 10")
		} else {
			exp++
		}

	}
}
