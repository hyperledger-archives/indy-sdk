use indy_api_types::errors::prelude::*;

use ursa::cl::{
    issuer::Issuer as UrsaIssuer, CredentialKeyCorrectnessProof, CredentialPrivateKey,
    CredentialPublicKey, CredentialSignature, Nonce, RevocationKeyPrivate, RevocationRegistry,
    RevocationRegistryDelta, RevocationTailsAccessor, RevocationTailsGenerator,
    SignatureCorrectnessProof,
};

use crate::{
    domain::anoncreds::{
        credential::CredentialValues,
        credential_definition::{
            CredentialDefinitionData, CredentialDefinitionV1 as CredentialDefinition,
        },
        credential_request::CredentialRequest,
        revocation_registry_definition::{
            RevocationRegistryDefinitionV1, RevocationRegistryDefinitionValuePublicKeys,
        },
        schema::AttributeNames,
    },
    domain::crypto::did::DidValue,
    services::AnoncredsHelpers,
};

pub(crate) struct IssuerService {}

impl IssuerService {
    pub(crate) fn new() -> IssuerService {
        IssuerService {}
    }

    pub(crate) fn new_credential_definition(
        attr_names: &AttributeNames,
        support_revocation: bool,
    ) -> IndyResult<(
        CredentialDefinitionData,
        CredentialPrivateKey,
        CredentialKeyCorrectnessProof,
    )> {
        trace!(
            "new_credential_definition > attr_names {:?} support_revocation {:?}",
            attr_names,
            support_revocation
        );

        let credential_schema = AnoncredsHelpers::build_credential_schema(&attr_names.0)?;
        let non_credential_schema = AnoncredsHelpers::build_non_credential_schema()?;

        let (credential_public_key, credential_private_key, credential_key_correctness_proof) =
            UrsaIssuer::new_credential_def(
                &credential_schema,
                &non_credential_schema,
                support_revocation,
            )?;

        let credential_definition_value = CredentialDefinitionData {
            primary: credential_public_key.get_primary_key()?.try_clone()?,
            revocation: credential_public_key.get_revocation_key()?.clone(),
        };

        let res = Ok((
            credential_definition_value,
            credential_private_key,
            credential_key_correctness_proof,
        ));

        trace!("new_credential_definition < {:?}", secret!(&res));
        res
    }

    pub(crate) fn new_revocation_registry(
        &self,
        cred_def: &CredentialDefinition,
        max_cred_num: u32,
        issuance_by_default: bool,
        issuer_did: &DidValue,
    ) -> IndyResult<(
        RevocationRegistryDefinitionValuePublicKeys,
        RevocationKeyPrivate,
        RevocationRegistry,
        RevocationTailsGenerator,
    )> {
        trace!(
            "new_revocation_registry > pub_key {:?} \
                max_cred_num {:?} issuance_by_default {:?} issuer_did {:?}",
            cred_def,
            max_cred_num,
            issuance_by_default,
            issuer_did
        );

        let credential_pub_key = CredentialPublicKey::build_from_parts(
            &cred_def.value.primary,
            cred_def.value.revocation.as_ref(),
        )?;

        let (rev_key_pub, rev_key_priv, rev_reg_entry, rev_tails_generator) =
            UrsaIssuer::new_revocation_registry_def(
                &credential_pub_key,
                max_cred_num,
                issuance_by_default,
            )?;

        let rev_keys_pub = RevocationRegistryDefinitionValuePublicKeys {
            accum_key: rev_key_pub,
        };

        let res = Ok((
            rev_keys_pub,
            rev_key_priv,
            rev_reg_entry,
            rev_tails_generator,
        ));

        trace!("new_revocation_registry < {:?}", secret!(&res));
        res
    }

