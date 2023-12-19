package sumcheck

import "fmt"

type SumcheckProtocol struct {
	gArity 		int
	p		 	*Prover
	v			*Verifier
	round 		int
	done		bool
}

func NewSumcheckProtocol (g FuncType) *SumcheckProtocol {
	gArity := Arity(g)

	if gArity < 1 {
		panic("Function arity must be greater than or equal to 1")
	}

	p := NewProver(g, gArity)
	v := NewVerifier(g, gArity, p.H)

	return &SumcheckProtocol{
		gArity: gArity,
		p:		p,
		v: 		v,
		round: 	1,
		done: 	false,
	}
}

func (s *SumcheckProtocol) String() string {
	return fmt.Sprintf("Protocol(round: %d, H: %d, challenges: %v)", s.round, s.p.H, s.p.randomChallenges)
}

// Advance protocol by 1 round
func (s *SumcheckProtocol) AdvanceRound () {
	if s.done {
		panic("Sumcheck protocol is finished")
	}

	s.p.ComputeAndSendNextPolynomial(s.v)
	s.v.CheckLatestPolynomial()

	if s.round == s.gArity {
		// final round
		s.done, _ = s.v.EvaluateAndCheckGV()
	} else {
		s.v.GetNewRandomValueAndSend(s.p)
		s.round++
	}
}

// Advance protocol to the end
func (s *SumcheckProtocol) AdvanceToEnd(verbose bool) {
	for !s.done {
		if verbose {
			fmt.Println("Advance Output:", s)
		}

		s.AdvanceRound()
	}
}

