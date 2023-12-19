package sumcheck

import "testing"

func TestSumcheck(t *testing.T) {
	g := func(args ...int) int 	{ 
		a := args[0]
		return a + a + a*a }

	protocol := NewSumcheckProtocol(g)
	protocol.AdvanceToEnd(true)

	f := func(args ...int) int { 	
		a := args[0]
		return a*a*a + a + a 
	}

	protocol = NewSumcheckProtocol(f)
	protocol.AdvanceToEnd(true)

	ff := func(args ...int) int { 
		a := args[0]
		return a*a*a + a + a + a*a }
	protocol = NewSumcheckProtocol(ff)
	protocol.AdvanceToEnd(true)
}