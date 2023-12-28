pragma circom 2.0.8;
include "./SingleTransaction.circom";

template MultipleTransaction2(transactionCount, n) {

    signal input intermediate_state[transactionCount];
    signal input final_state_roots[transactionCount];

    signal input initialOnChainRoot;
    signal input finalOnChainRoot;
    
    signal input paths2old_root_from[transactionCount][n-1];
    signal input paths2old_root_to[transactionCount][n-1];
    signal input paths2new_root_from[transactionCount][n-1];
    signal input paths2new_root_to[transactionCount][n-1];
    signal input paths2root_from_pos[transactionCount][n-1];
    signal input paths2root_to_pos[transactionCount][n-1];

    signal input pub_from[transactionCount][2];
    signal input pub_to[transactionCount][2];
    signal input nonce_from[transactionCount];
    signal input nonce_to[transactionCount];
    signal input amount[transactionCount];

    signal input R8x[transactionCount];
    signal input R8y[transactionCount];
    signal input S[transactionCount];
    signal input token_balance_from[transactionCount];

    signal output out;

    component transaction[transactionCount];
    for (var i = 0; i < transactionCount; i++){
        transaction[i] = SingleTransaction(n);

        transaction[i].initialOnChainRoot <== initialOnChainRoot;
        transaction[i].current_state <== intermediate_state[i];

        for (var j = 0; j < n-1; j++){
            transaction[i].paths2old_root_from[j] <== paths2old_root_from[i][j];
            transaction[i].paths2old_root_to[j] <== paths2old_root_to[i][j];
            transaction[i].paths2new_root_from[j] <== paths2new_root_from[i][j];
            transaction[i].paths2new_root_to[j] <== paths2new_root_to[i][j];
            transaction[i].paths2root_from_pos[j] <== paths2root_from_pos[i][j];
            transaction[i].paths2root_to_pos[j] <== paths2root_to_pos[i][j];
        }
        transaction[i].pub_from_x <== pub_from[i][0];
        transaction[i].pub_from_y <== pub_from[i][1];
        transaction[i].nonce_from <== nonce_from[i];
        transaction[i].pub_to_x <== pub_to[i][0];
        transaction[i].pub_to_y <== pub_to[i][1];
        transaction[i].nonce_to <== nonce_to[i];

        transaction[i].R8x <== R8x[i];
        transaction[i].R8y <== R8y[i];
        transaction[i].S <== S[i];
        transaction[i].amount <== amount[i];
        transaction[i].token_balance_from <== token_balance_from[i];

        final_state_roots[i] === transaction[i].out;

    }

    component state_root = MultiMiMC7(transactionCount,91);
    for (var i = 0; i<transactionCount; i++){
        state_root.in[i] <== transaction[i].out;
    }
    state_root.k <== 0;
    finalOnChainRoot === state_root.out ;
    out <== 1;
}
// component main = MultipleTransaction2(2, 2);
component main {
    public [
        initialOnChainRoot,
        finalOnChainRoot,
        pub_from,
        pub_to,
        amount,
        token_balance_from,
        nonce_from,
        nonce_to
    ]
} = MultipleTransaction2(2, 2);