package rollup

import "errors"

var (
	ErrSizeByteSlice = errors.New("byte slice size is inconsistent with Account size")

	ErrNonExistingAccount = errors.New("the account is not in the rollup database")

	ErrWrongSignature = errors.New("invalid signature")

	ErrAmountTooHigh = errors.New("amount is bigger than balance")

	ErrNonce = errors.New("incorrect nonce")

	ErrIndexConsistency = errors.New("account's position should match account's index")
)
