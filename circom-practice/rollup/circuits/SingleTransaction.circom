pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/mimc.circom";
include "../node_modules/circomlib/circuits/eddsamimc.circom";
 
template SingleTransaction(n) {
    signal input current_state;
    signal input initialOnChainRoot;

    signal input paths2old_root_from[n-1];
    signal input paths2old_root_to[n-1];
    signal input paths2new_root_from[n-1];
    signal input paths2new_root_to[n-1];
    signal input paths2root_from_pos[n-1];
    signal input paths2root_to_pos[n-1];
    
    signal input pub_from_x;
    signal input pub_from_y;
    signal input R8x;
    signal input R8y;
    signal input S;
    signal input amount;

    signal input nonce_from;
    signal input nonce_to;
    
    signal input pub_to_x;
    signal input pub_to_y;
    signal input token_balance_from;
    signal output out;

    var i;

    var NONCE_MAX_VALUE = 1000000000000000000;
    
    component old_hash_from = MultiMiMC7(7,91);
    old_hash_from.in[0] <== initialOnChainRoot;
    old_hash_from.in[1] <== pub_from_x;
    old_hash_from.in[2] <== pub_from_y;
    old_hash_from.in[3] <== nonce_from;
    old_hash_from.in[4] <== amount;
    old_hash_from.in[5] <== pub_to_x;
    old_hash_from.in[6] <== pub_to_y;
    old_hash_from.k <== 0;
    
    component old_merkle_from[n-1];
    old_merkle_from[0] = MultiMiMC7(2,91);
    old_merkle_from[0].in[0] <== old_hash_from.out - paths2root_from_pos[0]* (old_hash_from.out - paths2old_root_from[0]);
    old_merkle_from[0].in[1] <== paths2old_root_from[0] - paths2root_from_pos[0]* (paths2old_root_from[0] - old_hash_from.out);
    old_merkle_from[0].k     <== 0 ;
    for (i=1; i<n-1; i++){
     old_merkle_from[i] = MultiMiMC7(2,91);
     old_merkle_from[i].in[0] <== old_merkle_from[i-1].out - paths2root_from_pos[i]* (old_merkle_from[i-1].out - paths2old_root_from[i]);
     old_merkle_from[i].in[1] <== paths2old_root_from[i] - paths2root_from_pos[i]* (paths2old_root_from[i] - old_merkle_from[i-1].out);
        old_merkle_from[i].k     <== 0 ;
    }

    current_state === old_merkle_from[n-2].out;

    component old_hash_to = MultiMiMC7(4,91);
    old_hash_to.in[0] <== initialOnChainRoot;
    old_hash_to.in[1] <== pub_to_x;
    old_hash_to.in[2] <== pub_to_y;
    old_hash_to.in[3] <== nonce_to;
    old_hash_to.k     <== 0;

    component old_merkle_to[n-1];
    old_merkle_to[0] = MultiMiMC7(2,91);
    old_merkle_to[0].in[0] <== old_hash_to.out - paths2root_to_pos[0]* (old_hash_to.out - paths2old_root_to[0]);
    old_merkle_to[0].in[1] <== paths2old_root_to[0] - paths2root_to_pos[0]* (paths2old_root_to[0] - old_hash_to.out);
    old_merkle_to[0].k     <== 0; 
    for (i=1; i<n-1; i++){
     old_merkle_to[i] = MultiMiMC7(2,91);
     old_merkle_to[i].in[0] <== old_merkle_to[i-1].out - paths2root_to_pos[i]* (old_merkle_to[i-1].out - paths2old_root_to[i]);
     old_merkle_to[i].in[1] <== paths2old_root_to[i] - paths2root_to_pos[i]* (paths2old_root_to[i] - old_merkle_to[i-1].out);
        old_merkle_to[i].k     <== 0 ;  
    }
    
    current_state === old_merkle_to[n-2].out;

    component verifier = EdDSAMiMCVerifier();   
    verifier.enabled <== 1;
    verifier.Ax <== pub_from_x;
    verifier.Ay <== pub_from_y;
    verifier.R8x <== R8x ;
    verifier.R8y <== R8y ;
    verifier.S <== S;
    verifier.M <== old_hash_from.out;

    component greFrom = GreaterEqThan (252) ;
    greFrom.in[0] <== token_balance_from ;
    greFrom.in[1] <== amount ;
    greFrom.out === 1 ;

    // component greTo = GreaterEqThan (252) ;
    // greTo.in[0] <== token_balance_to + amount ;
    // greTo.in[1] <== token_balance_to ;
    // greTo.out === 1 ;

    component nonceCheck = GreaterEqThan (252) ;
    nonceCheck.in[0] <== NONCE_MAX_VALUE ;
    nonceCheck.in[1] <== nonce_from ;
    nonceCheck.out === 1 ;

    // // accounts updates
    component new_hash_from = MultiMiMC7(6,91);
    new_hash_from.in[0] <== pub_from_x;
    new_hash_from.in[1] <== pub_from_y;
    new_hash_from.in[2] <== nonce_from + 1;
    new_hash_from.in[3] <== amount;
    new_hash_from.in[4] <== pub_to_x;
    new_hash_from.in[5] <== pub_to_y;
    new_hash_from.k     <== 0 ;
   
 component new_merkle_from[n-1];
    new_merkle_from[0] = MultiMiMC7(2,91);
    new_merkle_from[0].in[0] <== new_hash_from.out - paths2root_from_pos[0]* (new_hash_from.out - paths2new_root_from[0]);
    new_merkle_from[0].in[1] <== paths2new_root_from[0] - paths2root_from_pos[0]* (paths2new_root_from[0] - new_hash_from.out);
    new_merkle_from[0].k     <== 0 ;
    for (i=1; i<n-1; i++){
     new_merkle_from[i] = MultiMiMC7(2,91);
     new_merkle_from[i].in[0] <== new_merkle_from[i-1].out - paths2root_from_pos[i]* (new_merkle_from[i-1].out - paths2new_root_from[i]);
     new_merkle_from[i].in[1] <== paths2new_root_from[i] - paths2root_from_pos[i]* (paths2new_root_from[i] - new_merkle_from[i-1].out);
        new_merkle_from[i].k     <== 0 ;
    }

    component new_hash_to = MultiMiMC7(4,91);
    new_hash_to.in[0] <== pub_to_x;
    new_hash_to.in[1] <== pub_to_y;
    new_hash_to.in[2] <== nonce_to + 1;
    new_hash_to.in[3] <== amount;
    new_hash_to.k     <== 0 ;
// log(new_hash_to.out) ;    
 component new_merkle_to[n-1];
    new_merkle_to[0] = MultiMiMC7(2,91);
    new_merkle_to[0].in[0] <== new_hash_to.out - paths2root_to_pos[0]* (new_hash_to.out - paths2new_root_to[0]);
    new_merkle_to[0].in[1] <== paths2new_root_to[0] - paths2root_to_pos[0]* (paths2new_root_to[0] - new_hash_to.out);
    new_merkle_to[0].k     <== 0;
    for (i=1; i<n-1; i++){
     new_merkle_to[i] = MultiMiMC7(2,91);
     new_merkle_to[i].in[0] <== new_merkle_to[i-1].out - paths2root_to_pos[i]* (new_merkle_to[i-1].out - paths2new_root_to[i]);
     new_merkle_to[i].in[1] <== paths2new_root_to[i] - paths2root_to_pos[i]* (paths2new_root_to[i] - new_merkle_to[i-1].out);
        new_merkle_to[i].k     <== 0;
    }
    new_merkle_from[n-2].out === new_merkle_to[n-2].out ;
    
    out <== new_merkle_to[n-2].out;
}

// component main  = SingleTransaction(2);