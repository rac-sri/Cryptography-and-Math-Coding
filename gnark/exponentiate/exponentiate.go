package exponentiate

import (
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/math/bits"
)

// Circuit y = x ** e
type Circuit struct {
	X frontend.Variable `gnark:",public"`
	Y frontend.Variable `gnark:",public"`

	E frontend.Variable
}

func (circuit *Circuit) Define(api frontend.API) error {
	const bitSize = 8;

	output := frontend.Variable(1)
	bits := bits.ToBinary(api, circuit.E, bits.WithNbDigits(bitSize))

	for i:=0; i< len(bits); i++ {
		if i!=0 {
			output = api.Mul(output, output)
		}

		multiply := api.Mul(output, circuit.X)
		output = api.Select(bits[len(bits) - 1 -i], multiply, output)
	}

	api.AssertIsEqual(circuit.Y, output)

	return nil
}