use crate::chain::{Chain, DIDChain};
use crate::resolver::{Resolver, ResolverError};
use crate::utils::canonicalize;
use crate::{controller, ROOT_EVENT_TIME};
use serde_json::to_string_pretty as to_json;
use ssi::did::{VerificationMethod, VerificationMethodMap};
use ssi::did_resolve::Metadata;
use ssi::did_resolve::ResolutionMetadata;
use ssi::jwk::{Base64urlUInt, ECParams, Params, JWK};
use ssi::one_or_many::OneOrMany;
use ssi::{
    did::Document,
    did_resolve::{DIDResolver, DocumentMetadata},
    ldp::JsonWebSignature2020,
};
use std::collections::HashMap;
use thiserror::Error;

/// An error relating to Trustchain verification.
#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerifierError {
    /// Invalid payload in proof compared to resolved document.
    #[error("Invalid payload provided in proof for dDID: {0}.")]
    InvalidPayload(String),
    /// Invalid payload in proof compared to resolved document.
    #[error("Invalid signature for proof in dDID: {0}.")]
    InvalidSignature(String),
    /// Invalid root DID after self-controller reached in path.
    #[error("Invalid root DID: {0}.")]
    InvalidRoot(String),
    /// DID not resolvable.
    #[error("DID: {0} is not resolvable.")]
    UnresolvableDID(String),
    /// Failed to build DID chain.
    #[error("Failed to build chain: {0}.")]
    ChainBuildFailure(String),
    /// Chain verification failed.
    #[error("Chain verification failed: {0}.")]
    InvalidChain(String),
    /// No proof value present.
    #[error("No proof could be retrieved from document metadata.")]
    FailureToGetProof,
    /// Failure to get controller from document.
    #[error("No controller could be retrieved from document.")]
    FailureToGetController,
}

/// Verifier of root and downstream DIDs.
trait Verifier {
    /// Verify a downstream DID by tracing its chain back to the root.
    fn verify(&self, did: &str, root_timestamp: u32) -> Result<(), VerifierError>;
    /// Get the verified timestamp for a DID as a Unix time.
    fn verified_timestamp(&self, did: &str) -> u32;
    // /// Get the resolver used for DID verification.
    // fn resolver(&self) -> Resolver<T>;
}

/// Type for resolver result.
type ResolverResult = Result<
    (
        ResolutionMetadata,
        Option<Document>,
        Option<DocumentMetadata>,
    ),
    ResolverError,
>;

/// Struct for TrustchainVerifier
pub struct TrustchainVerifier<T>
where
    T: Sync + Send + DIDResolver,
{
    resolver: Resolver<T>,
    // visited: HashMap<String, ResolverResult>,
}

impl<T> TrustchainVerifier<T>
where
    T: Send + Sync + DIDResolver,
{
    /// Construct a new TrustchainVerifier.
    pub fn new(resolver: Resolver<T>) -> Self {
        // Result<Self, VerifierError> {
        // let visited = HashMap::<String, ResolverResult>::new();
        // Ok(Self { resolver, visited })
        Self { resolver }
    }
}

// TODO: the functions below need completing. Comments:
//   - Some are already implemented in resolver.
//   - Some may benefit from being part of a struct impl.

/// Gets controller from the passed document.
fn get_controller(doc: &Document) -> Result<String, VerifierError> {
    // Get property set
    if let Some(OneOrMany::One(controller)) = doc.controller.as_ref() {
        Ok(controller.to_string())
    } else {
        Err(VerifierError::FailureToGetController)
    }
}
/// Gets proof from DocumentMetadata.
fn get_proof(doc_meta: &DocumentMetadata) -> Result<&str, VerifierError> {
    // Get property set
    if let Some(property_set) = doc_meta.property_set.as_ref() {
        // Get proof
        if let Some(Metadata::Map(proof)) = property_set.get("proof") {
            // Get proof value
            if let Some(Metadata::String(proof_value)) = proof.get("proofValue") {
                Ok(proof_value)
            } else {
                Err(VerifierError::FailureToGetProof)
            }
        } else {
            Err(VerifierError::FailureToGetProof)
        }
    } else {
        Err(VerifierError::FailureToGetProof)
    }
}

