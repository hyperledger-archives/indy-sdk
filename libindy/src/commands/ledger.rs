use std::{string::ToString, sync::Arc};

use indy_api_types::{errors::prelude::*, PoolHandle, WalletHandle};
use indy_wallet::{RecordOptions, WalletService};
use rust_base58::ToBase58;
use serde_json::{self, Value};

use crate::{
    api::ledger::{CustomFree, CustomTransactionParser},
    domain::{
        anoncreds::{
            credential_definition::{
                CredentialDefinition, CredentialDefinitionId, CredentialDefinitionV1,
            },
            revocation_registry_definition::{
                RevocationRegistryDefinition, RevocationRegistryDefinitionV1, RevocationRegistryId,
            },
            revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1},
            schema::{Schema, SchemaId, SchemaV1},
        },
        crypto::{
            did::{Did, DidValue},
            key::Key,
        },
        ledger::{
            auth_rule::{AuthRules, Constraint},
            author_agreement::{AcceptanceMechanisms, GetTxnAuthorAgreementData},
            node::NodeOperationData,
            pool::Schedule,
            request::Request,
        },
    },
    services::{
        crypto::CryptoService,
        ledger::LedgerService,
        pool::{parse_response_metadata, PoolService},
    },
    utils::crypto::signature_serializer::serialize_signature,
};

enum SignatureType {
    Single,
    Multi,
}

pub(crate) struct LedgerCommandExecutor {
    pool_service: Arc<PoolService>,
    crypto_service: Arc<CryptoService>,
    wallet_service: Arc<WalletService>,
    ledger_service: Arc<LedgerService>,
}

impl LedgerCommandExecutor {
    pub(crate) fn new(
        pool_service: Arc<PoolService>,
        crypto_service: Arc<CryptoService>,
        wallet_service: Arc<WalletService>,
        ledger_service: Arc<LedgerService>,
    ) -> LedgerCommandExecutor {
        LedgerCommandExecutor {
            pool_service,
            crypto_service,
            wallet_service,
            ledger_service,
        }
    }

    pub(crate) fn register_sp_parser(
        &self,
        txn_type: String,
        parser: CustomTransactionParser,
        free: CustomFree,
    ) -> IndyResult<()> {
        debug!(
            "register_sp_parser > txn_type {:?} parser {:?} free {:?}",
            txn_type, parser, free
        );

        unimplemented!();
        // FIXME: !!!
        // PoolService::register_sp_parser(txn_type, parser, free)
        //     .map_err(IndyError::from)
    }

    pub(crate) async fn sign_and_submit_request(
        &self,
        pool_handle: PoolHandle,
        wallet_handle: WalletHandle,
        submitter_did: DidValue,
        request_json: String,
    ) -> IndyResult<String> {
        debug!(
            "sign_and_submit_request > pool_handle {:?} \
                wallet_handle {:?} submitter_did {:?} request_json {:?}",
            pool_handle, wallet_handle, submitter_did, request_json
        );

        let signed_request = self
            ._sign_request(
                wallet_handle,
                &submitter_did,
                &request_json,
                SignatureType::Single,
            )
            .await?;

        let res = self
            ._submit_request(pool_handle, signed_request.as_str())
            .await?;

        let res = Ok(res);
        debug!("sign_and_submit_request < {:?}", res);
        res
    }

    pub(crate) async fn submit_request(
        &self,
        handle: PoolHandle,
        request_json: String,
    ) -> IndyResult<String> {
        debug!(
            "submit_request > handle {:?} request_json {:?}",
            handle, request_json
        );

        let res = self._submit_request(handle, &request_json).await?;

        let res = Ok(res);
        debug!("submit_request < {:?}", res);
        res
    }

