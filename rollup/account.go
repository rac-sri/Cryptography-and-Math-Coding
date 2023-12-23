package rollup

import (
	"encoding/binary"

	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark-crypto/ecc/bn254/twistededwards/eddsa"
)

var (
	// SizeAccount byte size of a serialized account (5*32 bytes)
	// index | nonce | balance | pubkeyX | pubkeyY , each chink is 32 bytes
	SizeAccount = 160
)

type Account struct {
	index   uint64
	nonce   uint64
	balance fr.Element
	pubKey  eddsa.PublicKey
}

func (ac *Account) Reset() {
	ac.index = 0
	ac.nonce = 0
	ac.balance.SetZero()
	ac.pubKey.A.X.SetZero()
	ac.pubKey.A.Y.SetOne()
}

// Serialize serializes the account as a concatenation of 5 chunks of 256 bits
// one chunk per field (pubKey has 2 chunks), except index and nonce that are concatenated in a single 256 bits chunk
// index ∥ nonce ∥ balance ∥ pubkeyX ∥ pubkeyY, each chunk is 256 bits
func (ac *Account) Serialize() []byte {
	var res [160]byte

	// firrst chunk of 256 bits
	binary.BigEndian.PutUint64(res[24:], ac.index) // index is on 64 bits, so fill the last chunk of 64bits in the first 256 bits slot
	binary.BigEndian.PutUint64(res[56:], ac.nonce)

	// balance
	buf := ac.balance.Bytes()
	copy(res[64:], buf[:])

	// public key
	buf = ac.pubKey.A.X.Bytes()
	copy(res[96:], buf[:])

	buf = ac.pubKey.A.Y.Bytes()
	copy(res[128:], buf[:])

	return res[:]
}

func Deserialize(res *Account, data []byte) error {
	res.Reset()

	// memory bound check
	if len(data) != SizeAccount {
		return ErrSizeByteSlice
	}

	res.index = binary.BigEndian.Uint64(data[24:32])
	res.nonce = binary.BigEndian.Uint64(data[56:64])
	res.balance.SetBytes(data[64:96])
	res.pubKey.A.X.SetBytes(data[96:128])
	res.pubKey.A.Y.SetBytes(data[128:])

	return nil
}
