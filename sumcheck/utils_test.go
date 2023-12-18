package sumcheck

import (
	"testing"

	"github.com/stretchr/testify/assert"
)


func Test_DegJ(t *testing.T) {
	f := func(a, b, c int) int { return a*b*b*c + b + c*c*c }

	assert.Equal(t,DegJ(f, 0),1)
	assert.Equal(t,DegJ(f, 1),2)
	assert.Equal(t,DegJ(f, 2),3)

}