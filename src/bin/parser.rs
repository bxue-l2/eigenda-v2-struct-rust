use std::{fs, result};
use alloy::eips::calc_blob_gasprice;
use alloy_sol_types::SolCall;
use eigenda_v2_struct_rust::v2_cert;
use eigenda_v2_struct_rust::v2_cert::sol_struct;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::primitives::{address, Address};

const CERT_VERIFIER: Address = address!("0xA6be9E1425CD1A49c1b5288903C54E807604d8DC");


#[tokio::main]
async fn main() {
    let batch_header_rlp = fs::read("batch_header.rlp")
        .expect("Should have been able to read the file");
    let non_signer_rlp = fs::read("non_signer.rlp")
        .expect("Should have been able to read the file");
    let blob_inclusion_rlp = fs::read("blob_inclusion.rlp")
        .expect("Should have been able to read the file");
    
    let batch_header = v2_cert::parse_batch_header(&batch_header_rlp);
    let non_signer = v2_cert::parse_non_signer(&non_signer_rlp);
    let blob_inclusion = v2_cert::parse_blob_inclusion(&blob_inclusion_rlp);
    //blob_inclusion.blobIndex = 3;


    let url = "https://eth-holesky.g.alchemy.com/v2/P4tiNCHIHYa0HnACGNUflmqeVZ6At4Ln".parse().unwrap();

    // just a random private key
    let signer = "966d6501da9ff16d1b460ea17e474b0efa27b0baf505af1162878b14e33afdf7"
        .parse::<PrivateKeySigner>().expect("valid private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let bn = provider.get_block_number().await.unwrap();
    println!("block number is {}", bn);

    /* 
    let input = sol_struct::IEigenDACertVerifier::verifyDACertV2Call {
        batchHeader: batch_header,
        blobInclusionInfo: blob_inclusion,
        nonSignerStakesAndSignature: non_signer,
    }.abi_encode();
    */
    let input = sol_struct::IEigenDACertVerifier::verifyDACertV2ForZKProofCall {
        batchHeader: batch_header,
        blobInclusionInfo: blob_inclusion,
        nonSignerStakesAndSignature: non_signer,
    }.abi_encode();

    let tx = TransactionRequest::default().with_to(CERT_VERIFIER).with_input(input);

    // as long as not error, the call is right
    let call_result = provider.call(&tx).await.expect("not revert").0;

    assert!(call_result.len() == 32);

    for (i, item) in call_result.iter().enumerate() {
        if i != 31 {
            assert!(*item == 0);
        } else {
            assert!(*item == 1);
        }
    }
    

    println!("verify V2 cert returns with {:?}", call_result[31]);
    
}


