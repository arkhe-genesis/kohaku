pragma circom 2.0.0;

template FordefiBridge() {
    signal input operation_id;
    signal input vault_id;
    signal input merkle_root;

    signal output out;
    out <== merkle_root;
}

component main = FordefiBridge();
