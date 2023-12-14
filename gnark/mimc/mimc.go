package mimc

import (
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/hash/mimc"
)

// mimc(secret preImage) = public hash
type Circuit struct {
	PreImage frontend.Variable
	Hash frontend.Variable `gnark:",public"`
}


// Hash = mimc(PreImage)
func (circuit *Circuit) Define(api frontend.API) error {
	mimc, _ := mimc.NewMiMC(api)
	mimc.Write(circuit.PreImage)

	api.AssertIsEqual(circuit.Hash, mimc.Sum())
	return nil
}

