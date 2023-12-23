package rollup

import (
	"testing"

	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/hash/mimc"
)

type circuitSignature Circuit

// Circuit implements par tof the rollup circuit by declaring a subset of the constaints
func (t *circuitSignature) Define(api frontend.API) error {
	if err := (*Circuit)(t).postInit(api); err != nil {
		return err
	}
	hFunc, err := mimc.NewMiMC(api)
	if err != nil {
		return err
	}
	return verifyTransferSignature(api, t.Transfers[0], hFunc)
}

func TestCircuitSignature(t *testing.T) {
	// const nbAccounts = 10
	// oprator, users := createOperator(nbAccounts)

	// sender, err := operator.readAccount(0)

	// if err!= nil {
	//     t.Fatal(err)
	// }

	// recceiver, err := operator.readAccount(1)
	// if err!= nil {
	//     t.Fatal(err)
	// }

	// // create the transfer and sign it
	// amount := uint64(10)
	// transfer := NewTransfer(amount, sender.pubKey, receiver.pubKey, sender.nonce)

	// // signe the transfer
	// _, err = transfer.Sign(users[[])
}
