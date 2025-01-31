use std::fs;
use alloy_sol_types::SolCall;
use eigenda_v2_struct_rust::v2_cert::rs_struct;
use eigenda_v2_struct_rust::v2_cert::sol_struct;
use alloy_rlp::{RlpEncodable, RlpDecodable, Decodable, Encodable};
use alloy_rpc_client::RpcClient;
use alloy::providers::{Provider, ProviderBuilder};
use alloy_transport_http::Http;
use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy_signer::{Signer, SignerSync};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::{primitives::{address, Address}};

const CERT_VERIFIER: Address = address!("0x5c33Ce64EE04400fD593F960d63336F1B65bF77B");

#[tokio::main]
async fn main() {
    let batch_header = parse_batch_header("batch_header.rlp");
    let non_signer = parse_non_signer("non_signer.rlp");
    let blob_inclusion = parse_blob_inclusion("blob_inclusion.rlp");

    let url = "https://ethereum-holesky.publicnode.com".parse().unwrap();

    // just a random private key
    let signer = "966d6501da9ff16d1b460ea17e474b0efa27b0baf505af1162878b14e33afdf7"
        .parse::<PrivateKeySigner>().expect("valid private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let bn = provider.get_block_number().await.unwrap();
    println!("block number is {}", bn);

    let input = sol_struct::IEigenDACertVerifier::verifyDACertV2Call {
        batchHeader: batch_header,
        blobInclusionInfo: blob_inclusion,
        nonSignerStakesAndSignature: non_signer,
    }.abi_encode();

    let tx = TransactionRequest::default().with_to(CERT_VERIFIER).with_input(input);

    // as long as not error, the call is right
    let _ = provider.call(&tx).await.expect("not revert");

    println!("verify V2 cert onchain is correct");
    
}


pub fn parse_batch_header(file_path: &str) -> sol_struct::BatchHeaderV2 {
    let mut data = fs::read(file_path)
        .expect("Should have been able to read the file");

    let batchHeader = rs_struct::BatchHeaderV2::decode(&mut data.as_slice()).unwrap();
    let mut batchHeader_sol = batchHeader.to_sol();

    println!("batchHeader referenceBlockNumber {:?}", batchHeader_sol.referenceBlockNumber);
    println!("batchHeaderm batchRoot {:?}", batchHeader_sol.batchRoot);

    batchHeader_sol
}


pub fn parse_non_signer(file_path: &str) -> sol_struct::NonSignerStakesAndSignature{
    let data = fs::read(file_path)
        .expect("Should have been able to read the file");

    let non_signer = rs_struct::NonSignerStakesAndSignature::decode(&mut data.as_slice()).unwrap();
    let non_signer_sol = non_signer.to_sol();

    println!("non_signer_sol totalStakeIndices {:?}", non_signer_sol.totalStakeIndices);
    non_signer_sol
}

pub fn parse_blob_inclusion(file_path: &str) -> sol_struct::BlobInclusionInfo {
    let data = fs::read(file_path)
        .expect("Should have been able to read the file");

    let blob_inclusion = rs_struct::BlobInclusionInfo::decode(&mut data.as_slice()).expect("decode to rust blob inclusion struct");
    let blob_inclusion_sol = blob_inclusion.to_sol();

    println!("non_signer_sol blobIndex {:?}", blob_inclusion_sol.blobIndex);
    blob_inclusion_sol
}