/// TODO: Extract payload from JWS
fn decode(proof_value: &JsonWebSignature2020) -> String {
    todo!()
}

// TODO: Hash a canonicalized object
fn hash(canonicalized_value: &str) -> String {
    todo!()
}

/// Extracts vec of public keys from a doc.
fn extract_keys(doc: &Document) -> Vec<JWK> {
    let mut public_keys: Vec<JWK> = Vec::new();
    if let Some(verification_methods) = doc.verification_method.as_ref() {
        for verification_method in verification_methods {
            if let VerificationMethod::Map(VerificationMethodMap {
                public_key_jwk: Some(key),
                ..
            }) = verification_method
            {
                public_keys.push(key.clone());
            } else {
                continue;
            }
        }
    }
    public_keys
}

// TODO: Check whether correct signature on proof_value given vec of public keys
fn verify_jws(proof_value: &JsonWebSignature2020, public_keys: &Vec<JWK>) -> bool {
    todo!()
}

// TODO: Get the created at time from document metadata for comparison with ROOT_EVENT_TIME
fn get_created_at(doc_meta: &DocumentMetadata) -> u64 {
    todo!()
}

impl<T> Verifier for TrustchainVerifier<T>
where
    T: Send + Sync + DIDResolver,
{
    fn verify(&self, did: &str, root_timestamp: u32) -> Result<(), VerifierError> {
        // Build a DID chain from the given DID to the root.
        let chain = match DIDChain::new(did, &self.resolver) {
            Ok(x) => x,
            Err(e) => return Err(VerifierError::ChainBuildFailure(e.to_string())),
        };

        // Verify the proofs in the chain.
        match chain.verify_proofs() {
            Ok(_) => (),
            Err(e) => return Err(VerifierError::InvalidChain(e.to_string())),
        };

        // Verify the root timestamp.
        let root = chain.root();
        if self.verified_timestamp(root) != root_timestamp {
            return Err(VerifierError::InvalidRoot(root.to_string()));
        }
        Ok(())
    }

    // TODO: re-instate (most of) the logic below into the DIDChain::verify method.

    // /// Verifies a dDID by following a chain Performs search from did upwards to root node.
    // fn verify(&mut self, did: &str) -> Result<(), VerifierError> {

    //     // Clear visited hashmap
    //     self.visited.clear();

    //     // Set downstream DID as passed did
    //     let mut ddid: String = did.to_string();

    //     // Begin loop up tree until root is reached or an error occurs
    //     loop {
    //         // Resolve current dDID (either get hashmap entry or resolve)
    //         let ddid_resolution = self
    //             .visited
    //             .entry(ddid.clone())
    //             .or_insert(self.resolver.resolve_as_result(&ddid));

    //         if let Ok((_, Some(ddoc), Some(ddoc_meta))) = ddid_resolution {
    //             // TODO: Main loop, use functionality from resolver type where possible

    //             // 0.1 Extract controller from doc
    //             let udid = get_controller(&ddoc).to_string();

    //             // 0.2 Extract proof from document metadata
    //             let proof = get_proof(&ddoc_meta);

    //             // 1. Verify the payload of the JWS proofvalue is equal to the doc
    //             // 1.1 Get proof payload
    //             let proof_payload = decode(&proof);

    //             // 1.2 Reconstruct payload
    //             let actual_payload = hash(&canonicalize(&ddoc).unwrap());

    //             // 1.3 Check equality
    //             if proof_payload != actual_payload {
    //                 return Err(VerifierError::InvalidPayload(ddid.to_string()));
    //             }

    //             // 2. Check the signature itself is valid
    //             // Resolve the uDID (either get hashmap entry or resolve)
    //             let udid_resolution = self
    //                 .visited
    //                 .entry(udid.clone())
    //                 .or_insert(self.resolver.resolve_as_result(&udid));

    //             if let Ok((_, Some(udoc), Some(udoc_meta))) = udid_resolution {
    //                 // 2.1 Extract keys from the uDID document
    //                 let udid_pks: Vec<JWK> = extract_keys(&udoc);

    //                 // // 2.2 Loop over the keys until signature is valid
    //                 let one_valid_key: bool = verify_jws(&proof, &udid_pks);

    //                 // // 2.3 If one_valid_key is false, return error
    //                 if !one_valid_key {
    //                     return Err(VerifierError::InvalidSignature(ddid.to_string()));
    //                 }

    //                 // 2.4 Get uDID controller (uuDID)
    //                 let uudid: &str = get_controller(&udoc);

    //                 // 2.5 If uuDID is the same as uDID, this is a root,
    //                 // check "created_at" property matches hard coded ROOT_EVENT_TIME
    //                 if uudid == udid {
    //                     let created_at = get_created_at(&udoc_meta);
    //                     if created_at == ROOT_EVENT_TIME {
    //                         return Ok(());
    //                     } else {
    //                         return Err(VerifierError::InvalidRoot(uudid.to_string()));
    //                     }
    //                 } else {
    //                     // 2.6 If not a root, set ddid as udid, and return to start of loop
    //                     ddid = udid;
    //                 }
    //             } else {
    //                 // Return an error as uDID not resolvable
    //                 return Err(VerifierError::UnresolvableDID(udid.to_string()));
    //             }
    //         } else {
    //             // Return an error as dDID not resolvable
    //             return Err(VerifierError::UnresolvableDID(ddid.to_string()));
    //         }
    //     }
    // }

    fn verified_timestamp(&self, did: &str) -> u32 {
        todo!()
    }

    // fn resolver(&self) -> Resolver<T> {
    //     todo!()
    // }
}

