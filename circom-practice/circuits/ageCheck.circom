pragma circom 2.0.5;

include "../node_modules/circomlib/circuits/comparators.circom";

template AgeCheck () {
    signal input age;
    signal input ageLimit;
    signal output isAgeAboveLimit;

    component greaterThan = GreaterEqThan(7); // we will work with 7 bit numbers because [0, 2^7 - 1] [0, 127]

    greaterThan.in[0] <== age;
    greaterThan.in[1] <== ageLimit;

    isAgeAboveLimit <== greaterThan.out;


}

component main {public [ageLimit]} = AgeCheck();