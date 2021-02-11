use std::collections::{HashMap, HashSet};

use indy_api_types::errors::prelude::*;

use ursa::cl::{
    issuer::Issuer as UrsaIssuer, verifier::Verifier as UrsaVerifier, CredentialSchema,
    CredentialValues, MasterSecret, NonCredentialSchema, SubProofRequest,
};

use crate::domain::{
    anoncreds::{
        credential::AttributeValues,
        credential_definition::CredentialDefinition,
        credential_definition::CredentialDefinitionId,
        credential_offer::CredentialOffer,
        credential_request::CredentialRequest,
        proof_request::ProofRequest,
        proof_request::{AttributeInfo, NonRevocedInterval, PredicateInfo},
        revocation_registry_definition::RevocationRegistryDefinition,
        revocation_registry_definition::RevocationRegistryId,
        schema::Schema,
        schema::SchemaId,
    },
    crypto::did::DidValue,
};

macro_rules! _id_to_unqualified {
    ($entity:expr, $type_:ident) => {{
        if $entity.starts_with($type_::PREFIX) {
            return Ok($type_($entity.to_string()).to_unqualified().0);
        }
    }};
}

macro_rules! _object_to_unqualified {
    ($entity:expr, $type_:ident) => {{
        if let Ok(object) = ::serde_json::from_str::<$type_>(&$entity) {
            return Ok(json!(object.to_unqualified()).to_string());
        }
    }};
}

pub(crate) struct AnoncredsHelpers {}

impl AnoncredsHelpers {
    pub(crate) fn attr_common_view(attr: &str) -> String {
        attr.replace(" ", "").to_lowercase()
    }

    pub(crate) fn build_credential_schema(attrs: &HashSet<String>) -> IndyResult<CredentialSchema> {
        trace!("build_credential_schema > attrs {:?}", attrs);

        let credential_schema = {
            let mut builder = UrsaIssuer::new_credential_schema_builder()?;

            for attr in attrs {
                builder.add_attr(&Self::attr_common_view(attr))?;
            }

            builder.finalize()?
        };

        let res = Ok(credential_schema);
        trace!("build_credential_schema < {:?}", res);
        res
    }

    pub(crate) fn build_non_credential_schema() -> IndyResult<NonCredentialSchema> {
        trace!("build_non_credential_schema >");

        let schema = {
            let mut builder = UrsaIssuer::new_non_credential_schema_builder()?;
            builder.add_attr("master_secret")?;
            builder.finalize()?
        };

        let res = Ok(schema);
        trace!("build_non_credential_schema < {:?}", res);
        res
    }

    pub(crate) fn build_credential_values(
        credential_values: &HashMap<String, AttributeValues>,
        master_secret: Option<&MasterSecret>,
    ) -> IndyResult<CredentialValues> {
        trace!(
            "build_credential_values > credential_values {:?} master_secret {:?}",
            credential_values,
            secret!(master_secret),
        );

        let credential_values = {
            let mut builder = UrsaIssuer::new_credential_values_builder()?;

            for (attr, values) in credential_values {
                builder.add_dec_known(&Self::attr_common_view(attr), &values.encoded)?;
            }

            if let Some(master_secret) = master_secret {
                builder.add_value_hidden("master_secret", &master_secret.value()?)?;
            }

            builder.finalize()?
        };

        let res = Ok(credential_values);
        trace!("build_credential_values < {:?}", res);
        res
    }

    pub(crate) fn build_sub_proof_request(
        attrs_for_credential: &[AttributeInfo],
        predicates_for_credential: &[PredicateInfo],
    ) -> IndyResult<SubProofRequest> {
        trace!(
            "build_sub_proof_request > attrs_for_credential {:?} \
            predicates_for_credential {:?}",
            attrs_for_credential,
            predicates_for_credential
        );

        let sub_proof_request = {
            let mut builder = UrsaVerifier::new_sub_proof_request_builder()?;

            for ref attr in attrs_for_credential {
                if let Some(ref name) = attr.name {
                    builder.add_revealed_attr(&Self::attr_common_view(name))?
                } else if let Some(ref names) = attr.names {
                    for ref name in names {
                        builder.add_revealed_attr(&Self::attr_common_view(name))?
                    }
                } else {
                    Err(IndyError::from_msg(
                        IndyErrorKind::InvalidStructure,
                        r#"Attr for credential restriction should contain "name" or "names" param."#,
                    ))?
                };
            }

            for ref predicate in predicates_for_credential {
                builder.add_predicate(
                    &Self::attr_common_view(&predicate.name),
                    &predicate.p_type.to_string(),
                    predicate.p_value,
                )?;
            }

            builder.finalize()?
        };

        let res = Ok(sub_proof_request);
        trace!("build_sub_proof_request < {:?}", res);
        res
    }

    pub(crate) fn parse_cred_rev_id(cred_rev_id: &str) -> IndyResult<u32> {
        trace!("parse_cred_rev_id > cred_rev_id {:?}", cred_rev_id);

        let cred_rev_id = cred_rev_id.parse::<u32>().to_indy(
            IndyErrorKind::InvalidStructure,
            "Cannot parse CredentialRevocationId",
        )?;

        let res = Ok(cred_rev_id);
        trace!("parse_cred_rev_id < {:?}", res);
        res
    }

