use bitcoincore_rpc;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin::merkle::MerkleProof;
use bitcoincore_rpc::bitcoin as btc;

fn main() {
    // Parsing hex encoded proof and generating merkle root
    let proof_string: &str = "03000000cb16155c86f33d0ef7abbbc1d8a5a0b1fcddf5ee27339c11da4e0101000000008b0b1b4be488923ad68a9ef2dce38a772d6f634fb33a5d40516f1395bccf47ea2a68b55551450c1aaa2a227f06000000043ef0e95a32eea19f4fbf5bf9a72cdfad1d70ead27ec1473c20d0c3d2bbed84ea3f800ffe0b6dcdae64482c015d69efa7c5b0846cad6353db55d64d5198c756ebe40c7f91620074c70b71e9d18225f7ed3a7161c6722f7f79098deeda0372bbc7f321d7c69cbabae00b8d82ba1f0c17670d2e5ee05ce8c0643623980bedd801d7010f";
    let raw_proof = hex::decode(proof_string).unwrap();
    let proof = MerkleProof::parse(&raw_proof).unwrap();
    let result = proof.verify_proof().unwrap();

    // type casting to btc hash types
    let block_header_hash = proof.block_header.hash.to_bytes_le();
    let block_header_hash: btc::BlockHash = btc::consensus::encode::deserialize(&block_header_hash).unwrap();
    let merkle_root = result.extracted_root.to_bytes_le();
    let merkle_root: btc::TxMerkleNode = btc::consensus::encode::deserialize(&merkle_root).unwrap();

    //connecting to bitcoin node rpc
    let rpc = Client::new(
        "http://localhost:18332".to_string(),
        Auth::UserPass("bitcoin".to_string(), "Persario_1".to_string())
    ).unwrap();

    // retrieving block header and verifying the merkle root
    let block_header = rpc.get_block_header(&block_header_hash).unwrap();
    if block_header.merkle_root == merkle_root{
        println!("True")
    }
    else{
        println!("False")
    }

}