// TODO: add tests for each of the verifier error cases
// TODO: add test DID document and document metadata
// TODO: add mock resolver functionality to return specific test doc and metadata for tests
#[cfg(test)]
mod tests {
    use super::*;
    // use crate::data::{
    //     TEST_SIDETREE_DOCUMENT, TEST_SIDETREE_DOCUMENT_METADATA,
    //     TEST_SIDETREE_DOCUMENT_MULTIPLE_PROOF, TEST_SIDETREE_DOCUMENT_SERVICE_AND_PROOF,
    //     TEST_SIDETREE_DOCUMENT_SERVICE_NOT_PROOF, TEST_SIDETREE_DOCUMENT_WITH_CONTROLLER,
    //     TEST_TRUSTCHAIN_DOCUMENT, TEST_TRUSTCHAIN_DOCUMENT_METADATA,
    // };

    const ROOT_SIGNING_KEYS: &str = r##"
    [
        {
            "kty": "EC",
            "crv": "secp256k1",
            "x": "7ReQHHysGxbyuKEQmspQOjL7oQUqDTldTHuc9V3-yso",
            "y": "kWvmS7ZOvDUhF8syO08PBzEpEk3BZMuukkvEJOKSjqE"
        }
    ]
    "##;

    use crate::data::{
        TEST_ROOT_DOCUMENT, TEST_ROOT_DOCUMENT_METADATA, TEST_ROOT_PLUS_1_DOCUMENT,
        TEST_ROOT_PLUS_1_DOCUMENT_METADATA, TEST_ROOT_PLUS_2_DOCUMENT_METADATA,
    };
    use crate::utils::canonicalize;
    use ssi::did_resolve::HTTPDIDResolver;

