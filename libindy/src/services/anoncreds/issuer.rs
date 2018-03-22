extern crate indy_crypto;
extern crate time;

use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;
use services::anoncreds::types::*;
use services::anoncreds::helpers::*;
use std::collections::HashMap;
use self::indy_crypto::cl::*;
use self::indy_crypto::cl::issuer::Issuer as CryptoIssuer;

pub struct Issuer {}

impl Issuer {
    pub fn new() -> Issuer {
        Issuer {}
    }

    pub fn new_credential_definition(&self,
                                     issuer_did: &str,
                                     schema: &Schema,
                                     support_revocation: bool) -> Result<(CredentialDefinitionValue,
                                                                          CredentialPrivateKey,
                                                                          CredentialKeyCorrectnessProof), AnoncredsError> {
        trace!("new_credential_definition >>> issuer_did: {:?}, schema: {:?}, support_revocation: {:?}",
               issuer_did, schema, support_revocation);

        let credential_schema = build_credential_schema(&schema.attr_names)?;

        let (credential_public_key, credential_private_key, credential_key_correctness_proof) =
            CryptoIssuer::new_credential_def(&credential_schema, support_revocation)?;

        let credential_definition_value = CredentialDefinitionValue {
            primary: credential_public_key.get_primary_key()?.clone()?,
            revocation: credential_public_key.get_revocation_key()?.clone()
        };

        trace!("new_credential_definition <<< credential_definition_value: {:?}, credential_private_key: {:?}, credential_key_correctness_proof: {:?}",
               credential_definition_value, credential_private_key, credential_key_correctness_proof);

        Ok((credential_definition_value, credential_private_key, credential_key_correctness_proof))
    }

    pub fn new_revocation_registry(&self,
                                   credential_def: &CredentialDefinition,
                                   max_cred_num: u32,
                                   issuance_by_default: bool,
                                   issuer_did: &str) -> Result<(RevocationRegistryDefinitionValuePublicKeys,
                                                                RevocationKeyPrivate,
                                                                RevocationRegistry,
                                                                RevocationTailsGenerator), AnoncredsError> {
        trace!("new_revocation_registry >>> pub_key: {:?}, max_cred_num: {:?}, issuance_by_default: {:?}, issuer_did: {:?}",
               credential_def, max_cred_num, issuance_by_default, issuer_did);

        let credential_pub_key =
            CredentialPublicKey::build_from_parts(&credential_def.value.primary, credential_def.value.revocation.as_ref())?;

        let (rev_key_pub, rev_key_priv, rev_reg_entry, rev_tails_generator) =
            CryptoIssuer::new_revocation_registry_def(&credential_pub_key, max_cred_num, issuance_by_default)?;

        let rev_keys_pub = RevocationRegistryDefinitionValuePublicKeys {
            accum_key: rev_key_pub
        };

        trace!("new_revocation_registry <<< rev_keys_pub: {:?}, rev_key_priv: {:?}, rev_reg_entry: {:?}, rev_tails_generator: {:?}",
               rev_keys_pub, rev_key_priv, rev_reg_entry, rev_tails_generator);

        Ok((rev_keys_pub, rev_key_priv, rev_reg_entry, rev_tails_generator))
    }

