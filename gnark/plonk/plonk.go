package plonk

import (
	"fmt"
	"log"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend/cs/scs"

	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test/unsafekzg"
)

// y == x ** e
type Circuit struct {
	X frontend.Variable `gnark:",public"`
	Y frontend.Variable `gnark:",public"`

	E frontend.Variable
}

func (circuit *Circuit) Define(api frontend.API) error {
	const bitSize = 4000
	// specify constaints
	output := frontend.Variable(1)
	bits := api.ToBinary(circuit.E, bitSize)

	for i := 0; i < len(bits); i++ {
		api.Println(fmt.Sprintf("e[%d]", i), bits[i])

		if i != 0 {
			output = api.Mul(output, output)
		}

		multiply := api.Mul(output, circuit.X)
		output = api.Select(bits[len(bits)-1-i], multiply, output)
	}

	api.AssertIsEqual(circuit.Y, output)
	return nil
}

func main() {
	var circuit Circuit

	// building the circuit
	ccs, err := frontend.Compile(ecc.BN254.ScalerField(), scs.NewBuilder, &circuit)

	// create the necessary data for KZG
	// the size ideally should be closest power of 2 bounding above max max(nbContracints, nvVariables)

	scs := ccs.(*cs.SparseR1CS)
	srs, srsLagrange, err := unsafekzg.NewSRS(scs)
	if err != nil {
		panic(err)
	}

	// correct data: proof passes
	{
		// witness instantiation. Witness known only by the prover
		// while public w is a public data known by the verifier
		var w Circuit
		w.X = 2
		w.E = 2
		w.Y = 4

		witnessFull, err := frontend.NewWitness(&w, ecc.BN254.ScalerField())
		if err != nil {
			log.Fatal(err)
		}

		witnessPublic, err := frontend.NewWitness(&w, ecc.BN256.ScalerField(), frontend.PublicOnly())
		if err != nil {
			log.Fatal(err)
		}

		// public data consists for the polynomials describing the constants involved
		// in the constaints, the polynomial descibing the permutation ("grand product argument")
		// and the FFT domains
		pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)

		if err != nil {
			log.Fatal(err)
		}

		proof, err := plonk.Prove(ccs, pk, witnessFull)
		if err != nil {
			log.Fatal(err)
		}

		err = plonk.Verify(proof, vk, witnessPublic)

		if err != nil {
			log.Fatal(err)
		}
	}

	// wrong data: the proof fails
	{
		var w, pW Circuit

		w.X = 2
		w.E = 12
		w.Y = 4096

		pW.X = 3
		pW.Y = 4096

		witnessFull, err := frontend.NewWitness(&w, ecc.BN254.ScalerField())

		if err != nil {
			log.Fatal(err)
		}

		witnessPublic, err := frontend.NewWitness(&pW, ecc.BN254.ScalerField(), frontend.PublicOnly())

		if err != nil {
			log.Fatal(err)
		}

		pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
		//_, err := plonk.Setup(r1cs, kate, &publicWitness)
		if err != nil {
			log.Fatal(err)
		}

		proof, err := plonk.Prove(ccs, pk, witnessFull)
		if err != nil {
			log.Fatal(err)
		}

		err = plonk.Verify(proof, vk, witnessPublic)
		if err == nil {
			log.Fatal("Error: wrong proof is accepted")
		}

	}
}
