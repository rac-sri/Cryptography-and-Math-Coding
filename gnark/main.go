package main

import (
	"math/big"

	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/algebra/twistededwards"
	"github.com/consensys/gnark/std/hash"
)

func Verify(curve twistededwards.Curve, sig Signature, msg frontend.Variable, pubKey PublicKey, hash hash.Hash) error {

    // compute H(R, A, M)
    data := []frontend.Variable{
        sig.R.A.X,
        sig.R.A.Y,
        pubKey.A.X,
        pubKey.A.Y,
        msg,
    }
    hramConstant := hash.Hash(cs, data...)

    return nil
}
type CurveParams  struct {
	A, D, Cofactor, Order *big.Int
	Base [2]*big.Int
}

type PublicKey struct {
	A twistededwards.Point
}

type Signature struct {
	R twistededwards.Point
	S frontend.Variable
}

// eddsa verification : [2 c∗S]G=[2c]R+[2^c.H(R,A,M)]A


func Verify(curve twistededwards.Curve, sig Signature, msg frontend.Variable, pubKey PublicKey, hash hash.Hash) error {

    // compute H(R, A, M)
    data := []frontend.Variable{
        sig.R.A.X,
        sig.R.A.Y,
        pubKey.A.X,
        pubKey.A.Y,
        msg,
    }
    hramConstant := hash.Hash(cs, data...)

    return nil
}