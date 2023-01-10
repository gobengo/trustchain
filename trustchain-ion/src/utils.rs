//! Utils module.
use std::convert::TryFrom;

use did_ion::sidetree::{Delta, DocumentState, PublicKey, PublicKeyEntry, ServiceEndpointEntry};
use ssi::did::{Document, ServiceEndpoint};
use ssi::jwk::JWK;

pub trait HasKeys {
    fn get_keys(&self) -> Option<Vec<JWK>>;
}

pub trait HasEndpoints {
    fn get_endpoints(&self) -> Option<Vec<ServiceEndpoint>>;
}

impl HasKeys for Document {
    fn get_keys(&self) -> Option<Vec<JWK>> {
        todo!()
    }
}

impl HasKeys for DocumentState {
    fn get_keys(&self) -> Option<Vec<JWK>> {
        let public_key_entries: Vec<PublicKeyEntry> = match &self.public_keys {
            Some(x) => x.to_vec(),
            None => {
                eprintln!("No public keys found in DocumentState.");
                return None;
            }
        };
        let public_keys: Vec<JWK> = public_key_entries
            .iter()
            .filter_map(|entry| {
                match &entry.public_key {
                    PublicKey::PublicKeyJwk(pub_key_jwk) => {
                        // Return the JWK
                        match JWK::try_from(pub_key_jwk.to_owned()) {
                            Ok(jwk) => return Some(jwk),
                            Err(e) => {
                                eprintln!("Failed to convert PublicKeyJwk to JWK: {}", e);
                                return None;
                            }
                        }
                    }
                    PublicKey::PublicKeyMultibase(_) => {
                        eprintln!("PublicKey::PublicKeyMultibase not handled.");
                        return None;
                    }
                }
            })
            .collect();
        return Some(public_keys);
    }
}

impl HasEndpoints for Document {
    fn get_endpoints(&self) -> Option<Vec<ServiceEndpoint>> {
        todo!()
    }
}

impl HasEndpoints for DocumentState {
    fn get_endpoints(&self) -> Option<Vec<ServiceEndpoint>> {
        let service_endpoint_entries: Vec<ServiceEndpointEntry> = match &self.services {
            Some(x) => x.to_vec(),
            None => {
                eprintln!("No service endpoints found in DocumentState.");
                return None;
            }
        };
        let service_endpoints: Vec<ServiceEndpoint> = service_endpoint_entries
            .iter()
            .filter_map(|entry| {
                if let ServiceEndpoint::URI(_) = entry.service_endpoint {
                    return Some(entry.service_endpoint.to_owned());
                } else {
                    eprintln!("ServiceEndpoint::Map not handled.");
                    return None;
                }
            })
            .collect();
        return Some(service_endpoints);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::TEST_CHUNK_FILE_CONTENT;
    use crate::verifier::extract_did_content;
    use serde_json::Value;

    #[test]
    fn test_get_keys_from_document_state() {
        let chunk_file_json: Value = serde_json::from_str(TEST_CHUNK_FILE_CONTENT).unwrap();
        let doc_state = extract_did_content(&chunk_file_json).unwrap();

        let result = doc_state.get_keys();
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_get_endpoints_from_document_state() {
        let chunk_file_json: Value = serde_json::from_str(TEST_CHUNK_FILE_CONTENT).unwrap();
        let doc_state = extract_did_content(&chunk_file_json).unwrap();

        let result = doc_state.get_endpoints();
        assert!(&result.is_some());
        for (i, endpoint) in result.as_ref().unwrap().iter().enumerate() {
            if let ServiceEndpoint::URI(uri) = endpoint {
                if i == 0 {
                    assert_eq!(uri, "https://identity.foundation/ion/trustchain-root");
                }
            } else {
                panic!();
            }
        }
        assert_eq!(result.unwrap().len(), 3);
    }
}