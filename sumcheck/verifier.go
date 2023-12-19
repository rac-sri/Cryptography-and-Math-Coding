package sumcheck

import (
	"fmt"
	"math/rand"
	"time"
)

type Verifier struct {
	g                FuncType
	gArity           int // represents the number of inputs to polynomial
	H                int // witness
	randomChallenges []int
	round            int
	polynomials      []FuncType
}

// Initialize verifier with the claimed witness H
func NewVerifier(g FuncType, gArity int, H int) *Verifier {
	return &Verifier{
		g:           g,
		gArity:      gArity,
		H:           H,
		round:       1,
		polynomials: []FuncType{},
	}
}

func (v *Verifier) RecievePolynomials(polynomial FuncType) {
	v.polynomials = append(v.polynomials, polynomial)
}

// verify that latest polynomial is a univariate polynomial of at most deg_j(g) and that
// g_{j-1}(r_{j-i}) = g_j(0) + g_j(1)
func (v *Verifier) CheckLatestPolynomial() error {
	latestPoly := v.polynomials[len(v.polynomials)-1]
	degLatest := DegJ(latestPoly, 0)
	degMax := DegJ(v.g, v.round-1)

	if degLatest > degMax {
		return fmt.Errorf("Prover sent polynomial of degree %d greater than expected : %d", degLatest, degMax)
	}

	newSum := latestPoly(0) + latestPoly(1)

	var check int

	if v.round == 1 {
		check = v.H
	} else {
		check = v.polynomials[len(v.polynomials)-2](v.randomChallenges[len(v.randomChallenges)-1])
	}

	if check != newSum {
		return fmt.Errorf("Prover sent incorrect polynomials: %d, expected %d", newSum, check)
	}

	return nil
}

func (v *Verifier) GetNewRandomValueAndSend(p *Prover) {
	rand.Seed(time.Now().UnixNano())
	v.randomChallenges = append(v.randomChallenges, rand.Intn(2))
	p.ReceiveChallenge(v.randomChallenges[len(v.randomChallenges)-1])
	v.round++
}

// Evaluate and check the final value of
func (v *Verifier) EvaluateAndCheckGV() (bool, error) {
	if len(v.randomChallenges) != v.gArity-1 {
		return false, fmt.Errorf("Incorrect number of random challenges")
	}

	v.randomChallenges = append(v.randomChallenges, rand.Intn(2))
	gFinal := v.g(v.randomChallenges...)
	check := v.polynomials[len(v.polynomials)-1](v.randomChallenges[len(v.randomChallenges)-1])

	if gFinal != check {
		return false, fmt.Errorf("Prover sent incorrect final polynomials")
	}

	fmt.Println("VERIFIER ACCEPTS")
	return true, nil
}
