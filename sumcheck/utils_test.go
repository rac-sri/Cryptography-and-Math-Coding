package sumcheck

import (
	"testing"

	"github.com/stretchr/testify/assert"
)


func TestDegJ(t *testing.T) {
	f := func(args ...int) int {
        if len(args) == 0 {
            return 0 // or some default value
        }
        a := args[0] 
        return a*a*a + a + a*a*19
    }

	assert.Equal(t, 3, DegJ(FuncType(f), 0))
}