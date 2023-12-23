package fptower

func (z *E24) nSquare(n int) {
	for i := 0; i < n; i++ {
		z.CyclotomicSquare(z)
	}
}

func (z *E24) nSquareCompressed(n int) {
	for i := 0; i < n; i++ {
		z.CyclotomicSquareCompressed(z)
	}
}

// ExptHalf set z to x^(t/2) in E24 and return z (t is the seed of the curve)
// t/2 = 1820377088
func (z *E24) ExptHalf(x *E24) *E24 {
	// Expt computation is derived from the addition chain:
	//
	//	_10       = 2*1
	//	_11       = 1 + _10
	//	_11000    = _11 << 3
	//	_11000000 = _11000 << 3
	//	_11011000 = _11000 + _11000000
	//	_11011001 = 1 + _11011000
	//	return      (_11011001 << 9 + _11) << 14
	//
	// Operations: 30 squares 4 multiplies
	//
	// Generated by github.com/mmcloughlin/addchain v0.4.0.

	// Allocate Temporaries.
	var t0, t1, result E24

	// Step 1: result = x^0x2
	result.CyclotomicSquare(x)

	// Step 2: result = x^0x3
	result.Mul(x, &result)

	// Step 5: t0 = x^0x18
	t0.CyclotomicSquare(&result)
	t0.nSquare(2)

	// Step 8: t1 = x^0xc0
	t1.CyclotomicSquare(&t0)
	t1.nSquare(2)

	// Step 9: t0 = x^0xd8
	t0.Mul(&t0, &t1)

	// Step 10: t0 = x^0xd9
	t0.Mul(x, &t0)

	// Step 19: t0 = x^0x1b200
	t0.nSquareCompressed(9)
	t0.DecompressKarabina(&t0)

	// Step 20: result = x^0x1b203
	result.Mul(&result, &t0)

	// Step 35: result = x^0xd9018000
	result.nSquareCompressed(14)
	result.DecompressKarabina(&result)

	z.Set(&result)

	return z
}

// Expt set z to x^t in E24 and return z (t is the seed of the curve)
// t = 3640754176
func (z *E24) Expt(x *E24) *E24 {
	var result E24
	result.ExptHalf(x)
	return z.CyclotomicSquare(&result)
}

// MulBy014 multiplication by sparse element (c0, c1, 0, 0, c4, 0)
func (z *E24) MulBy014(c0, c1, c4 *E4) *E24 {

	var a, b E12
	var d E4

	a.Set(&z.D0)
	a.MulBy01(c0, c1)

	b.Set(&z.D1)
	b.MulBy1(c4)
	d.Add(c1, c4)

	z.D1.Add(&z.D1, &z.D0)
	z.D1.MulBy01(c0, &d)
	z.D1.Sub(&z.D1, &a)
	z.D1.Sub(&z.D1, &b)
	z.D0.MulByNonResidue(&b)
	z.D0.Add(&z.D0, &a)

	return z
}

// Mul014By014 multiplication of sparse element (c0,c1,0,0,c4,0) by sparse element (d0,d1,0,0,d4,0)
func Mul014By014(d0, d1, d4, c0, c1, c4 *E4) [5]E4 {
	var z00, tmp, x0, x1, x4, x04, x01, x14 E4
	x0.Mul(c0, d0)
	x1.Mul(c1, d1)
	x4.Mul(c4, d4)
	tmp.Add(c0, c4)
	x04.Add(d0, d4).
		Mul(&x04, &tmp).
		Sub(&x04, &x0).
		Sub(&x04, &x4)
	tmp.Add(c0, c1)
	x01.Add(d0, d1).
		Mul(&x01, &tmp).
		Sub(&x01, &x0).
		Sub(&x01, &x1)
	tmp.Add(c1, c4)
	x14.Add(d1, d4).
		Mul(&x14, &tmp).
		Sub(&x14, &x1).
		Sub(&x14, &x4)

	z00.MulByNonResidue(&x4).
		Add(&z00, &x0)

	return [5]E4{z00, x01, x1, x04, x14}
}

// MulBy01245 multiplies z by an E24 sparse element of the form (x0, x1, x2, 0, x4, x5)
func (z *E24) MulBy01245(x *[5]E4) *E24 {
	var c1, a, b, c, z0, z1 E12
	c0 := &E12{C0: x[0], C1: x[1], C2: x[2]}
	c1.C1 = x[3]
	c1.C2 = x[4]
	a.Add(&z.D0, &z.D1)
	b.Add(c0, &c1)
	a.Mul(&a, &b)
	b.Mul(&z.D0, c0)
	c.Set(&z.D1).MulBy12(&x[3], &x[4])
	z1.Sub(&a, &b)
	z1.Sub(&z1, &c)
	z0.MulByNonResidue(&c)
	z0.Add(&z0, &b)

	z.D0 = z0
	z.D1 = z1

	return z
}
