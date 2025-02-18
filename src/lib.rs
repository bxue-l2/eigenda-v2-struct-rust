pub mod sol_struct;

use alloy_primitives::Bytes;
use alloy_primitives::{FixedBytes, U256};
use alloy_rlp::{Decodable, RlpDecodable, RlpEncodable};

// G1Point represents a point on the BN254 G1 curve
#[derive(Debug, Clone, Copy, RlpEncodable, RlpDecodable)]
pub struct G1Point {
    //pub x: [u8; 32],
    pub x: U256,
    pub y: U256,
}

impl G1Point {
    pub fn to_sol(&self) -> sol_struct::G1Point {
        sol_struct::G1Point {
            X: self.x,
            Y: self.y,
        }
    }
}

// G2Point represents a point on the BN254 G2 curve
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct G2Point {
    pub x: Vec<U256>,
    pub y: Vec<U256>,
}

impl G2Point {
    pub fn to_sol(&self) -> sol_struct::G2Point {
        let mut x = [U256::default(); 2];
        x[0] = self.x[0];
        x[1] = self.x[1];

        let mut y = [U256::default(); 2];
        y[0] = self.y[0];
        y[1] = self.y[1];

        sol_struct::G2Point { X: x, Y: y }
    }
}

// BlobCommitment contains commitment information for a blob
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct BlobCommitment {
    pub commitment: G1Point,
    pub length_commitment: G2Point,
    pub length_proof: G2Point,
    pub length: u32,
}

impl BlobCommitment {
    pub fn to_sol(&self) -> sol_struct::BlobCommitment {
        sol_struct::BlobCommitment {
            commitment: self.commitment.to_sol(),
            lengthCommitment: self.length_commitment.to_sol(),
            lengthProof: self.length_proof.to_sol(),
            length: self.length,
        }
    }
}

// BlobHeaderV2 is the version 2 of blob header
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct BlobHeaderV2 {
    pub version: u16,
    pub quorum_numbers: Bytes,
    pub commitment: BlobCommitment,
    pub payment_header_hash: [u8; 32],
    pub salt: u32,
}

impl BlobHeaderV2 {
    pub fn to_sol(&self) -> sol_struct::BlobHeaderV2 {
        sol_struct::BlobHeaderV2 {
            version: self.version,
            quorumNumbers: Bytes::copy_from_slice(&self.quorum_numbers),
            paymentHeaderHash: FixedBytes::<32>(self.payment_header_hash),
            salt: self.salt,
            commitment: self.commitment.to_sol(),
        }
    }
}

// BlobCertificate contains certification information for a blob
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct BlobCertificate {
    pub blob_header: BlobHeaderV2,
    pub signature: Bytes,
    pub relay_keys: Vec<u32>,
}

impl BlobCertificate {
    pub fn to_sol(&self) -> sol_struct::BlobCertificate {
        sol_struct::BlobCertificate {
            signature: self.signature.clone(),
            relayKeys: self.relay_keys.clone(),
            blobHeader: self.blob_header.to_sol(),
        }
    }
}

// BlobInclusionInfo contains inclusion proof information for a blob
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct BlobInclusionInfo {
    pub blob_certificate: BlobCertificate,
    pub blob_index: u32,
    pub inclusion_proof: Bytes,
}

// BatchHeaderV2 is the version 2 of batch header
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct BatchHeaderV2 {
    pub batch_root: [u8; 32],
    pub reference_block_number: u32,
}
// NonSignerStakesAndSignature contains information about non-signers and their stakes
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct NonSignerStakesAndSignature {
    pub non_signer_quorum_bitmap_indices: Vec<u32>,
    pub non_signer_pubkeys: Vec<G1Point>,
    pub quorum_apks: Vec<G1Point>,
    pub apk_g2: G2Point,
    pub sigma: G1Point,
    pub quorum_apk_indices: Vec<u32>,
    pub total_stake_indices: Vec<u32>,
    pub non_signer_stake_indices: Vec<Vec<u32>>,
}

impl BatchHeaderV2 {
    pub fn to_sol(&self) -> sol_struct::BatchHeaderV2 {
        sol_struct::BatchHeaderV2 {
            batchRoot: FixedBytes::<32>(self.batch_root),
            referenceBlockNumber: self.reference_block_number,
        }
    }
}

impl BlobInclusionInfo {
    pub fn to_sol(&self) -> sol_struct::BlobInclusionInfo {
        sol_struct::BlobInclusionInfo {
            blobIndex: self.blob_index,
            inclusionProof: self.inclusion_proof.clone(),
            blobCertificate: self.blob_certificate.to_sol(),
        }
    }
}

impl NonSignerStakesAndSignature {
    pub fn to_sol(&self) -> sol_struct::NonSignerStakesAndSignature {
        sol_struct::NonSignerStakesAndSignature {
            nonSignerQuorumBitmapIndices: self.non_signer_quorum_bitmap_indices.clone(),
            nonSignerPubkeys: self.non_signer_pubkeys.iter().map(|e| e.to_sol()).collect(),
            quorumApks: self
                .quorum_apks
                .iter()
                .map(|e| sol_struct::G1Point { X: e.x, Y: e.y })
                .collect(),
            apkG2: self.apk_g2.to_sol(),
            sigma: self.sigma.to_sol(),
            quorumApkIndices: self.quorum_apk_indices.clone(),
            totalStakeIndices: self.total_stake_indices.clone(),
            nonSignerStakeIndices: self.non_signer_stake_indices.clone(),
        }
    }
}

pub fn parse_batch_header(data: &Vec<u8>) -> sol_struct::BatchHeaderV2 {
    BatchHeaderV2::decode(&mut data.as_slice())
        .unwrap()
        .to_sol()
}

pub fn parse_non_signer(data: &Vec<u8>) -> sol_struct::NonSignerStakesAndSignature {
    NonSignerStakesAndSignature::decode(&mut data.as_slice())
        .unwrap()
        .to_sol()
}

pub fn parse_blob_inclusion(data: &Vec<u8>) -> sol_struct::BlobInclusionInfo {
    BlobInclusionInfo::decode(&mut data.as_slice())
        .expect("decode to rust blob inclusion struct")
        .to_sol()
}

/// EigenDAV2Cert to be updatd in the solidity
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct EigenDAV2Cert {
    pub batch_header_v2: BatchHeaderV2,
    pub nonsigner_stake_and_signature: NonSignerStakesAndSignature,
    pub blob_inclusion_info: BlobInclusionInfo,
}
