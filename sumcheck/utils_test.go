package sumcheck

import (
	"fmt"
	"testing"
)


func Test_DegJ(t *testing.T) {
	f := func(a, b, c int) int { return a*b*b*c + b + c*c*c }
	
	fmt.Println(DegJ(f, 0))
	fmt.Println(DegJ(f, 1))
	fmt.Println(DegJ(f, 2))
}