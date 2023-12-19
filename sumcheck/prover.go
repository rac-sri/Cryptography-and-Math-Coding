package sumcheck

import "fmt"

type Prover struct {
	gArity 				int
	randomChallenges 	[]int
	cachedPolynomials	[]FuncType
	round 				int
	H 	  				int
}

func NewProver(g FuncType, gArity int) *Prover {
	p:= &Prover {
		gArity: 					gArity,
		cachedPolynomials: 			[]FuncType{g},
		round : 					1,
	}

	// compute witness H
	var sum int
	for i:=0; i< (1<<gArity); i++ {
		args := ToBits(i, gArity)
		sum += g(args...)
	}

	p.H = sum

	return p
}

// computes the next polynomial and sends it to the verifier. and update the round
func (p *Prover) ComputeAndSendNextPolynomial(v *Verifier) {
	round := p.round
	poly := p.cachedPolynomials[len(p.cachedPolynomials)-1]

	gJ := func (args ...int) int {
		pad := p.gArity - round
		var sum int
		for i:= 0; i< (1<<pad); i++ {
			args := append([]int{args[0]}, ToBits(i, pad)...)
			sum += poly(args...)
		}
		return sum
	}

	v.RecievePolynomials(FuncType(gJ))
	p.round++
}


func (p *Prover) ReceiveChallenge(challenge int) {
	p.randomChallenges = append(p.randomChallenges, challenge)
	p.CacheNext(challenge)
	fmt.Printf("Received challenge %d, initiating round %d\n",challenge, p.round)
}

func (p *Prover) CacheNext (challenge int) {
	poly := p.cachedPolynomials[len(p.cachedPolynomials)-1]

	nextPoly := func(args ...int) int {
		return poly(append([]int{challenge},args...)...)
	}

	p.cachedPolynomials = append(p.cachedPolynomials, nextPoly)
}
