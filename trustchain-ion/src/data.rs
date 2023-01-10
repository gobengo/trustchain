#![allow(dead_code)]

// Note on test fixtures:
//
// This file contains samples of content from the three ION file types written to IPFS:
// 1. coreIndexFile
// 2. provisionalIndexFile
// 3. chunkFile
//
// The samples contain content associated with the following DIDs:
// ROOT DID:    "did:ion:test:EiCClfEdkTv_aM3UnBBhlOV89LlGhpQAbfeZLFdFxVFkEg"
// ROOT+1 DID:  "did:ion:test:EiBVpjUxXeSRJpvj2TewlX9zNF3GKMCKWwGmKBZqF6pk_A"
// ROOT+2 DID:  "did:ion:test:EiAtHHKFJWAk5AsM3tgCut3OiBY4ekHTf66AAjoysXL65Q"
//
// The OP_RETURN data for the create operation is:
// ion:3.QmRvgZm4J3JSxfk4wRjE2u2Hi2U7VmobYnpqhqH5QP6J97
//
// The Bitcoin transaction containing this OP_RETURN data has TxID:
// 9dc43cca950d923442445340c2e30bc57761a62ef3eaf2417ec5c75784ea9c2c
//
// The IPFS CID (for the coreIndexFile) is:
// QmRvgZm4J3JSxfk4wRjE2u2Hi2U7VmobYnpqhqH5QP6J97

// Sample ION coreIndexFile content (see https://identity.foundation/sidetree/spec/#core-index-file).
pub const TEST_CORE_INDEX_FILE_CONTENT: &'static str = r#"{"provisionalIndexFileUri":"QmfXAa2MsHspcTSyru4o1bjPQELLi62sr2pAKizFstaxSs","operations":{"create":[{"suffixData":{"deltaHash":"EiBkAX9y-Ts_siMzTzkfAzPKPIIbB033PlF0RlvF97ydJg","recoveryCommitment":"EiCymv17OGBAs7eLmm4BIXDCQBVhdOUAX5QdpIrN4SDE5w"}},{"suffixData":{"deltaHash":"EiBBkv0j587BDSTjJtIv2DJFOOHk662n9Uoh1vtBaY3JKA","recoveryCommitment":"EiClOaWycGv1m-QejUjB0L18G6DVFVeTQCZCuTRrmzCBQg"}},{"suffixData":{"deltaHash":"EiDTaFAO_ae63J4LMApAM-9VAo8ng58TTp2K-2r1nek6lQ","recoveryCommitment":"EiCy4pW16uB7H-ijA6V6jO6ddWfGCwqNcDSJpdv_USzoRA"}}]}}"#;

// Sample ION coreIndexFile content (see https://identity.foundation/sidetree/spec/#provisional-index-file).
pub const TEST_PROVISIONAL_INDEX_FILE_CONTENT: &'static str =
    r#"{"chunks":[{"chunkFileUri":"QmWeK5PbKASyNjEYKJ629n6xuwmarZTY6prd19ANpt6qyN"}]}"#;

// Sample ION chunk file content (see https://identity.foundation/sidetree/spec/#chunk-files).
pub const TEST_CHUNK_FILE_CONTENT: &'static str = r#"{"deltas":[{"patches":[{"action":"replace","document":{"publicKeys":[{"id":"9CMTR3dvGvwm6KOyaXEEIOK8EOTtek-n7BV9SVBr2Es","type":"JsonWebSignature2020","publicKeyJwk":{"crv":"secp256k1","kty":"EC","x":"7ReQHHysGxbyuKEQmspQOjL7oQUqDTldTHuc9V3-yso","y":"kWvmS7ZOvDUhF8syO08PBzEpEk3BZMuukkvEJOKSjqE"},"purposes":["assertionMethod","authentication","keyAgreement","capabilityInvocation","capabilityDelegation"]}],"services":[{"id":"TrustchainID","type":"Identity","serviceEndpoint":"https://identity.foundation/ion/trustchain-root"}]}}],"updateCommitment":"EiDVRETvZD9iSUnou-HUAz5Ymk_F3tpyzg7FG1jdRG-ZRg"},{"patches":[{"action":"replace","document":{"publicKeys":[{"id":"kjqrr3CTkmlzJZVo0uukxNs8vrK5OEsk_OcoBO4SeMQ","type":"JsonWebSignature2020","publicKeyJwk":{"crv":"secp256k1","kty":"EC","x":"aApKobPO8H8wOv-oGT8K3Na-8l-B1AE3uBZrWGT6FJU","y":"dspEqltAtlTKJ7cVRP_gMMknyDPqUw-JHlpwS2mFuh0"},"purposes":["assertionMethod","authentication","keyAgreement","capabilityInvocation","capabilityDelegation"]}],"services":[{"id":"TrustchainID","type":"Identity","serviceEndpoint":"https://identity.foundation/ion/trustchain-root-plus-1"}]}}],"updateCommitment":"EiC0EdwzQcqMYNX_3aqoZNUau4AKOL3gXQ5Pz3ATi1q_iA"},{"patches":[{"action":"replace","document":{"publicKeys":[{"id":"ePyXsaNza8buW6gNXaoGZ07LMTxgLC9K7cbaIjIizTI","type":"JsonWebSignature2020","publicKeyJwk":{"crv":"secp256k1","kty":"EC","x":"0nnR-pz2EZGfb7E1qfuHhnDR824HhBioxz4E-EBMnM4","y":"rWqDVJ3h16RT1N-Us7H7xRxvbC0UlMMQQgxmXOXd4bY"},"purposes":["assertionMethod","authentication","keyAgreement","capabilityInvocation","capabilityDelegation"]}],"services":[{"id":"TrustchainID","type":"Identity","serviceEndpoint":"https://identity.foundation/ion/trustchain-root-plus-2"}]}}],"updateCommitment":"EiBDfsKvBaSAYO8Hp77eP9NHOpUWRMhcUNMJNHTDWQNw2w"}]}"#;