    pub(crate) async fn submit_action(
        &self,
        handle: PoolHandle,
        request_json: String,
        nodes: Option<String>,
        timeout: Option<i32>,
    ) -> IndyResult<String> {
        debug!(
            "submit_action > handle {:?} request_json {:?} nodes {:?} timeout {:?}",
            handle, request_json, nodes, timeout
        );

        self.ledger_service.validate_action(&request_json)?;

        let res = self
            .pool_service
            .send_action(handle, &request_json, nodes.as_deref(), timeout)
            .await?;

        let res = Ok(res);
        debug!("submit_action < {:?}", res);
        res
    }

    pub(crate) async fn sign_request(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: DidValue,
        request_json: String,
    ) -> IndyResult<String> {
        debug!(
            "sign_request > wallet_handle {:?} submitter_did {:?} request_json {:?}",
            wallet_handle, submitter_did, request_json
        );

        let res = self
            ._sign_request(
                wallet_handle,
                &submitter_did,
                &request_json,
                SignatureType::Single,
            )
            .await?;

        let res = Ok(res);
        debug!("sign_request < {:?}", res);
        res
    }

    pub(crate) async fn multi_sign_request(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: DidValue,
        request_json: String,
    ) -> IndyResult<String> {
        debug!(
            "multi_sign_request > wallet_handle {:?} submitter_did {:?} request_json {:?}",
            wallet_handle, submitter_did, request_json
        );

        let res = self
            ._sign_request(
                wallet_handle,
                &submitter_did,
                &request_json,
                SignatureType::Multi,
            )
            .await?;

        let res = Ok(res);
        debug!("multi_sign_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_ddo_request(
        &self,
        submitter_did: Option<DidValue>,
        target_did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "build_get_ddo_request > submitter_did {:?} target_did {:?}",
            submitter_did, target_did
        );

        let res = self
            .ledger_service
            .build_get_ddo_request(submitter_did.as_ref(), &target_did)?;

        let res = Ok(res);
        debug!("build_get_ddo_request < {:?}", res);
        res
    }

    pub(crate) async fn build_nym_request(
        &self,
        submitter_did: DidValue,
        target_did: DidValue,
        verkey: Option<String>,
        alias: Option<String>,
        role: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "build_nym_request > submitter_did {:?} \
                target_did {:?} verkey {:?} alias {:?} role {:?}",
            submitter_did, target_did, verkey, alias, role
        );

        self.crypto_service.validate_did(&submitter_did)?;
        self.crypto_service.validate_did(&target_did)?;

        if let Some(ref vk) = verkey {
            self.crypto_service.validate_key(vk).await?;
        }

        let res = self.ledger_service.build_nym_request(
            &submitter_did,
            &target_did,
            verkey.as_deref(),
            alias.as_deref(),
            role.as_deref(),
        )?;