    pub(crate) fn get_non_revoc_interval(
        global_interval: &Option<NonRevocedInterval>,
        local_interval: &Option<NonRevocedInterval>,
    ) -> Option<NonRevocedInterval> {
        trace!(
            "get_non_revoc_interval > global_interval {:?} local_interval {:?}",
            global_interval,
            local_interval
        );

        let res = local_interval
            .clone()
            .or_else(|| global_interval.clone().or(None))
            .filter(|x| x.to.is_some() || x.from.is_some());

        trace!("get_non_revoc_interval < {:?}", res);
        res
    }

    pub(crate) fn to_unqualified(entity: &str) -> IndyResult<String> {
        trace!("to_unqualified > entity {:?}", entity);

        _id_to_unqualified!(entity, DidValue);
        _id_to_unqualified!(entity, SchemaId);
        _id_to_unqualified!(entity, CredentialDefinitionId);
        _id_to_unqualified!(entity, RevocationRegistryId);

        _object_to_unqualified!(entity, Schema);
        _object_to_unqualified!(entity, CredentialDefinition);
        _object_to_unqualified!(entity, RevocationRegistryDefinition);
        _object_to_unqualified!(entity, CredentialOffer);
        _object_to_unqualified!(entity, CredentialRequest);
        _object_to_unqualified!(entity, ProofRequest);

        let res = Ok(entity.to_string());
        trace!("to_unqualified < {:?}", res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _interval() -> NonRevocedInterval {
        NonRevocedInterval {
            from: None,
            to: Some(123),
        }
    }

    #[test]
    fn get_non_revoc_interval_for_global() {
        let res = AnoncredsHelpers::get_non_revoc_interval(&Some(_interval()), &None).unwrap();
        assert_eq!(_interval(), res);
    }

    #[test]
    fn get_non_revoc_interval_for_local() {
        let res = AnoncredsHelpers::get_non_revoc_interval(&None, &Some(_interval())).unwrap();
        assert_eq!(_interval(), res);
    }

    #[test]
    fn get_non_revoc_interval_for_none() {
        let res = AnoncredsHelpers::get_non_revoc_interval(&None, &None);
        assert_eq!(None, res);
    }

    #[test]
    fn get_non_revoc_interval_for_empty_interval() {
        let res = AnoncredsHelpers::get_non_revoc_interval(
            &Some(NonRevocedInterval {
                from: None,
                to: None,
            }),
            &None,
        );
        assert_eq!(None, res);
    }

    mod to_unqualified {
        use super::*;

        const DID_QUALIFIED: &str = "did:sov:NcYxiDXkpYi6ov5FcYDi1e";
        const DID_UNQUALIFIED: &str = "NcYxiDXkpYi6ov5FcYDi1e";
        const SCHEMA_ID_QUALIFIED: &str = "schema:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0";
        const SCHEMA_ID_UNQUALIFIED: &str = "NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0";
        const CRED_DEF_ID_QUALIFIED: &str = "creddef:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:3:CL:schema:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:tag";
        const CRED_DEF_ID_UNQUALIFIED: &str =
            "NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:tag";
        const REV_REG_ID_QUALIFIED: &str = "revreg:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:4:creddef:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:3:CL:schema:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:tag:CL_ACCUM:TAG_1";
        const REV_REG_ID_UNQUALIFIED: &str = "NcYxiDXkpYi6ov5FcYDi1e:4:NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:tag:CL_ACCUM:TAG_1";
        const SCHEMA_ID_WITH_SPACES_QUALIFIED: &str =
            "schema:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:2:Passport Schema:1.0";
        const SCHEMA_ID_WITH_SPACES_UNQUALIFIED: &str =
            "NcYxiDXkpYi6ov5FcYDi1e:2:Passport Schema:1.0";

        #[test]
        fn test_to_unqualified() {
            // DID
            assert_eq!(
                DID_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(DID_QUALIFIED).unwrap()
            );
            assert_eq!(
                DID_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(DID_UNQUALIFIED).unwrap()
            );

            // SchemaId
            assert_eq!(
                SCHEMA_ID_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(SCHEMA_ID_QUALIFIED).unwrap()
            );
            assert_eq!(
                SCHEMA_ID_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(SCHEMA_ID_UNQUALIFIED).unwrap()
            );

            // SchemaId
            assert_eq!(
                SCHEMA_ID_WITH_SPACES_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(SCHEMA_ID_WITH_SPACES_QUALIFIED).unwrap()
            );
            assert_eq!(
                SCHEMA_ID_WITH_SPACES_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(SCHEMA_ID_WITH_SPACES_UNQUALIFIED).unwrap()
            );

            // Credential Definition Id
            assert_eq!(
                CRED_DEF_ID_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(CRED_DEF_ID_QUALIFIED).unwrap()
            );
            assert_eq!(
                CRED_DEF_ID_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(CRED_DEF_ID_UNQUALIFIED).unwrap()
            );

            // Revocation Registry Id
            assert_eq!(
                REV_REG_ID_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(REV_REG_ID_QUALIFIED).unwrap()
            );
            assert_eq!(
                REV_REG_ID_UNQUALIFIED,
                AnoncredsHelpers::to_unqualified(REV_REG_ID_UNQUALIFIED).unwrap()
            );
        }
    }
}
