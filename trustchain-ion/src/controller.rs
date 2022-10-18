use serde_json::Value;
use ssi::did::Document;
use ssi::jwk::{Base64urlUInt, ECParams, Params, JWK};
use thiserror::Error;

use crate::subject::IONSubject;
use trustchain_core::controller::Controller;
use trustchain_core::key_manager::{ControllerKeyManager, KeyManager, KeyManagerError};
use trustchain_core::subject::Subject;

// DID, Update Key, Recovery Key
type ControllerData = (String, String, JWK, JWK);

impl From<ControllerData> for IONController {
    fn from(data: ControllerData) -> Self {
        IONController {
            did: data.0,
            controlled_did: data.1,
            update_key: Some(data.2),
            recovery_key: Some(data.3),
            next_update_key: None,
        }
    }
}

impl KeyManager for IONController {}
impl ControllerKeyManager for IONController {}

/// Struct for common IONController.
pub struct IONController {
    did: String,
    controlled_did: String,
    update_key: Option<JWK>,
    recovery_key: Option<JWK>,
    next_update_key: Option<JWK>,
}

impl IONController {
    /// Construct a new IONController instance
    /// from existing Subject and Controller DIDs.
    pub fn new(did: &str, controlled_did: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Returns a result with propagating error

        // Construct a KeyManager for the Subject.
        let subject = IONSubject::new(did);

        // // Construct a KeyManager for the Controller.
        // let update_key: Option<JWK> = match self.read_update_key(controlled_did) {
        //     Ok(x) => Some(x),
        //     Err(_) => {
        //         return Err(Box::new(ControllerError::NoUpdateKey(
        //             controlled_did.to_string(),
        //         )))
        //     }
        // };
        // let recovery_key: Option<JWK> = match self.read_recovery_key(controlled_did) {
        //     Ok(x) => Some(x),
        //     Err(_) => {
        //         return Err(Box::new(ControllerError::NoRecoveryKey(
        //             controlled_did.to_string(),
        //         )))
        //     }
        // };

        Ok(Self {
            did: did.to_owned(),
            controlled_did: controlled_did.to_owned(),
            update_key: None,
            recovery_key: None,
            next_update_key: None,
        })
    }

    /// Assume that the document to be made into a ION DID is agreed
    /// with subject (i.e. content is correct and subject has private key
    /// for public key in doc). The function then converts the document into
    /// a create operation that can be pushed to the ION server.
    fn create_subject(doc: Document) -> IONController {
        todo!()
    }
}

impl Subject for IONController {
    fn did(&self) -> &str {
        &self.did
    }
}

impl Controller for IONController {
    fn load(&self, controlled_did: &str) {
        todo!()
    }

    fn update_key(&mut self) -> Result<&JWK, KeyManagerError> {
        if self.update_key.is_none() {
            let read_key = self.read_update_key(self.did())?;
            self.update_key = Some(read_key);
        }
        Ok(&self.update_key.as_ref().unwrap())
    }

    fn next_update_key(&mut self) -> Result<&Option<JWK>, KeyManagerError> {
        if self.next_update_key.is_none() {
            let read_key = self.read_next_update_key(self.did())?;
            self.next_update_key = Some(read_key);
        }
        Ok(&self.next_update_key)
    }

    fn generate_next_update_key(&self) {
        todo!()
    }

    fn recovery_key(&mut self) -> Result<&JWK, KeyManagerError> {
        if self.recovery_key.is_none() {
            let read_key = self.read_recovery_key(self.did())?;
            self.recovery_key = Some(read_key);
        }
        Ok(&self.recovery_key.as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trustchain_core::data::{
        TEST_NEXT_UPDATE_KEY, TEST_RECOVERY_KEY, TEST_SIGNING_KEYS, TEST_UPDATE_KEY,
    };

    #[test]
    fn test_from() -> Result<(), Box<dyn std::error::Error>> {
        let did = "did:ion:test:EiCBr7qGDecjkR2yUBhn3aNJPUR3TSEOlkpNcL0Q5Au9YP";
        let controlled_did = "did:ion:test:EiCBr7qGDecjkR2yUBhn3aNJPUR3TSEOlkpNcL0Q5AuAAA";
        let update_key: JWK = serde_json::from_str(TEST_UPDATE_KEY)?;
        let recovery_key: JWK = serde_json::from_str(TEST_RECOVERY_KEY)?;

        // Fn being tested
        let mut target = IONController::from((
            did.to_string(),
            controlled_did.to_string(),
            update_key.clone(),
            recovery_key.clone(),
        ));

        assert_eq!(target.did(), did);
        let loaded_update_key = target.update_key()?;
        assert_eq!(loaded_update_key, &update_key);

        let loaded_recovery_key = target.recovery_key()?;
        assert_eq!(loaded_recovery_key, &recovery_key);

        // Getter tested elsewhere, should be None here.
        assert_eq!(target.next_update_key, None);
        Ok(())
    }

    // #[test]
    // fn test_to_subject() {

    //     let did = "did:ion:test:EiCBr7qGDecjkR2yUBhn3aNJPUR3TSEOlkpNcL0Q5Au9YP";
    //     let controlled_did = "did:ion:test:EiA8yZGuDKbcnmPRs9ywaCsoE2FT9HMuyD9WmOiQasxBBg";
    //     let target = IONController::new(did, controlled_did);

    //     assert!(target.is_ok());

    //     let controller = target.unwrap();
    //     let subject = controller.to_subject();
    //     assert_eq!(subject.did(), did);
    // }
}