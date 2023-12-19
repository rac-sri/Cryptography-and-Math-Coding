package sumcheck

import "fmt"

type InefficientProver struct {
	g  FuncType
	gArity int
	randomChallenges []int
	polynomials 	[]FuncType
	round 			int
	H				int
}

func NewInefficientProver(g FuncType, gArity int) *InefficientProver {
	p := &InefficientProver{
		g:	g,
		gArity: gArity,
		round:	1,
	}

	var sum int
	for i:=0; i< (1<<gArity); i++ {
		args:= ToBits(i, gArity)
		sum += g(args...)
	}

	p.H = sum

	return p
}


func (p *InefficientProver) ComputeAndSendNextPolynomial(v *Verifier) {
	round := p.round

	gJ := func(args ...int) int {
		argsInit := append(p.randomChallenges[:round - 1], args[0])
		padLen := p.gArity - len(argsInit)

		var sum int

		for i:=0; i< (1<<padLen); i++ {
			args:= append(argsInit, ToBits(i, padLen)...)
			sum += p.g(args...)
		}
		return sum
	}

	p.polynomials = append(p.polynomials, gJ)
	v.RecievePolynomials(gJ)
	p.round++
}

func (p *InefficientProver) ReceiveChallenge(challenge int) {
	p.randomChallenges = append(p.randomChallenges, challenge)
	fmt.Printf("Received challenge %d, initiating round %d\n", challenge, p.round)
}