        let res = Ok(res);
        debug!("build_nym_request < {:?}", res);
        res
    }

    pub(crate) fn build_attrib_request(
        &self,
        submitter_did: DidValue,
        target_did: DidValue,
        hash: Option<String>,
        raw: Option<serde_json::Value>,
        enc: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "build_attrib_request > submitter_did {:?} \
                target_did {:?} hash {:?} raw {:?} enc {:?}",
            submitter_did, target_did, hash, raw, enc
        );

        self.crypto_service.validate_did(&submitter_did)?;
        self.crypto_service.validate_did(&target_did)?;

        let res = self.ledger_service.build_attrib_request(
            &submitter_did,
            &target_did,
            hash.as_deref(),
            raw.as_ref(),
            enc.as_deref(),
        )?;

        let res = Ok(res);
        debug!("build_attrib_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_attrib_request(
        &self,
        submitter_did: Option<DidValue>,
        target_did: DidValue,
        raw: Option<String>,
        hash: Option<String>,
        enc: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "build_get_attrib_request > submitter_did {:?} \
                target_did {:?} raw {:?} hash {:?} enc {:?}",
            submitter_did, target_did, raw, hash, enc
        );

        self._validate_opt_did(submitter_did.as_ref())?;
        self.crypto_service.validate_did(&target_did)?;

        let res = self.ledger_service.build_get_attrib_request(
            submitter_did.as_ref(),
            &target_did,
            raw.as_deref(),
            hash.as_deref(),
            enc.as_deref(),
        )?;

        let res = Ok(res);
        debug!("build_get_attrib_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_nym_request(
        &self,
        submitter_did: Option<DidValue>,
        target_did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "build_get_nym_request > submitter_did {:?} target_did {:?}",
            submitter_did, target_did
        );

        self._validate_opt_did(submitter_did.as_ref())?;
        self.crypto_service.validate_did(&target_did)?;

        let res = self
            .ledger_service
            .build_get_nym_request(submitter_did.as_ref(), &target_did)?;

        let res = Ok(res);
        debug!("build_get_attrib_request < {:?}", res);
        res
    }

    pub(crate) fn parse_get_nym_response(&self, get_nym_response: String) -> IndyResult<String> {
        debug!(
            "parse_get_nym_response > get_nym_response {:?}",
            get_nym_response
        );

        let res = self
            .ledger_service
            .parse_get_nym_response(&get_nym_response)?;

        let res = Ok(res);
        debug!("parse_get_nym_response < {:?}", res);
        res
    }

    pub(crate) fn build_schema_request(
        &self,
        submitter_did: DidValue,
        schema: Schema,
    ) -> IndyResult<String> {
        debug!(
            "build_schema_request > submitter_did {:?} schema {:?}",
            submitter_did, schema
        );

        let schema = SchemaV1::from(schema);

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self
            .ledger_service
            .build_schema_request(&submitter_did, schema)?;

        let res = Ok(res);
        debug!("build_schema_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_schema_request(
        &self,
        submitter_did: Option<DidValue>,
        id: SchemaId,
    ) -> IndyResult<String> {
        debug!(
            "build_get_schema_request > submitter_did {:?} id {:?}",
            submitter_did, id
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self
            .ledger_service
            .build_get_schema_request(submitter_did.as_ref(), &id)?;

        let res = Ok(res);
        debug!("build_get_schema_request < {:?}", res);
        res
    }

    pub(crate) fn parse_get_schema_response(
        &self,
        get_schema_response: String,
    ) -> IndyResult<(String, String)> {
        debug!(
            "parse_get_schema_response > get_schema_response {:?}",
            get_schema_response
        );

        let res = self
            .ledger_service
            .parse_get_schema_response(&get_schema_response, None)?;

        let res = Ok(res);
        debug!("parse_get_schema_response < {:?}", res);
        res
    }

    pub(crate) fn build_cred_def_request(
        &self,
        submitter_did: DidValue,
        cred_def: CredentialDefinition,
    ) -> IndyResult<String> {
        debug!(
            "build_cred_def_request > submitter_did {:?} cred_def {:?}",
            submitter_did, cred_def
        );

        let cred_def = CredentialDefinitionV1::from(cred_def);

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self
            .ledger_service
            .build_cred_def_request(&submitter_did, cred_def)?;

        let res = Ok(res);
        debug!("build_cred_def_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_cred_def_request(
        &self,
        submitter_did: Option<DidValue>,
        id: CredentialDefinitionId,
    ) -> IndyResult<String> {
        debug!(
            "build_get_cred_def_request > submitter_did {:?} id {:?}",
            submitter_did, id
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self
            .ledger_service
            .build_get_cred_def_request(submitter_did.as_ref(), &id)?;

        let res = Ok(res);
        debug!("build_get_cred_def_request < {:?}", res);
        res
    }

    pub(crate) fn parse_get_cred_def_response(
        &self,
        get_cred_def_response: String,
    ) -> IndyResult<(String, String)> {
        debug!(
            "parse_get_cred_def_response > get_cred_def_response {:?}",
            get_cred_def_response
        );

        let res = self
            .ledger_service
            .parse_get_cred_def_response(&get_cred_def_response, None)?;

        let res = Ok(res);
        debug!("parse_get_cred_def_response < {:?}", res);
        res
    }

    pub(crate) fn build_node_request(
        &self,
        submitter_did: DidValue,
        target_did: DidValue,
        data: NodeOperationData,
    ) -> IndyResult<String> {
        debug!(
            "build_node_request > submitter_did {:?} target_did {:?} data {:?}",
            submitter_did, target_did, data
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self
            .ledger_service
            .build_node_request(&submitter_did, &target_did, data)?;

        let res = Ok(res);
        debug!("build_node_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_validator_info_request(
        &self,
        submitter_did: DidValue,
    ) -> IndyResult<String> {
        info!(
            "build_get_validator_info_request > submitter_did {:?}",
            submitter_did
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self
            .ledger_service
            .build_get_validator_info_request(&submitter_did)?;

        let res = Ok(res);
        info!("build_get_validator_info_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_txn_request(
        &self,
        submitter_did: Option<DidValue>,
        ledger_type: Option<String>,
        seq_no: i32,
    ) -> IndyResult<String> {
        debug!(
            "build_get_txn_request > submitter_did {:?} ledger_type {:?} seq_no {:?}",
            submitter_did, ledger_type, seq_no
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self.ledger_service.build_get_txn_request(
            submitter_did.as_ref(),
            ledger_type.as_deref(),
            seq_no,
        )?;

        let res = Ok(res);
        debug!("build_get_txn_request < {:?}", res);
        res
    }

    pub(crate) fn build_pool_config_request(
        &self,
        submitter_did: DidValue,
        writes: bool,
        force: bool,
    ) -> IndyResult<String> {
        debug!(
            "build_pool_config_request > submitter_did {:?} writes {:?} force {:?}",
            submitter_did, writes, force
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self
            .ledger_service
            .build_pool_config(&submitter_did, writes, force)?;

        let res = Ok(res);
        debug!("build_pool_config_request < {:?}", res);
        res
    }

    pub(crate) fn build_pool_restart_request(
        &self,
        submitter_did: DidValue,
        action: String,
        datetime: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "build_pool_restart_request > submitter_did {:?} action {:?} datetime {:?}",
            submitter_did, action, datetime
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res =
            self.ledger_service
                .build_pool_restart(&submitter_did, &action, datetime.as_deref())?;

        let res = Ok(res);
        debug!("build_pool_config_request < {:?}", res);
        res
    }

    pub(crate) fn build_pool_upgrade_request(
        &self,
        submitter_did: DidValue,
        name: String,
        version: String,
        action: String,
        sha256: String,
        timeout: Option<u32>,
        schedule: Option<Schedule>,
        justification: Option<String>,
        reinstall: bool,
        force: bool,
        package: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "build_pool_upgrade_request > submitter_did {:?} \
                name {:?} version {:?} action {:?} sha256 {:?} \
                timeout {:?} schedule {:?} justification {:?} \
                reinstall {:?} force {:?} package {:?}",
            submitter_did,
            name,
            version,
            action,
            sha256,
            timeout,
            schedule,
            justification,
            reinstall,
            force,
            package
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self.ledger_service.build_pool_upgrade(
            &submitter_did,
            &name,
            &version,
            &action,
            &sha256,
            timeout,
            schedule,
            justification.as_deref(),
            reinstall,
            force,
            package.as_deref(),
        )?;

        let res = Ok(res);
        debug!("build_pool_upgrade_request < {:?}", res);
        res
    }

    pub(crate) fn build_revoc_reg_def_request(
        &self,
        submitter_did: DidValue,
        data: RevocationRegistryDefinition,
    ) -> IndyResult<String> {
        debug!(
            "build_revoc_reg_def_request > submitter_did {:?} data {:?}",
            submitter_did, data
        );

        let data = RevocationRegistryDefinitionV1::from(data);

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self
            .ledger_service
            .build_revoc_reg_def_request(&submitter_did, data)?;

        let res = Ok(res);
        debug!("build_revoc_reg_def_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_revoc_reg_def_request(
        &self,
        submitter_did: Option<DidValue>,
        id: RevocationRegistryId,
    ) -> IndyResult<String> {
        debug!(
            "build_get_revoc_reg_def_request > submitter_did {:?} id {:?}",
            submitter_did, id
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self
            .ledger_service
            .build_get_revoc_reg_def_request(submitter_did.as_ref(), &id)?;

        let res = Ok(res);
        debug!("build_get_revoc_reg_def_request < {:?}", res);
        res
    }

    pub(crate) fn parse_revoc_reg_def_response(
        &self,
        get_revoc_reg_def_response: String,
    ) -> IndyResult<(String, String)> {
        debug!(
            "parse_revoc_reg_def_response > get_revoc_reg_def_response {:?}",
            get_revoc_reg_def_response
        );

        let res = self
            .ledger_service
            .parse_get_revoc_reg_def_response(&get_revoc_reg_def_response)?;

        let res = Ok(res);
        debug!("parse_revoc_reg_def_response < {:?}", res);
        res
    }

    pub(crate) fn build_revoc_reg_entry_request(
        &self,
        submitter_did: DidValue,
        revoc_reg_def_id: RevocationRegistryId,
        revoc_def_type: String,
        value: RevocationRegistryDelta,
    ) -> IndyResult<String> {
        debug!("build_revoc_reg_entry_request > submitter_did {:?} revoc_reg_def_id {:?} revoc_def_type {:?} value {:?}",
               submitter_did, revoc_reg_def_id, revoc_def_type, value);

        let value = RevocationRegistryDeltaV1::from(value);

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self.ledger_service.build_revoc_reg_entry_request(
            &submitter_did,
            &revoc_reg_def_id,
            &revoc_def_type,
            value,
        )?;

        let res = Ok(res);
        debug!("build_revoc_reg_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_revoc_reg_request(
        &self,
        submitter_did: Option<DidValue>,
        revoc_reg_def_id: RevocationRegistryId,
        timestamp: i64,
    ) -> IndyResult<String> {
        debug!(
            "build_get_revoc_reg_request > submitter_did {:?} revoc_reg_def_id {:?} timestamp {:?}",
            submitter_did, revoc_reg_def_id, timestamp
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self.ledger_service.build_get_revoc_reg_request(
            submitter_did.as_ref(),
            &revoc_reg_def_id,
            timestamp,
        )?;

        let res = Ok(res);
        debug!("build_get_revoc_reg_request < {:?}", res);
        res
    }

    pub(crate) fn parse_revoc_reg_response(
        &self,
        get_revoc_reg_response: String,
    ) -> IndyResult<(String, String, u64)> {
        debug!(
            "parse_revoc_reg_response > get_revoc_reg_response {:?}",
            get_revoc_reg_response
        );

        let res = self
            .ledger_service
            .parse_get_revoc_reg_response(&get_revoc_reg_response)?;

        let res = Ok(res);
        debug!("parse_revoc_reg_response < {:?}", res);
        res
    }

    pub(crate) fn build_get_revoc_reg_delta_request(
        &self,
        submitter_did: Option<DidValue>,
        revoc_reg_def_id: RevocationRegistryId,
        from: Option<i64>,
        to: i64,
    ) -> IndyResult<String> {
        debug!(
            "build_get_revoc_reg_delta_request > submitter_did {:?} \
                revoc_reg_def_id {:?} from {:?} to {:?}",
            submitter_did, revoc_reg_def_id, from, to
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self.ledger_service.build_get_revoc_reg_delta_request(
            submitter_did.as_ref(),
            &revoc_reg_def_id,
            from,
            to,
        )?;

        let res = Ok(res);
        debug!("build_get_revoc_reg_delta_request < {:?}", res);
        res
    }

    pub(crate) fn parse_revoc_reg_delta_response(
        &self,
        get_revoc_reg_delta_response: String,
    ) -> IndyResult<(String, String, u64)> {
        debug!(
            "parse_revoc_reg_delta_response > get_revoc_reg_delta_response {:?}",
            get_revoc_reg_delta_response
        );

        let res = self
            .ledger_service
            .parse_get_revoc_reg_delta_response(&get_revoc_reg_delta_response)?;

        let res = Ok(res);
        debug!("parse_revoc_reg_delta_response < {:?}", res);
        res
    }

    pub(crate) fn get_response_metadata(&self, response: String) -> IndyResult<String> {
        debug!("get_response_metadata > response {:?}", response);

        let metadata = parse_response_metadata(&response)?;

        let res = serde_json::to_string(&metadata).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize ResponseMetadata",
        )?;

        let res = Ok(res);
        debug!("get_response_metadata < {:?}", res);
        res
    }

    pub(crate) fn build_auth_rule_request(
        &self,
        submitter_did: DidValue,
        txn_type: String,
        action: String,
        field: String,
        old_value: Option<String>,
        new_value: Option<String>,
        constraint: Constraint,
    ) -> IndyResult<String> {
        debug!(
            "build_auth_rule_request > submitter_did {:?} txn_type {:?} \
                action {:?} field {:?} old_value {:?} new_value {:?} \
                constraint {:?}",
            submitter_did, txn_type, action, field, old_value, new_value, constraint
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self.ledger_service.build_auth_rule_request(
            &submitter_did,
            &txn_type,
            &action,
            &field,
            old_value.as_deref(),
            new_value.as_deref(),
            constraint,
        )?;

        let res = Ok(res);
        debug!("build_auth_rule_request < {:?}", res);
        res
    }

    pub(crate) fn build_auth_rules_request(
        &self,
        submitter_did: DidValue,
        rules: AuthRules,
    ) -> IndyResult<String> {
        debug!(
            "build_auth_rules_request > submitter_did {:?} rules {:?}",
            submitter_did, rules
        );

        self._validate_opt_did(Some(&submitter_did))?;

        let res = self
            .ledger_service
            .build_auth_rules_request(&submitter_did, rules)?;

        let res = Ok(res);
        debug!("build_auth_rules_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_auth_rule_request(
        &self,
        submitter_did: Option<DidValue>,
        txn_type: Option<String>,
        action: Option<String>,
        field: Option<String>,
        old_value: Option<String>,
        new_value: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "build_get_auth_rule_request > submitter_did {:?} \
            auth_type {:?} auth_action {:?} field {:?} \
            old_value {:?} new_value {:?}",
            submitter_did, txn_type, action, field, old_value, new_value
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self.ledger_service.build_get_auth_rule_request(
            submitter_did.as_ref(),
            txn_type.as_deref(),
            action.as_deref(),
            field.as_deref(),
            old_value.as_deref(),
            new_value.as_deref(),
        )?;

        let res = Ok(res);
        debug!("build_get_auth_rule_request < {:?}", res);
        res
    }

    pub(crate) fn build_txn_author_agreement_request(
        &self,
        submitter_did: DidValue,
        text: Option<String>,
        version: String,
        ratification_ts: Option<u64>,
        retirement_ts: Option<u64>,
    ) -> IndyResult<String> {
        debug!(
            "build_txn_author_agreement_request > submitter_did {:?} \
                text {:?} version {:?} ratification_ts {:?} \
                retirement_ts {:?}",
            submitter_did, text, version, ratification_ts, retirement_ts
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self.ledger_service.build_txn_author_agreement_request(
            &submitter_did,
            text.as_deref(),
            &version,
            ratification_ts,
            retirement_ts,
        )?;

        let res = Ok(res);
        debug!("build_txn_author_agreement_request < {:?}", res);
        res
    }

    pub(crate) fn build_disable_all_txn_author_agreements_request(
        &self,
        submitter_did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "build_disable_all_txn_author_agreements_request > submitter_did {:?}",
            submitter_did
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self
            .ledger_service
            .build_disable_all_txn_author_agreements_request(&submitter_did)?;

        let res = Ok(res);

        debug!(
            "build_disable_all_txn_author_agreements_request < {:?}",
            res
        );

        res
    }

    pub(crate) fn build_get_txn_author_agreement_request(
        &self,
        submitter_did: Option<DidValue>,
        data: Option<GetTxnAuthorAgreementData>,
    ) -> IndyResult<String> {
        debug!(
            "build_get_txn_author_agreement_request > submitter_did {:?} data {:?}",
            submitter_did, data
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self
            .ledger_service
            .build_get_txn_author_agreement_request(submitter_did.as_ref(), data.as_ref())?;

        let res = Ok(res);
        debug!("build_get_txn_author_agreement_request < {:?}", res);
        res
    }

    pub(crate) fn build_acceptance_mechanisms_request(
        &self,
        submitter_did: DidValue,
        aml: AcceptanceMechanisms,
        version: String,
        aml_context: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "build_acceptance_mechanisms_request > submitter_did {:?} \
                aml {:?} version {:?} aml_context {:?}",
            submitter_did, aml, version, aml_context
        );

        self.crypto_service.validate_did(&submitter_did)?;

        let res = self.ledger_service.build_acceptance_mechanisms_request(
            &submitter_did,
            aml,
            &version,
            aml_context.as_deref(),
        )?;

        let res = Ok(res);
        debug!("build_acceptance_mechanisms_request < {:?}", res);
        res
    }

    pub(crate) fn build_get_acceptance_mechanisms_request(
        &self,
        submitter_did: Option<DidValue>,
        timestamp: Option<u64>,
        version: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "build_get_acceptance_mechanisms_request > submitter_did {:?} \
                timestamp {:?} version {:?}",
            submitter_did, timestamp, version
        );

        self._validate_opt_did(submitter_did.as_ref())?;

        let res = self
            .ledger_service
            .build_get_acceptance_mechanisms_request(
                submitter_did.as_ref(),
                timestamp,
                version.as_deref(),
            )?;

        let res = Ok(res);
        debug!("build_get_acceptance_mechanisms_request < {:?}", res);
        res
    }

    pub(crate) fn append_txn_author_agreement_acceptance_to_request(
        &self,
        request_json: String,
        text: Option<String>,
        version: Option<String>,
        taa_digest: Option<String>,
        acc_mech_type: String,
        time: u64,
    ) -> IndyResult<String> {
        debug!(
            "append_txn_author_agreement_acceptance_to_request > request_json {:?} \
                text {:?} version {:?} taa_digest {:?} acc_mech_type {:?} time {:?}",
            request_json, text, version, taa_digest, acc_mech_type, time
        );

        let mut request: serde_json::Value = serde_json::from_str(&request_json).to_indy(
            IndyErrorKind::InvalidStructure,
            "Cannot deserialize request",
        )?;

        request["taaAcceptance"] = json!(self.ledger_service.prepare_acceptance_data(
            text.as_deref(),
            version.as_deref(),
            taa_digest.as_deref(),
            &acc_mech_type,
            time
        )?);

        let res: String = serde_json::to_string(&request).to_indy(
            IndyErrorKind::InvalidState,
            "Can't serialize request after adding author agreement acceptance data",
        )?;

        let res = Ok(res);

        debug!(
            "append_txn_author_agreement_acceptance_to_request < {:?}",
            res
        );

        res
    }

    pub(crate) fn append_request_endorser(
        &self,
        request_json: String,
        endorser_did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "append_request_endorser > request_json {:?} endorser_did {:?}",
            request_json, endorser_did
        );

        self.crypto_service.validate_did(&endorser_did)?;

        let endorser_did = endorser_did.to_short();

        let mut request: serde_json::Value = serde_json::from_str(&request_json).to_indy(
            IndyErrorKind::InvalidStructure,
            "Cannot deserialize request",
        )?;

        request["endorser"] = json!(endorser_did);

        let res: String = serde_json::to_string(&request).to_indy(
            IndyErrorKind::InvalidState,
            "Can't serialize request after adding endorser",
        )?;

        let res = Ok(res);
        debug!("append_request_endorser < {:?}", res);
        res
    }

    fn _validate_opt_did(&self, did: Option<&DidValue>) -> IndyResult<()> {
        match did {
            Some(did) => Ok(self.crypto_service.validate_did(did)?),
            None => Ok(()),
        }
    }

    async fn _sign_request(
        &self,
        wallet_handle: WalletHandle,
        submitter_did: &DidValue,
        request_json: &str,
        signature_type: SignatureType,
    ) -> IndyResult<String> {
        debug!(
            "_sign_request > wallet_handle {:?} submitter_did {:?} request_json {:?}",
            wallet_handle, submitter_did, request_json
        );

        let my_did: Did = self
            .wallet_service
            .get_indy_object(wallet_handle, &submitter_did.0, &RecordOptions::id_value())
            .await?;

        let my_key: Key = self
            .wallet_service
            .get_indy_object(wallet_handle, &my_did.verkey, &RecordOptions::id_value())
            .await?;

        let mut request: Value = serde_json::from_str(request_json)
            .to_indy(IndyErrorKind::InvalidStructure, "Message is invalid json")?;

        if !request.is_object() {
            let res = Err(err_msg(
                IndyErrorKind::InvalidStructure,
                "Message isn't json object",
            ));

            debug!("_sign_request < isn't object {:?}", res);
            return res;
        }

        let serialized_request = serialize_signature(request.clone())?;

        let signature = self
            .crypto_service
            .sign(&my_key, &serialized_request.as_bytes().to_vec())
            .await?;

        let did = my_did.did.to_short();

        match signature_type {
            SignatureType::Single => {
                request["signature"] = Value::String(signature.to_base58());
            }
            SignatureType::Multi => {
                request.as_object_mut().map(|request| {
                    if !request.contains_key("signatures") {
                        request.insert(
                            "signatures".to_string(),
                            Value::Object(serde_json::Map::new()),
                        );
                    }
                    request["signatures"]
                        .as_object_mut()
                        .unwrap()
                        .insert(did.0, Value::String(signature.to_base58()));

                    if let (Some(identifier), Some(signature)) = (
                        request
                            .get("identifier")
                            .and_then(Value::as_str)
                            .map(str::to_owned),
                        request.remove("signature"),
                    ) {
                        request["signatures"]
                            .as_object_mut()
                            .unwrap()
                            .insert(identifier, signature);
                    }
                });
            }
        }

        let res: String = serde_json::to_string(&request).to_indy(
            IndyErrorKind::InvalidState,
            "Can't serialize message after signing",
        )?;

        let res = Ok(res);
        debug!("_sign_request < {:?}", res);
        res
    }

    async fn _submit_request<'a>(
        &self,
        handle: PoolHandle,
        request_json: &str,
    ) -> IndyResult<String> {
        debug!(
            "_submit_request > handle {:?} request_json {:?}",
            handle, request_json
        );

        serde_json::from_str::<Request<serde_json::Value>>(&request_json)
            .to_indy(IndyErrorKind::InvalidStructure, "Request is invalid json")?;

        let res = self.pool_service.send_tx(handle, request_json).await?;

        let res = Ok(res);
        debug!("_submit_request < {:?}", res);
        res
    }
}