    pub(crate) fn new_credential<RTA>(
        &self,
        cred_def: &CredentialDefinition,
        cred_priv_key: &CredentialPrivateKey,
        cred_issuance_blinding_nonce: &Nonce,
        cred_request: &CredentialRequest,
        cred_values: &CredentialValues,
        rev_idx: Option<u32>,
        rev_reg_def: Option<&RevocationRegistryDefinitionV1>,
        rev_reg: Option<&mut RevocationRegistry>,
        rev_key_priv: Option<&RevocationKeyPrivate>,
        rev_tails_accessor: Option<&RTA>,
    ) -> IndyResult<(
        CredentialSignature,
        SignatureCorrectnessProof,
        Option<RevocationRegistryDelta>,
    )>
    where
        RTA: RevocationTailsAccessor,
    {
        trace!(
            "new_credential > cred_def {:?} cred_priv_key {:?} \
                cred_issuance_blinding_nonce {:?} cred_request {:?} \
                cred_values {:?} rev_idx {:?} rev_reg_def {:?} \
                rev_reg {:?} rev_key_priv {:?}",
            cred_def,
            secret!(&cred_priv_key),
            secret!(&cred_issuance_blinding_nonce),
            secret!(&cred_request),
            secret!(&cred_values),
            secret!(&rev_idx),
            rev_reg_def,
            rev_reg,
            secret!(&rev_key_priv)
        );

        let credential_values = AnoncredsHelpers::build_credential_values(&cred_values.0, None)?;

        let credential_pub_key = CredentialPublicKey::build_from_parts(
            &cred_def.value.primary,
            cred_def.value.revocation.as_ref(),
        )?;

        let (credential_signature, signature_correctness_proof, rev_reg_delta) = match rev_idx {
            Some(rev_idx) => {
                let rev_reg = rev_reg.ok_or_else(|| {
                    err_msg(IndyErrorKind::InvalidState, "RevocationRegistry not found")
                })?;

                let rev_key_priv = rev_key_priv.ok_or_else(|| {
                    err_msg(
                        IndyErrorKind::InvalidState,
                        "RevocationKeyPrivate not found",
                    )
                })?;

                let rev_reg_def = rev_reg_def.ok_or_else(|| {
                    err_msg(
                        IndyErrorKind::InvalidState,
                        "RevocationRegistryDefinitionValue not found",
                    )
                })?;

                let rev_tails_accessor = rev_tails_accessor.ok_or_else(|| {
                    err_msg(
                        IndyErrorKind::InvalidState,
                        "RevocationTailsAccessor not found",
                    )
                })?;

                UrsaIssuer::sign_credential_with_revoc(
                    &cred_request.prover_did.0,
                    &cred_request.blinded_ms,
                    &cred_request.blinded_ms_correctness_proof,
                    cred_issuance_blinding_nonce,
                    &cred_request.nonce,
                    &credential_values,
                    &credential_pub_key,
                    &cred_priv_key,
                    rev_idx,
                    rev_reg_def.value.max_cred_num,
                    rev_reg_def.value.issuance_type.to_bool(),
                    rev_reg,
                    rev_key_priv,
                    rev_tails_accessor,
                )?
            }
            None => {
                let (signature, correctness_proof) = UrsaIssuer::sign_credential(
                    &cred_request.prover_did.0,
                    &cred_request.blinded_ms,
                    &cred_request.blinded_ms_correctness_proof,
                    cred_issuance_blinding_nonce,
                    &cred_request.nonce,
                    &credential_values,
                    &credential_pub_key,
                    &cred_priv_key,
                )?;
                (signature, correctness_proof, None)
            }
        };

        let res = Ok((
            credential_signature,
            signature_correctness_proof,
            rev_reg_delta,
        ));

        trace!("new_credential < {:?}", secret!(&res));
        res
    }

    pub(crate) fn revoke<RTA>(
        &self,
        rev_reg: &mut RevocationRegistry,
        max_cred_num: u32,
        rev_idx: u32,
        rev_tails_accessor: &RTA,
    ) -> IndyResult<RevocationRegistryDelta>
    where
        RTA: RevocationTailsAccessor,
    {
        trace!(
            "revoke > rev_reg {:?} max_cred_num {:?} rev_idx {:?}",
            rev_reg,
            max_cred_num,
            secret!(&rev_idx)
        );

        let rev_reg_delta =
            UrsaIssuer::revoke_credential(rev_reg, max_cred_num, rev_idx, rev_tails_accessor)?;

        let res = Ok(rev_reg_delta);
        trace!("recovery < {:?}", res);
        res
    }

    #[allow(dead_code)]
    pub(crate) fn recovery<RTA>(
        &self,
        rev_reg: &mut RevocationRegistry,
        max_cred_num: u32,
        rev_idx: u32,
        rev_tails_accessor: &RTA,
    ) -> IndyResult<RevocationRegistryDelta>
    where
        RTA: RevocationTailsAccessor,
    {
        trace!(
            "revoke > rev_reg {:?} max_cred_num {:?} rev_idx {:?}",
            rev_reg,
            max_cred_num,
            secret!(&rev_idx)
        );

        let rev_reg_delta =
            UrsaIssuer::recovery_credential(rev_reg, max_cred_num, rev_idx, rev_tails_accessor)?;

        let res = Ok(rev_reg_delta);
        trace!("recovery < {:?}", res);
        res
    }
}