    #[test]
    fn test_get_proof() -> Result<(), Box<dyn std::error::Error>> {
        let root_doc_meta: DocumentMetadata = serde_json::from_str(TEST_ROOT_DOCUMENT_METADATA)?;
        let root_plus_1_doc_meta: DocumentMetadata =
            serde_json::from_str(TEST_ROOT_PLUS_1_DOCUMENT_METADATA)?;
        let root_plus_2_doc_meta: DocumentMetadata =
            serde_json::from_str(TEST_ROOT_PLUS_2_DOCUMENT_METADATA)?;

        let root_proof = get_proof(&root_doc_meta);
        let root_plus_1_proof = get_proof(&root_plus_1_doc_meta);
        let root_plus_2_proof = get_proof(&root_plus_2_doc_meta);

        assert!(root_proof.is_err());
        assert!(root_plus_1_proof.is_ok());
        assert!(root_plus_2_proof.is_ok());
        Ok(())
    }

    #[test]
    fn test_extract_keys() -> Result<(), Box<dyn std::error::Error>> {
        let expected_root_keys: Vec<JWK> = serde_json::from_str(ROOT_SIGNING_KEYS)?;
        let root_doc: Document = serde_json::from_str(TEST_ROOT_DOCUMENT)?;
        let actual_root_keys = extract_keys(&root_doc);
        assert_eq!(actual_root_keys, expected_root_keys);
        Ok(())
    }

    #[test]
    fn test_get_controller() -> Result<(), Box<dyn std::error::Error>> {
        let doc: Document = serde_json::from_str(TEST_ROOT_PLUS_1_DOCUMENT)?;
        let expected_controller = "did:ion:test:EiCClfEdkTv_aM3UnBBhlOV89LlGhpQAbfeZLFdFxVFkEg";
        let actual_controller = get_controller(&doc)?;
        assert_eq!(expected_controller, actual_controller);
        Ok(())
    }

    // TODO: make valid DDID_DOC test doc with proof
    // const DDID_DOC: &str = r##"
    //
    // "##
    // TODO: make valid DDID_DOC test doc metadata with proof
    // const DDID_DOC_META: &str = r##"
    //
    // "##
    // TODO: make valid UDID_DOC test doc with proof
    // const UDID_DOC: &str = r##"
    //
    // "##
    // TODO: make valid UDID_DOC test doc metadata with proof
    // const UDID_DOC_META: &str = r##"
    //
    // "##
    // TODO: make valid UUDID_DOC test doc with proof (also a root DID)
    // const UUDID_DOC: &str = r##"
    //
    // "##
    // TODO: make valid UUDID_DOC test doc metadata with proof (also a root DID)
    // const UUID_DOC_META: &str = r##"
    //
    // "##
    // "##
    // TODO: make invalid UDID_DOC test doc metadata with incorrect proof *payload*
    // const UUID_DOC_META: &str = r##"
    //
    // "##
    // "##
    // TODO: make invalid UDID_DOC test doc metadata with incorrect proof *signature*
    // const UUID_DOC_META: &str = r##"
    //
    // "##
    // TODO: make invalid UUDID_DOC test doc with proof (not a real root DID)
    // const UUDID_DOC: &str = r##"
    //
    // "##
    // TODO: make invalid UUDID_DOC test doc metadata with proof (*not a real root DID*)
    // const UUID_DOC_META: &str = r##"
    //
    // "##

    // TODO: create mock resolver that returns specific test DID documents from above
    // test documents
    //
    // e.g.:
    // Succcess: dDID -> uDID -> uuDID (valid root)
    // Fail: dDID -> uDID -> uuDID_invalid_root (invalid root)
    // Fail: dDID (invalid payload) -> uDID -> uuDID
    // Fail: dDID (invalid signature) -> uDID -> uuDID
    // Fail: dDID (not resolvable) -> uDID -> uuDID

    // #[test]
    // fn verify_success() {
    //     todo!()
    // }

    // #[test]
    // fn verify_fail_for_invalid_payload() {
    //     todo!()
    // }

    // #[test]
    // fn verify_fail_for_invalid_signature() {
    //     todo!()
    // }

    // #[test]
    // fn verify_fail_for_invalid_root() {
    //     todo!()
    // }

    // #[test]
    // fn verify_fail_for_unresolvable_did() {
    //     todo!()
    // }
}