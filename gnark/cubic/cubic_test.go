package cubic

import (
	"testing"

	"github.com/consensys/gnark/test"
)

func TestCubicEquation(t *testing.T) {
	assert := test.NewAssert(t)

	var cubicCircuit Circuit

	assert.ProverFailed(&cubicCircuit, &Circuit{
		X: 42,
		Y: 42,
	})

	assert.ProverSucceeded(&cubicCircuit, &Circuit {
		X :3,
		Y: 35,
	})
}