    pub fn new_credential<RTA>(&self,
                               credential_def: &CredentialDefinition,
                               credential_priv_key: &CredentialPrivateKey,
                               master_secret_blinding_nonce: &Nonce,
                               credential_request: &CredentialRequest,
                               credential_values: &HashMap<String, AttributeValues>,
                               rev_idx: Option<u32>,
                               rev_reg_def: Option<&RevocationRegistryDefinition>,
                               rev_reg: Option<&mut RevocationRegistry>,
                               rev_key_priv: Option<&RevocationKeyPrivate>,
                               rev_tails_accessor: Option<&RTA>) -> Result<(CredentialSignature,
                                                                            SignatureCorrectnessProof,
                                                                            Option<RevocationRegistryDelta>), AnoncredsError> where RTA: RevocationTailsAccessor {
        trace!("new_credential >>> credential_def: {:?}, credential_priv_key: {:?}, master_secret_blinding_nonce: {:?}, credential_request: {:?},\
               credential_values: {:?}, rev_idx: {:?}, rev_reg_def: {:?}, rev_reg: {:?}, rev_key_priv: {:?}",
               credential_def, credential_priv_key, master_secret_blinding_nonce, credential_request, credential_values, rev_idx, rev_reg_def, rev_reg, rev_key_priv);

        let credential_values = build_credential_values(&credential_values)?;
        let credential_pub_key = CredentialPublicKey::build_from_parts(&credential_def.value.primary, credential_def.value.revocation.as_ref())?;

        let (credential_signature, signature_correctness_proof, rev_reg_delta) =
            if rev_idx.is_some() {
                let rev_idx = rev_idx.unwrap();
                let rev_reg = rev_reg
                    .ok_or(CommonError::InvalidState(format!("RevocationRegistry not found")))?;
                let rev_key_priv = rev_key_priv
                    .ok_or(CommonError::InvalidState(format!("RevocationKeyPrivate not found")))?;
                let rev_reg_def = rev_reg_def
                    .ok_or(CommonError::InvalidState(format!("RevocationRegistryDefinitionValue not found")))?;
                let rev_tails_accessor = rev_tails_accessor
                    .ok_or(CommonError::InvalidState(format!("RevocationTailsAccessor not found")))?;

                CryptoIssuer::sign_credential_with_revoc(&credential_request.prover_did,
                                                         &credential_request.blinded_ms,
                                                         &credential_request.blinded_ms_correctness_proof,
                                                         master_secret_blinding_nonce,
                                                         &credential_request.nonce,
                                                         &credential_values,
                                                         &credential_pub_key,
                                                         &credential_priv_key,
                                                         rev_idx,
                                                         rev_reg_def.value.max_cred_num,
                                                         rev_reg_def.value.issuance_type.to_bool(),
                                                         rev_reg,
                                                         rev_key_priv,
                                                         rev_tails_accessor)?



            } else {
                let (signature, correctness_proof) =
                    CryptoIssuer::sign_credential(&credential_request.prover_did,
                                                  &credential_request.blinded_ms,
                                                  &credential_request.blinded_ms_correctness_proof,
                                                  master_secret_blinding_nonce,
                                                  &credential_request.nonce,
                                                  &credential_values,
                                                  &credential_pub_key,
                                                  &credential_priv_key)?;
                (signature, correctness_proof, None)
            };

        trace!("new_credential <<< credential_signature {:?}, signature_correctness_proof {:?}, rev_reg_delta {:?}",
               credential_signature, signature_correctness_proof, rev_reg_delta);

        Ok((credential_signature, signature_correctness_proof, rev_reg_delta))
    }

    pub fn revoke<RTA>(&self,
                       rev_reg: &mut RevocationRegistry,
                       max_cred_num: u32,
                       rev_idx: u32,
                       rev_tails_accessor: &RTA) -> Result<RevocationRegistryDelta, AnoncredsError> where RTA: RevocationTailsAccessor {
        trace!("revoke >>> rev_reg: {:?}, max_cred_num: {:?}, rev_idx: {:?}", rev_reg, max_cred_num, rev_idx);

        let rev_reg_delta = CryptoIssuer::revoke_credential(rev_reg, max_cred_num, rev_idx, rev_tails_accessor)?;

        trace!("recovery <<< rev_reg_delta {:?}", rev_reg_delta);

        Ok(rev_reg_delta)
    }

    pub fn recovery<RTA>(&self,
                         rev_reg: &mut RevocationRegistry,
                         max_cred_num: u32,
                         rev_idx: u32,
                         rev_tails_accessor: &RTA) -> Result<RevocationRegistryDelta, AnoncredsError> where RTA: RevocationTailsAccessor {
        trace!("revoke >>> rev_reg: {:?}, max_cred_num: {:?}, rev_idx: {:?}", rev_reg, max_cred_num, rev_idx);

        let rev_reg_delta = CryptoIssuer::recovery_credential(rev_reg, max_cred_num, rev_idx, rev_tails_accessor)?;

        trace!("recovery <<< rev_reg_delta {:?}", rev_reg_delta);

        Ok(rev_reg_delta)
    }
}