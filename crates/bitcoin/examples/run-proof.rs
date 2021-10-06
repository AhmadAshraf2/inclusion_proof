extern crate bitcoin;

use bitcoin::merkle::MerkleProof;

// Proving that the transaction
// 8d30eb0f3e65b8d8a9f26f6f73fc5aafa5c0372f9bb38aa38dd4c9dd1933e090
// is included in block 0000000000000a3290f20e75860d505ce0e948a1d1d846bec7e39015d242884b
// https://www.blockchain.com/btc/block/150000
// curl -s -d '{"jsonrpc": "1.0", "id":"test", "method": "gettxoutproof", "params": [["8d30eb0f3e65b8d8a9f26f6f73fc5aafa5c0372f9bb38aa38dd4c9dd1933e090"], "0000000000000a3290f20e75860d505ce0e948a1d1d846bec7e39015d242884b"] }' http://satoshi.doc.ic.ac.uk:8332

const PROOF_HEX: &str = "03000000cb16155c86f33d0ef7abbbc1d8a5a0b1fcddf5ee27339c11da4e0101000000008b0b1b4be488923ad68a9ef2dce38a772d6f634fb33a5d40516f1395bccf47ea2a68b55551450c1aaa2a227f06000000043ef0e95a32eea19f4fbf5bf9a72cdfad1d70ead27ec1473c20d0c3d2bbed84ea3f800ffe0b6dcdae64482c015d69efa7c5b0846cad6353db55d64d5198c756ebe40c7f91620074c70b71e9d18225f7ed3a7161c6722f7f79098deeda0372bbc7f321d7c69cbabae00b8d82ba1f0c17670d2e5ee05ce8c0643623980bedd801d7010f";

fn main() {
    let raw_proof = hex::decode(PROOF_HEX).unwrap();
    let proof = MerkleProof::parse(&raw_proof).unwrap();
    let result = proof.verify_proof().unwrap();
    println!(
        "proof: transactions count = {}, hash count = {}, tree height = {},\nmerkle root = {:?}, hashes count = {}, flags={:?},\ncomputed merkle root = {}, position = {}",
        proof.transactions_count,
        proof.hashes.len(),
        proof.compute_partial_tree_height(),
        proof.block_header.merkle_root,
        proof.hashes.len(),
        proof.flag_bits,
        result.extracted_root,
        result.transaction_position
    );
}
