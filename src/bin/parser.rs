use std::fs;
use eigenda_v2_struct_rust::v2_cert::rs_struct;
use eigenda_v2_struct_rust::v2_cert::sol_struct;
use alloy_rlp::{RlpEncodable, RlpDecodable, Decodable, Encodable};

fn main() {
    parse_batch_header("batch_header.rlp");
    parse_non_signer("non_signer.rlp");
    parse_blob_inclusion("blob_inclusion.rlp");
}


pub fn parse_batch_header(file_path: &str) {
    let mut data = fs::read(file_path)
        .expect("Should have been able to read the file");

    let batchHeader = rs_struct::BatchHeaderV2::decode(&mut data.as_slice()).unwrap();
    let batchHeader_sol = batchHeader.to_sol();

    println!("batchHeader referenceBlockNumber {:?}", batchHeader_sol.referenceBlockNumber);
    println!("batchHeaderm batchRoot {:?}", batchHeader_sol.batchRoot);
}


pub fn parse_non_signer(file_path: &str) {
    let data = fs::read(file_path)
        .expect("Should have been able to read the file");

    let non_signer = rs_struct::NonSignerStakesAndSignature::decode(&mut data.as_slice()).unwrap();
    let non_signer_sol = non_signer.to_sol();

    println!("non_signer_sol totalStakeIndices {:?}", non_signer_sol.totalStakeIndices);
}

pub fn parse_blob_inclusion(file_path: &str) {
    let data = fs::read(file_path)
        .expect("Should have been able to read the file");

    let blob_inclusion = rs_struct::BlobInclusionInfo::decode(&mut data.as_slice()).expect("decode to rust blob inclusion struct");
    let blob_inclusion_sol = blob_inclusion.to_sol();

    println!("non_signer_sol blobIndex {:?}", blob_inclusion_sol.blobIndex);
}