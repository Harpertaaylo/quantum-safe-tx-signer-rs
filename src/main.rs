// [Rust] PQC-based Ethereum transaction signer

use pqcrypto_dilithium::dilithium2::{keypair, sign};
use rlp::RlpStream;

fn main() {
    let (pk, sk) = keypair();
    println!("Generated PQC keypair: {:?}", pk);

    let mut rlp = RlpStream::new();
    rlp.begin_list(3);
    rlp.append(&"to_address");
    rlp.append(&"amount_in_wei");
    rlp.append(&"nonce_value");

    let tx_data = rlp.out().to_vec();
    let signature = sign(&tx_data, &sk);

    println!("Signed tx with PQC: {:?}", signature);
}
