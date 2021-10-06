#[derive(Debug, PartialEq)]
pub enum Error {
    MalformedMerkleProof,
    InvalidMerkleProof,
    EndOfFile,
    MalformedHeader,
    MalformedTransaction,
    UnsupportedInputFormat,
    MalformedWitnessOutput,
    MalformedP2PKHOutput,
    MalformedP2SHOutput,
    UnsupportedOutputFormat,
    MalformedOpReturnOutput,
    InvalidHeaderSize,
    InvalidBtcHash,
    InvalidScript,
    InvalidBtcAddress,
    ArithmeticOverflow,
    ArithmeticUnderflow,
}
