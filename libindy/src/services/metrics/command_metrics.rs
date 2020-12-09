use crate::commands::Command;
use crate::commands::ledger::LedgerCommand;
use crate::commands::anoncreds::AnoncredsCommand;
use crate::commands::anoncreds::issuer::IssuerCommand;
use crate::commands::anoncreds::prover::ProverCommand;
use crate::commands::anoncreds::verifier::VerifierCommand;
use crate::commands::blob_storage::BlobStorageCommand;
use crate::commands::crypto::CryptoCommand;
use crate::commands::pool::PoolCommand;
use crate::commands::did::DidCommand;
use crate::commands::wallet::WalletCommand;
use crate::commands::pairwise::PairwiseCommand;
use crate::commands::non_secrets::NonSecretsCommand;
use crate::commands::payments::PaymentsCommand;
use crate::commands::cache::CacheCommand;
use crate::commands::metrics::MetricsCommand;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandMetrics {
    pub issuer: IssuerCommandMetrics,
    pub prover: ProverCommandMetrics,
    pub verifier: VerifierCommandMetrics,
    pub anoncreds: AnoncredsCommandMetrics,
    pub blob_storage: BlobStorageMetrics,
    pub crypto: CryptoCommandMetrics,
    pub ledger: LedgerCommandMetrics,
    pub pool: PoolCommandMetrics,
    pub did: DidCommandMetrics,
    pub wallet: WalletCommandMetrics,
    pub pairwise: PairwiseCommandMetrics,
    pub non_secrets: NonSecretsCommandMetrics,
    pub payments: PaymentsCommandMetrics,
    pub cache: CacheCommandMetrics,
    pub metrics: MetricsCommandMetrics,
    pub exit: Exit,
}

impl CommandMetrics {
    pub fn new() -> Self {
        CommandMetrics {
            issuer: IssuerCommandMetrics::new(),
            prover: ProverCommandMetrics::new(),
            verifier: VerifierCommandMetrics::new(),
            anoncreds: AnoncredsCommandMetrics::new(),
            blob_storage: BlobStorageMetrics::new(),
            crypto: CryptoCommandMetrics::new(),
            ledger: LedgerCommandMetrics::new(),
            pool: PoolCommandMetrics::new(),
            did: DidCommandMetrics::new(),
            wallet: WalletCommandMetrics::new(),
            pairwise: PairwiseCommandMetrics::new(),
            non_secrets: NonSecretsCommandMetrics::new(),
            payments: PaymentsCommandMetrics::new(),
            cache: CacheCommandMetrics::new(),
            metrics: MetricsCommandMetrics::new(),
            exit: Exit::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandMetric {
    pub queued: Counters,
    pub executed: Counters
}

impl CommandMetric {
    pub fn new() -> Self {
        CommandMetric { queued: Counters::new(), executed: Counters::new() }
    }

    pub fn cmd_left_queue(&mut self, duration: u128) {
        self.queued.count += 1;
        self.queued.sum += duration;
    }

    pub fn cmd_executed(&mut self, duration: u128) {
        self.queued.count += 1;
        self.queued.sum += duration;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Counters {
    pub count: u128,
    pub sum: u128
}

impl Counters {
    pub fn new() -> Self {
        Counters{
            count: 0,
            sum: 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct IssuerCommandMetrics {
    create_schema: CommandMetric,
    create_and_store_credential_definition: CommandMetric,
    create_and_store_credential_definition_continue: CommandMetric,
    rotate_credential_definition_start: CommandMetric,
    rotate_credential_definition_start_complete: CommandMetric,
    rotate_credential_definition_apply: CommandMetric,
    create_and_store_revocation_registry: CommandMetric,
    create_credential_offer: CommandMetric,
    create_credential: CommandMetric,
    revoke_credential: CommandMetric,
    merge_revocation_registry_deltas: CommandMetric,
}

impl IssuerCommandMetrics {
    pub fn new() -> Self {
        IssuerCommandMetrics{
            create_schema: CommandMetric::new(),
            create_and_store_credential_definition: CommandMetric::new(),
            create_and_store_credential_definition_continue: CommandMetric::new(),
            rotate_credential_definition_start: CommandMetric::new(),
            rotate_credential_definition_start_complete: CommandMetric::new(),
            rotate_credential_definition_apply: CommandMetric::new(),
            create_and_store_revocation_registry: CommandMetric::new(),
            create_credential_offer: CommandMetric::new(),
            create_credential: CommandMetric::new(),
            revoke_credential: CommandMetric::new(),
            merge_revocation_registry_deltas: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProverCommandMetrics {
    create_master_secret: CommandMetric,
    create_credential_request: CommandMetric,
    set_credential_attr_tag_policy: CommandMetric,
    get_credential_attr_tag_policy: CommandMetric,
    store_credential: CommandMetric,
    get_credentials: CommandMetric,
    get_credential: CommandMetric,
    delete_credential: CommandMetric,
    search_credentials: CommandMetric,
    fetch_credentials: CommandMetric,
    close_credentials_search: CommandMetric,
    get_credentials_for_proof_req: CommandMetric,
    search_credentials_for_proof_req: CommandMetric,
    fetch_credential_for_proof_req: CommandMetric,
    close_credentials_search_for_proof_req: CommandMetric,
    create_proof: CommandMetric,
    create_revocation_state: CommandMetric,
    update_revocation_state: CommandMetric,
}

impl ProverCommandMetrics {
    pub fn new() -> Self {
        Self{
            create_master_secret: CommandMetric::new(),
            create_credential_request: CommandMetric::new(),
            set_credential_attr_tag_policy: CommandMetric::new(),
            get_credential_attr_tag_policy: CommandMetric::new(),
            store_credential: CommandMetric::new(),
            get_credentials: CommandMetric::new(),
            get_credential: CommandMetric::new(),
            delete_credential: CommandMetric::new(),
            search_credentials: CommandMetric::new(),
            fetch_credentials: CommandMetric::new(),
            close_credentials_search: CommandMetric::new(),
            get_credentials_for_proof_req: CommandMetric::new(),
            search_credentials_for_proof_req: CommandMetric::new(),
            fetch_credential_for_proof_req: CommandMetric::new(),
            close_credentials_search_for_proof_req: CommandMetric::new(),
            create_proof: CommandMetric::new(),
            create_revocation_state: CommandMetric::new(),
            update_revocation_state: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VerifierCommandMetrics {
    verify_proof: CommandMetric,
    generate_nonce: CommandMetric,
}

impl VerifierCommandMetrics {
    pub fn new() -> Self {
        Self{ verify_proof: CommandMetric::new(), generate_nonce: CommandMetric::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AnoncredsCommandMetrics {
    to_unqualified: CommandMetric,
}

impl AnoncredsCommandMetrics {
    pub fn new() -> Self {
        Self{ to_unqualified: CommandMetric::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlobStorageMetrics {
    command_open_reader: CommandMetric,
    command_open_writer: CommandMetric,
}

impl BlobStorageMetrics {
    pub fn new() -> Self {
        Self{ command_open_reader: CommandMetric::new(), command_open_writer: CommandMetric::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CryptoCommandMetrics {
    create_key: CommandMetric,
    set_key_metadata: CommandMetric,
    get_key_metadata: CommandMetric,
    crypto_sign: CommandMetric,
    crypto_verify: CommandMetric,
    authenticated_encrypt: CommandMetric,
    authenticated_decrypt: CommandMetric,
    anonymous_encrypt: CommandMetric,
    anonymous_decrypt: CommandMetric,
    pack_message: CommandMetric,
    unpack_message: CommandMetric,
}

impl CryptoCommandMetrics {
    pub fn new() -> Self {
        Self{
            create_key: CommandMetric::new(),
            set_key_metadata: CommandMetric::new(),
            get_key_metadata: CommandMetric::new(),
            crypto_sign: CommandMetric::new(),
            crypto_verify: CommandMetric::new(),
            authenticated_encrypt: CommandMetric::new(),
            authenticated_decrypt: CommandMetric::new(),
            anonymous_encrypt: CommandMetric::new(),
            anonymous_decrypt: CommandMetric::new(),
            pack_message: CommandMetric::new(),
            unpack_message: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LedgerCommandMetrics {
    sign_and_submit_request: CommandMetric,
    submit_request: CommandMetric,
    submit_ack: CommandMetric,
    submit_action: CommandMetric,
    sign_request: CommandMetric,
    multi_sign_request: CommandMetric,
    build_get_ddo_request: CommandMetric,
    build_nym_request: CommandMetric,
    build_attrib_request: CommandMetric,
    build_get_attrib_request: CommandMetric,
    build_get_nym_request: CommandMetric,
    parse_get_nym_response: CommandMetric,
    build_schema_request: CommandMetric,
    build_get_schema_request: CommandMetric,
    parse_get_schema_response: CommandMetric,
    build_cred_def_request: CommandMetric,
    build_get_cred_def_request: CommandMetric,
    parse_get_cred_def_response: CommandMetric,
    build_node_request: CommandMetric,
    build_get_validator_info_request: CommandMetric,
    build_get_txn_request: CommandMetric,
    build_pool_config_req: CommandMetric,
    build_pool_restart_request: CommandMetric,
    build_pool_upgrade_request: CommandMetric,
    build_revoc_reg_def_request: CommandMetric,
    build_get_revoc_reg_def_request: CommandMetric,
    parse_get_revoc_reg_def_response: CommandMetric,
    parse_get_revoc_reg_response: CommandMetric,
    build_revoc_reg_entry_request: CommandMetric,
    build_get_revoc_reg_request: CommandMetric,
    parseget_revoc_reg_response: CommandMetric,
    build_get_revoc_reg_delta_request: CommandMetric,
    parse_get_revoc_reg_delta_response: CommandMetric,
    register_sp_parser: CommandMetric,
    get_response_metadata: CommandMetric,
    build_auth_rule_request: CommandMetric,
    build_auth_rules_request: CommandMetric,
    build_get_auth_rule_request: CommandMetric,
    get_schema: CommandMetric,
    get_schema_continue: CommandMetric,
    get_cred_def: CommandMetric,
    get_cred_def_continue: CommandMetric,
    build_txn_author_agreement_request: CommandMetric,
    build_disable_all_txn_author_agreements_request: CommandMetric,
    build_get_txn_author_agreement_request: CommandMetric,
    build_acceptance_mechanism_requests: CommandMetric,
    build_get_acceptance_mechanisms_request: CommandMetric,
    append_txn_author_agreement_acceptance_to_request: CommandMetric,
    append_request_endorser: CommandMetric,
}

impl LedgerCommandMetrics {
    pub fn new() -> Self {
        Self{
            sign_and_submit_request: CommandMetric::new(),
            submit_request: CommandMetric::new(),
            submit_ack: CommandMetric::new(),
            submit_action: CommandMetric::new(),
            sign_request: CommandMetric::new(),
            multi_sign_request: CommandMetric::new(),
            build_get_ddo_request: CommandMetric::new(),
            build_nym_request: CommandMetric::new(),
            build_attrib_request: CommandMetric::new(),
            build_get_attrib_request: CommandMetric::new(),
            build_get_nym_request: CommandMetric::new(),
            parse_get_nym_response: CommandMetric::new(),
            build_schema_request: CommandMetric::new(),
            build_get_schema_request: CommandMetric::new(),
            parse_get_schema_response: CommandMetric::new(),
            build_cred_def_request: CommandMetric::new(),
            build_get_cred_def_request: CommandMetric::new(),
            parse_get_cred_def_response: CommandMetric::new(),
            build_node_request: CommandMetric::new(),
            build_get_validator_info_request: CommandMetric::new(),
            build_get_txn_request: CommandMetric::new(),
            build_pool_config_req: CommandMetric::new(),
            build_pool_restart_request: CommandMetric::new(),
            build_pool_upgrade_request: CommandMetric::new(),
            build_revoc_reg_def_request: CommandMetric::new(),
            build_get_revoc_reg_def_request: CommandMetric::new(),
            parse_get_revoc_reg_def_response: CommandMetric::new(),
            parse_get_revoc_reg_response: CommandMetric::new(),
            build_revoc_reg_entry_request: CommandMetric::new(),
            build_get_revoc_reg_request: CommandMetric::new(),
            parseget_revoc_reg_response: CommandMetric::new(),
            build_get_revoc_reg_delta_request: CommandMetric::new(),
            parse_get_revoc_reg_delta_response: CommandMetric::new(),
            register_sp_parser: CommandMetric::new(),
            get_response_metadata: CommandMetric::new(),
            build_auth_rule_request: CommandMetric::new(),
            build_auth_rules_request: CommandMetric::new(),
            build_get_auth_rule_request: CommandMetric::new(),
            get_schema: CommandMetric::new(),
            get_schema_continue: CommandMetric::new(),
            get_cred_def: CommandMetric::new(),
            get_cred_def_continue: CommandMetric::new(),
            build_txn_author_agreement_request: CommandMetric::new(),
            build_disable_all_txn_author_agreements_request: CommandMetric::new(),
            build_get_txn_author_agreement_request: CommandMetric::new(),
            build_acceptance_mechanism_requests: CommandMetric::new(),
            build_get_acceptance_mechanisms_request: CommandMetric::new(),
            append_txn_author_agreement_acceptance_to_request: CommandMetric::new(),
            append_request_endorser: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PoolCommandMetrics {
    create: CommandMetric,
    delete: CommandMetric,
    open: CommandMetric,
    open_ack: CommandMetric,
    list: CommandMetric,
    close: CommandMetric,
    close_ack: CommandMetric,
    refresh: CommandMetric,
    refresh_ack: CommandMetric,
    set_protocol_version: CommandMetric,
}

impl PoolCommandMetrics {
    pub fn new() -> Self {
        Self{
            create: CommandMetric::new(),
            delete: CommandMetric::new(),
            open: CommandMetric::new(),
            open_ack: CommandMetric::new(),
            list: CommandMetric::new(),
            close: CommandMetric::new(),
            close_ack: CommandMetric::new(),
            refresh: CommandMetric::new(),
            refresh_ack: CommandMetric::new(),
            set_protocol_version: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DidCommandMetrics {
    create_and_store_my_did: CommandMetric,
    replace_keys_start: CommandMetric,
    replace_keys_apply: CommandMetric,
    store_their_did: CommandMetric,
    get_my_did_with_meta: CommandMetric,
    list_my_dids_with_meta: CommandMetric,
    key_for_did: CommandMetric,
    key_for_local_did: CommandMetric,
    set_endpoint_for_did: CommandMetric,
    get_endpoint_for_did: CommandMetric,
    set_did_metadata: CommandMetric,
    get_did_metadata: CommandMetric,
    abbreviate_verkey: CommandMetric,
    get_nym_ack: CommandMetric,
    get_attrib_ack: CommandMetric,
    qualify_did: CommandMetric,
}

impl DidCommandMetrics {
    pub fn new() -> Self {
        Self{
            create_and_store_my_did: CommandMetric::new(),
            replace_keys_start: CommandMetric::new(),
            replace_keys_apply: CommandMetric::new(),
            store_their_did: CommandMetric::new(),
            get_my_did_with_meta: CommandMetric::new(),
            list_my_dids_with_meta: CommandMetric::new(),
            key_for_did: CommandMetric::new(),
            key_for_local_did: CommandMetric::new(),
            set_endpoint_for_did: CommandMetric::new(),
            get_endpoint_for_did: CommandMetric::new(),
            set_did_metadata: CommandMetric::new(),
            get_did_metadata: CommandMetric::new(),
            abbreviate_verkey: CommandMetric::new(),
            get_nym_ack: CommandMetric::new(),
            get_attrib_ack: CommandMetric::new(),
            qualify_did: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WalletCommandMetrics {
    register_wallet_type: CommandMetric,
    create: CommandMetric,
    create_continue: CommandMetric,
    open: CommandMetric,
    open_continue: CommandMetric,
    close: CommandMetric,
    delete: CommandMetric,
    delete_continue: CommandMetric,
    export: CommandMetric,
    export_continue: CommandMetric,
    import: CommandMetric,
    import_continue: CommandMetric,
    generate_key: CommandMetric,
    derive_key: CommandMetric,
}

impl WalletCommandMetrics {
    pub fn new() -> Self {
        Self{
            register_wallet_type: CommandMetric::new(),
            create: CommandMetric::new(),
            create_continue: CommandMetric::new(),
            open: CommandMetric::new(),
            open_continue: CommandMetric::new(),
            close: CommandMetric::new(),
            delete: CommandMetric::new(),
            delete_continue: CommandMetric::new(),
            export: CommandMetric::new(),
            export_continue: CommandMetric::new(),
            import: CommandMetric::new(),
            import_continue: CommandMetric::new(),
            generate_key: CommandMetric::new(),
            derive_key: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PairwiseCommandMetrics {
    pairwise_exists: CommandMetric,
    create_pairwise: CommandMetric,
    list_pairwise: CommandMetric,
    get_pairwise: CommandMetric,
    set_pairwise_metadata: CommandMetric,
}

impl PairwiseCommandMetrics {
    pub fn new() -> Self {
        Self{
            pairwise_exists: CommandMetric::new(),
            create_pairwise: CommandMetric::new(),
            list_pairwise: CommandMetric::new(),
            get_pairwise: CommandMetric::new(),
            set_pairwise_metadata: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NonSecretsCommandMetrics {
    add_record: CommandMetric,
    update_record_value: CommandMetric,
    update_record_tags: CommandMetric,
    add_record_tags: CommandMetric,
    delete_record_tags: CommandMetric,
    delete_record: CommandMetric,
    get_record: CommandMetric,
    open_search: CommandMetric,
    fetch_search_next_records: CommandMetric,
    close_search: CommandMetric,
}

impl NonSecretsCommandMetrics {
    pub fn new() -> Self {
        Self{
            add_record: CommandMetric::new(),
            update_record_value: CommandMetric::new(),
            update_record_tags: CommandMetric::new(),
            add_record_tags: CommandMetric::new(),
            delete_record_tags: CommandMetric::new(),
            delete_record: CommandMetric::new(),
            get_record: CommandMetric::new(),
            open_search: CommandMetric::new(),
            fetch_search_next_records: CommandMetric::new(),
            close_search: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PaymentsCommandMetrics {
    register_method: CommandMetric,
    create_address: CommandMetric,
    create_address_ack: CommandMetric,
    list_addresses: CommandMetric,
    add_request_fees: CommandMetric,
    add_request_fees_ack: CommandMetric,
    parse_response_with_fees: CommandMetric,
    parse_response_with_fees_ack: CommandMetric,
    build_get_payment_sources_request: CommandMetric,
    build_get_payment_sources_request_ack: CommandMetric,
    parse_get_payment_sources_response: CommandMetric,
    parse_get_payment_sources_response_ack: CommandMetric,
    build_payment_req: CommandMetric,
    build_payment_req_ack: CommandMetric,
    parse_payment_response: CommandMetric,
    parse_payment_response_ack: CommandMetric,
    append_txn_author_agreement_acceptance_to_extra: CommandMetric,
    build_mint_req: CommandMetric,
    build_mint_req_ack: CommandMetric,
    build_set_txn_fees_req: CommandMetric,
    build_set_txn_fees_req_ack: CommandMetric,
    build_get_txn_fees_req: CommandMetric,
    build_get_txn_fees_req_ack: CommandMetric,
    parse_get_txn_fees_response: CommandMetric,
    parse_get_txn_fees_response_ack: CommandMetric,
    build_verify_payment_req: CommandMetric,
    build_verify_payment_req_ack: CommandMetric,
    parse_verify_payment_response: CommandMetric,
    parse_verify_payment_response_ack: CommandMetric,
    get_request_info: CommandMetric,
    sign_with_address_req: CommandMetric,
    sign_with_address_ack: CommandMetric,
    verify_with_address_req: CommandMetric,
    verify_with_address_ack: CommandMetric,
}

impl PaymentsCommandMetrics {
    pub fn new() -> Self {
        Self{
            register_method: CommandMetric::new(),
            create_address: CommandMetric::new(),
            create_address_ack: CommandMetric::new(),
            list_addresses: CommandMetric::new(),
            add_request_fees: CommandMetric::new(),
            add_request_fees_ack: CommandMetric::new(),
            parse_response_with_fees: CommandMetric::new(),
            parse_response_with_fees_ack: CommandMetric::new(),
            build_get_payment_sources_request: CommandMetric::new(),
            build_get_payment_sources_request_ack: CommandMetric::new(),
            parse_get_payment_sources_response: CommandMetric::new(),
            parse_get_payment_sources_response_ack: CommandMetric::new(),
            build_payment_req: CommandMetric::new(),
            build_payment_req_ack: CommandMetric::new(),
            parse_payment_response: CommandMetric::new(),
            parse_payment_response_ack: CommandMetric::new(),
            append_txn_author_agreement_acceptance_to_extra: CommandMetric::new(),
            build_mint_req: CommandMetric::new(),
            build_mint_req_ack: CommandMetric::new(),
            build_set_txn_fees_req: CommandMetric::new(),
            build_set_txn_fees_req_ack: CommandMetric::new(),
            build_get_txn_fees_req: CommandMetric::new(),
            build_get_txn_fees_req_ack: CommandMetric::new(),
            parse_get_txn_fees_response: CommandMetric::new(),
            parse_get_txn_fees_response_ack: CommandMetric::new(),
            build_verify_payment_req: CommandMetric::new(),
            build_verify_payment_req_ack: CommandMetric::new(),
            parse_verify_payment_response: CommandMetric::new(),
            parse_verify_payment_response_ack: CommandMetric::new(),
            get_request_info: CommandMetric::new(),
            sign_with_address_req: CommandMetric::new(),
            sign_with_address_ack: CommandMetric::new(),
            verify_with_address_req: CommandMetric::new(),
            verify_with_address_ack: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CacheCommandMetrics {
    get_schema: CommandMetric,
    get_schema_continue: CommandMetric,
    get_cred_def: CommandMetric,
    get_cred_def_continue: CommandMetric,
    purge_schema_cache: CommandMetric,
    purge_cred_def_cache: CommandMetric,
}

impl CacheCommandMetrics {
    pub fn new() -> Self {
        Self{
            get_schema: CommandMetric::new(),
            get_schema_continue: CommandMetric::new(),
            get_cred_def: CommandMetric::new(),
            get_cred_def_continue: CommandMetric::new(),
            purge_schema_cache: CommandMetric::new(),
            purge_cred_def_cache: CommandMetric::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MetricsCommandMetrics {
    collect_metrics: CommandMetric,
}

impl MetricsCommandMetrics {
    pub fn new() -> Self {
        Self{ collect_metrics: CommandMetric::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Exit {
    exit: CommandMetric,
}

impl Exit {
    pub fn new() -> Self {
        Self{ exit: CommandMetric::new() }
    }
}

pub trait CommandAware<T> {
    fn borrow_metrics_mut(&mut self, command: T) -> &mut CommandMetric;
}

impl CommandAware<&IssuerCommand> for CommandMetrics {
    fn borrow_metrics_mut(&mut self, command: &IssuerCommand) -> &mut CommandMetric {
        match command {
            IssuerCommand::CreateSchema(_, _, _, _, _) => &mut self.issuer.create_schema,
            IssuerCommand::CreateAndStoreCredentialDefinition(_, _, _, _, _, _, _) => &mut self.issuer.create_and_store_credential_definition,
            IssuerCommand::CreateAndStoreCredentialDefinitionContinue(_, _, _, _, _, _, _, _) => &mut self.issuer.create_and_store_credential_definition_continue,
            IssuerCommand::RotateCredentialDefinitionStart(_, _, _, _) => &mut self.issuer.rotate_credential_definition_start,
            IssuerCommand::RotateCredentialDefinitionStartComplete(_, _, _, _, _, _, _) => &mut self.issuer.rotate_credential_definition_start_complete,
            IssuerCommand::RotateCredentialDefinitionApply(_, _, _) => &mut self.issuer.rotate_credential_definition_apply,
            IssuerCommand::CreateAndStoreRevocationRegistry(_, _, _, _, _, _, _, _) => &mut self.issuer.create_and_store_revocation_registry,
            IssuerCommand::CreateCredentialOffer(_, _, _) => &mut self.issuer.create_credential_offer,
            IssuerCommand::CreateCredential(_, _, _, _, _, _, _) => &mut self.issuer.create_credential,
            IssuerCommand::RevokeCredential(_, _, _, _, _) => &mut self.issuer.revoke_credential,
            IssuerCommand::MergeRevocationRegistryDeltas(_, _, _) => &mut self.issuer.merge_revocation_registry_deltas,
        }
    }
}

impl CommandAware<&ProverCommand> for CommandMetrics {
    fn borrow_metrics_mut(&mut self, command: &ProverCommand) -> &mut CommandMetric {
        match command {
            ProverCommand::CreateMasterSecret(_, _, _) => &mut self.prover.create_master_secret,
            ProverCommand::CreateCredentialRequest(_, _, _, _, _, _) => &mut self.prover.create_credential_request,
            ProverCommand::SetCredentialAttrTagPolicy(_, _, _, _, _) => &mut self.prover.set_credential_attr_tag_policy,
            ProverCommand::GetCredentialAttrTagPolicy(_, _, _) => &mut self.prover.get_credential_attr_tag_policy,
            ProverCommand::StoreCredential(_, _, _, _, _, _, _) => &mut self.prover.store_credential,
            ProverCommand::GetCredentials(_, _, _) => &mut self.prover.get_credentials,
            ProverCommand::GetCredential(_, _, _) => &mut self.prover.get_credential,
            ProverCommand::DeleteCredential(_, _, _) => &mut self.prover.delete_credential,
            ProverCommand::SearchCredentials(_, _, _) => &mut self.prover.search_credentials,
            ProverCommand::FetchCredentials(_, _, _) => &mut self.prover.fetch_credentials,
            ProverCommand::CloseCredentialsSearch(_, _) => &mut self.prover.close_credentials_search,
            ProverCommand::GetCredentialsForProofReq(_, _, _) => &mut self.prover.get_credentials_for_proof_req,
            ProverCommand::SearchCredentialsForProofReq(_, _, _, _) => &mut self.prover.search_credentials_for_proof_req,
            ProverCommand::FetchCredentialForProofReq(_, _, _, _) => &mut self.prover.fetch_credential_for_proof_req,
            ProverCommand::CloseCredentialsSearchForProofReq(_, _) => &mut self.prover.close_credentials_search_for_proof_req,
            ProverCommand::CreateProof(_, _, _, _, _, _, _, _) => &mut self.prover.create_proof,
            ProverCommand::CreateRevocationState(_, _, _, _, _, _) => &mut self.prover.create_revocation_state,
            ProverCommand::UpdateRevocationState(_, _, _, _, _, _, _) => &mut self.prover.update_revocation_state,
        }
    }
}

impl CommandAware<&VerifierCommand> for CommandMetrics {
    fn borrow_metrics_mut(&mut self, command: &VerifierCommand) -> &mut CommandMetric {
        match command {
            VerifierCommand::VerifyProof(_, _, _, _, _, _, _) => &mut self.verifier.verify_proof,
            VerifierCommand::GenerateNonce(_) => &mut self.verifier.generate_nonce
        }
    }
}

impl CommandAware<&Command> for CommandMetrics {
    fn borrow_metrics_mut(&mut self, cmd: &Command) -> &mut CommandMetric {
        match cmd {
            Command::Exit => &mut self.exit.exit,
            Command::Anoncreds(cmd) => {
                match cmd {
                    // AnoncredsCommand::Issuer(_) => &mut self.anoncreds.
                    // AnoncredsCommand::Prover(_) => { cmd.into() }
                    // AnoncredsCommand::Verifier(cmd) => { cmd.into() }
                    AnoncredsCommand::ToUnqualified(_, _) => &mut self.anoncreds.to_unqualified,
                    _ => panic!("TODO")
                }
            }
            Command::BlobStorage(cmd) => {
                match cmd {
                    BlobStorageCommand::OpenReader(_, _, _) => &mut self.blob_storage.command_open_reader,
                    BlobStorageCommand::OpenWriter(_, _, _) => &mut self.blob_storage.command_open_writer
                }
            }
            Command::Crypto(cmd) => {
                match cmd {
                    CryptoCommand::CreateKey(_, _, _) => &mut self.crypto.create_key,
                    CryptoCommand::SetKeyMetadata(_, _, _, _) => &mut self.crypto.set_key_metadata,
                    CryptoCommand::GetKeyMetadata(_, _, _) => &mut self.crypto.get_key_metadata,
                    CryptoCommand::CryptoSign(_, _, _, _) => &mut self.crypto.crypto_sign,
                    CryptoCommand::CryptoVerify(_, _, _, _) => &mut self.crypto.crypto_verify,
                    CryptoCommand::AuthenticatedEncrypt(_, _, _, _, _) => &mut self.crypto.authenticated_encrypt,
                    CryptoCommand::AuthenticatedDecrypt(_, _, _, _) => &mut self.crypto.authenticated_decrypt,
                    CryptoCommand::AnonymousEncrypt(_, _, _) => &mut self.crypto.anonymous_encrypt,
                    CryptoCommand::AnonymousDecrypt(_, _, _, _) => &mut self.crypto.anonymous_decrypt,
                    CryptoCommand::PackMessage(_, _, _, _, _) => &mut self.crypto.pack_message,
                    CryptoCommand::UnpackMessage(_, _, _) => &mut self.crypto.unpack_message,
                }
            }
            Command::Ledger(cmd) => {
                match cmd {
                    LedgerCommand::SignAndSubmitRequest(_, _, _, _, _) => &mut self.ledger.sign_and_submit_request,
                    LedgerCommand::SubmitRequest(_, _, _) => &mut self.ledger.submit_request,
                    LedgerCommand::SubmitAck(_, _) => &mut self.ledger.submit_ack,
                    LedgerCommand::SubmitAction(_, _, _, _, _) => &mut self.ledger.submit_action,
                    LedgerCommand::SignRequest(_, _, _, _) => &mut self.ledger.sign_request,
                    LedgerCommand::MultiSignRequest(_, _, _, _) => &mut self.ledger.multi_sign_request,
                    LedgerCommand::BuildGetDdoRequest(_, _, _) => &mut self.ledger.build_get_ddo_request,
                    LedgerCommand::BuildNymRequest(_, _, _, _, _, _) => &mut self.ledger.build_nym_request,
                    LedgerCommand::BuildAttribRequest(_, _, _, _, _, _) => &mut self.ledger.build_attrib_request,
                    LedgerCommand::BuildGetAttribRequest(_, _, _, _, _, _) => &mut self.ledger.build_get_attrib_request,
                    LedgerCommand::BuildGetNymRequest(_, _, _) => &mut self.ledger.build_get_nym_request,
                    LedgerCommand::ParseGetNymResponse(_, _) => &mut self.ledger.parse_get_nym_response,
                    LedgerCommand::BuildSchemaRequest(_, _, _) => &mut self.ledger.build_schema_request,
                    LedgerCommand::BuildGetSchemaRequest(_, _, _) => &mut self.ledger.build_get_schema_request,
                    LedgerCommand::ParseGetSchemaResponse(_, _) => &mut self.ledger.parse_get_schema_response,
                    LedgerCommand::BuildCredDefRequest(_, _, _) => &mut self.ledger.build_cred_def_request,
                    LedgerCommand::BuildGetCredDefRequest(_, _, _) => &mut self.ledger.build_get_cred_def_request,
                    LedgerCommand::ParseGetCredDefResponse(_, _) => &mut self.ledger.parse_get_cred_def_response,
                    LedgerCommand::BuildNodeRequest(_, _, _, _) => &mut self.ledger.build_node_request,
                    LedgerCommand::BuildGetValidatorInfoRequest(_, _) => &mut self.ledger.build_get_validator_info_request,
                    LedgerCommand::BuildGetTxnRequest(_, _, _, _) => &mut self.ledger.build_get_txn_request,
                    LedgerCommand::BuildPoolConfigRequest(_, _, _, _) => &mut self.ledger.build_pool_config_req,
                    LedgerCommand::BuildPoolRestartRequest(_, _, _, _) => &mut self.ledger.build_pool_restart_request,
                    LedgerCommand::BuildPoolUpgradeRequest(_, _, _, _, _, _, _, _, _, _, _, _) => &mut self.ledger.build_pool_upgrade_request,
                    LedgerCommand::BuildRevocRegDefRequest(_, _, _) => &mut self.ledger.build_revoc_reg_def_request,
                    LedgerCommand::BuildGetRevocRegDefRequest(_, _, _) => &mut self.ledger.build_get_revoc_reg_def_request,
                    LedgerCommand::ParseGetRevocRegDefResponse(_, _) => &mut self.ledger.parse_get_revoc_reg_def_response,
                    LedgerCommand::BuildRevocRegEntryRequest(_, _, _, _, _) => &mut self.ledger.build_revoc_reg_entry_request,
                    LedgerCommand::BuildGetRevocRegRequest(_, _, _, _) => &mut self.ledger.build_get_revoc_reg_request,
                    LedgerCommand::ParseGetRevocRegResponse(_, _) => &mut self.ledger.parse_get_revoc_reg_response,
                    LedgerCommand::BuildGetRevocRegDeltaRequest(_, _, _, _, _) => &mut self.ledger.build_get_revoc_reg_delta_request,
                    LedgerCommand::ParseGetRevocRegDeltaResponse(_, _) => &mut self.ledger.parse_get_revoc_reg_delta_response,
                    LedgerCommand::RegisterSPParser(_, _, _, _) => &mut self.ledger.register_sp_parser,
                    LedgerCommand::GetResponseMetadata(_, _) => &mut self.ledger.get_response_metadata,
                    LedgerCommand::BuildAuthRuleRequest(_, _, _, _, _, _, _, _) => &mut self.ledger.build_auth_rule_request,
                    LedgerCommand::BuildAuthRulesRequest(_, _, _) => &mut self.ledger.build_auth_rules_request,
                    LedgerCommand::BuildGetAuthRuleRequest(_, _, _, _, _, _, _) => &mut self.ledger.build_get_auth_rule_request,
                    LedgerCommand::GetSchema(_, _, _, _) => &mut self.ledger.get_schema,
                    LedgerCommand::GetSchemaContinue(_, _, _) => &mut self.ledger.get_schema_continue,
                    LedgerCommand::GetCredDef(_, _, _, _) => &mut self.ledger.get_cred_def,
                    LedgerCommand::GetCredDefContinue(_, _, _) => &mut self.ledger.get_cred_def_continue,
                    LedgerCommand::BuildTxnAuthorAgreementRequest(_, _, _, _, _, _) => &mut self.ledger.build_txn_author_agreement_request,
                    LedgerCommand::BuildDisableAllTxnAuthorAgreementsRequest(_, _) => &mut self.ledger.build_disable_all_txn_author_agreements_request,
                    LedgerCommand::BuildGetTxnAuthorAgreementRequest(_, _, _) => &mut self.ledger.build_get_txn_author_agreement_request,
                    LedgerCommand::BuildAcceptanceMechanismRequests(_, _, _, _, _) => &mut self.ledger.build_acceptance_mechanism_requests,
                    LedgerCommand::BuildGetAcceptanceMechanismsRequest(_, _, _, _) => &mut self.ledger.build_get_acceptance_mechanisms_request,
                    LedgerCommand::AppendTxnAuthorAgreementAcceptanceToRequest(_, _, _, _, _, _, _) => &mut self.ledger.append_txn_author_agreement_acceptance_to_request,
                    LedgerCommand::AppendRequestEndorser(_, _, _) => &mut self.ledger.append_request_endorser,
                }
            }
            Command::Pool(cmd) => {
                match cmd {
                    PoolCommand::Create(_, _, _) => &mut self.pool.create,
                    PoolCommand::Delete(_, _) => &mut self.pool.delete,
                    PoolCommand::Open(_, _, _) => &mut self.pool.open,
                    PoolCommand::OpenAck(_, _, _) => &mut self.pool.open_ack,
                    PoolCommand::List(_) => &mut self.pool.list,
                    PoolCommand::Close(_, _) => &mut self.pool.close,
                    PoolCommand::CloseAck(_, _) => &mut self.pool.close_ack,
                    PoolCommand::Refresh(_, _) => &mut self.pool.refresh,
                    PoolCommand::RefreshAck(_, _) => &mut self.pool.refresh_ack,
                    PoolCommand::SetProtocolVersion(_, _) => &mut self.pool.set_protocol_version,
                }
            }
            Command::Did(cmd) => {
                match cmd {
                    DidCommand::CreateAndStoreMyDid(_, _, _) => &mut self.did.create_and_store_my_did,
                    DidCommand::ReplaceKeysStart(_, _, _, _) => &mut self.did.replace_keys_start,
                    DidCommand::ReplaceKeysApply(_, _, _) => &mut self.did.replace_keys_apply,
                    DidCommand::StoreTheirDid(_, _, _) => &mut self.did.store_their_did,
                    DidCommand::GetMyDidWithMeta(_, _, _) => &mut self.did.get_my_did_with_meta,
                    DidCommand::ListMyDidsWithMeta(_, _) => &mut self.did.list_my_dids_with_meta,
                    DidCommand::KeyForDid(_, _, _, _) => &mut self.did.key_for_did,
                    DidCommand::KeyForLocalDid(_, _, _) => &mut self.did.key_for_local_did,
                    DidCommand::SetEndpointForDid(_, _, _, _) => &mut self.did.set_endpoint_for_did,
                    DidCommand::GetEndpointForDid(_, _, _, _) => &mut self.did.get_endpoint_for_did,
                    DidCommand::SetDidMetadata(_, _, _, _) => &mut self.did.set_did_metadata,
                    DidCommand::GetDidMetadata(_, _, _) => &mut self.did.get_did_metadata,
                    DidCommand::AbbreviateVerkey(_, _, _) => &mut self.did.abbreviate_verkey,
                    DidCommand::GetNymAck(_, _, _, _) => &mut self.did.get_nym_ack,
                    DidCommand::GetAttribAck(_, _, _) => &mut self.did.get_attrib_ack,
                    DidCommand::QualifyDid(_, _, _, _) => &mut self.did.qualify_did,
                }
            }
            Command::Wallet(cmd) => {
                match cmd {
                    WalletCommand::RegisterWalletType(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => &mut self.wallet.register_wallet_type,
                    WalletCommand::Create(_, _, _) => &mut self.wallet.create,
                    WalletCommand::CreateContinue(_, _, _, _, _) => &mut self.wallet.create_continue,
                    WalletCommand::Open(_, _, _) => &mut self.wallet.open,
                    WalletCommand::OpenContinue(_, _) => &mut self.wallet.open_continue,
                    WalletCommand::Close(_, _) => &mut self.wallet.close,
                    WalletCommand::Delete(_, _, _) => &mut self.wallet.delete,
                    WalletCommand::DeleteContinue(_, _, _, _, _) => &mut self.wallet.delete_continue,
                    WalletCommand::Export(_, _, _) => &mut self.wallet.export,
                    WalletCommand::ExportContinue(_, _, _, _, _) => &mut self.wallet.export_continue,
                    WalletCommand::Import(_, _, _, _) => &mut self.wallet.import,
                    WalletCommand::ImportContinue(_, _, _, _, _) => &mut self.wallet.import_continue,
                    WalletCommand::GenerateKey(_, _) => &mut self.wallet.generate_key,
                    WalletCommand::DeriveKey(_, _) => &mut self.wallet.derive_key,
                }
            }
            Command::Pairwise(cmd) => {
                match cmd {
                    PairwiseCommand::PairwiseExists(_, _, _) => &mut self.pairwise.pairwise_exists,
                    PairwiseCommand::CreatePairwise(_, _, _, _, _) => &mut self.pairwise.create_pairwise,
                    PairwiseCommand::ListPairwise(_, _) => &mut self.pairwise.list_pairwise,
                    PairwiseCommand::GetPairwise(_, _, _) => &mut self.pairwise.get_pairwise,
                    PairwiseCommand::SetPairwiseMetadata(_, _, _, _) => &mut self.pairwise.set_pairwise_metadata,
                }
            }
            Command::NonSecrets(cmd) => {
                match cmd {
                    NonSecretsCommand::AddRecord(_, _, _, _, _, _) => &mut self.non_secrets.add_record,
                    NonSecretsCommand::UpdateRecordValue(_, _, _, _, _) => &mut self.non_secrets.update_record_value,
                    NonSecretsCommand::UpdateRecordTags(_, _, _, _, _) => &mut self.non_secrets.update_record_tags,
                    NonSecretsCommand::AddRecordTags(_, _, _, _, _) => &mut self.non_secrets.add_record_tags,
                    NonSecretsCommand::DeleteRecordTags(_, _, _, _, _) => &mut self.non_secrets.delete_record_tags,
                    NonSecretsCommand::DeleteRecord(_, _, _, _) => &mut self.non_secrets.delete_record,
                    NonSecretsCommand::GetRecord(_, _, _, _, _) => &mut self.non_secrets.get_record,
                    NonSecretsCommand::OpenSearch(_, _, _, _, _) => &mut self.non_secrets.open_search,
                    NonSecretsCommand::FetchSearchNextRecords(_, _, _, _) => &mut self.non_secrets.fetch_search_next_records,
                    NonSecretsCommand::CloseSearch(_, _) => &mut self.non_secrets.close_search,
                }
            }
            Command::Payments(cmd) => {
                match cmd {
                    PaymentsCommand::RegisterMethod(_, _, _) => &mut self.payments.register_method,
                    PaymentsCommand::CreateAddress(_, _, _, _) => &mut self.payments.create_address,
                    PaymentsCommand::CreateAddressAck(_, _, _) => &mut self.payments.create_address_ack,
                    PaymentsCommand::ListAddresses(_, _) => &mut self.payments.list_addresses,
                    PaymentsCommand::AddRequestFees(_, _, _, _, _, _, _) => &mut self.payments.add_request_fees,
                    PaymentsCommand::AddRequestFeesAck(_, _) => &mut self.payments.add_request_fees_ack,
                    PaymentsCommand::ParseResponseWithFees(_, _, _) => &mut self.payments.parse_response_with_fees,
                    PaymentsCommand::ParseResponseWithFeesAck(_, _) => &mut self.payments.parse_response_with_fees_ack,
                    PaymentsCommand::BuildGetPaymentSourcesRequest(_, _, _, _, _) => &mut self.payments.build_get_payment_sources_request,
                    PaymentsCommand::BuildGetPaymentSourcesRequestAck(_, _) => &mut self.payments.build_get_payment_sources_request_ack,
                    PaymentsCommand::ParseGetPaymentSourcesResponse(_, _, _) => &mut self.payments.parse_get_payment_sources_response,
                    PaymentsCommand::ParseGetPaymentSourcesResponseAck(_, _) => &mut self.payments.parse_get_payment_sources_response_ack,
                    PaymentsCommand::BuildPaymentReq(_, _, _, _, _, _) => &mut self.payments.build_payment_req,
                    PaymentsCommand::BuildPaymentReqAck(_, _) => &mut self.payments.build_payment_req_ack,
                    PaymentsCommand::ParsePaymentResponse(_, _, _) => &mut self.payments.parse_payment_response,
                    PaymentsCommand::ParsePaymentResponseAck(_, _) => &mut self.payments.parse_payment_response_ack,
                    PaymentsCommand::AppendTxnAuthorAgreementAcceptanceToExtra(_, _, _, _, _, _, _) => &mut self.payments.append_txn_author_agreement_acceptance_to_extra,
                    PaymentsCommand::BuildMintReq(_, _, _, _, _) => &mut self.payments.build_mint_req,
                    PaymentsCommand::BuildMintReqAck(_, _) => &mut self.payments.build_mint_req_ack,
                    PaymentsCommand::BuildSetTxnFeesReq(_, _, _, _, _) => &mut self.payments.build_set_txn_fees_req,
                    PaymentsCommand::BuildSetTxnFeesReqAck(_, _) => &mut self.payments.build_set_txn_fees_req_ack,
                    PaymentsCommand::BuildGetTxnFeesReq(_, _, _, _) => &mut self.payments.build_get_txn_fees_req,
                    PaymentsCommand::BuildGetTxnFeesReqAck(_, _) => &mut self.payments.build_get_txn_fees_req_ack,
                    PaymentsCommand::ParseGetTxnFeesResponse(_, _, _) => &mut self.payments.parse_get_txn_fees_response,
                    PaymentsCommand::ParseGetTxnFeesResponseAck(_, _) => &mut self.payments.parse_get_txn_fees_response_ack,
                    PaymentsCommand::BuildVerifyPaymentReq(_, _, _, _) => &mut self.payments.build_verify_payment_req,
                    PaymentsCommand::BuildVerifyPaymentReqAck(_, _) => &mut self.payments.build_verify_payment_req_ack,
                    PaymentsCommand::ParseVerifyPaymentResponse(_, _, _) => &mut self.payments.parse_verify_payment_response,
                    PaymentsCommand::ParseVerifyPaymentResponseAck(_, _) => &mut self.payments.parse_verify_payment_response_ack,
                    PaymentsCommand::GetRequestInfo(_, _, _, _) => &mut self.payments.get_request_info,
                    PaymentsCommand::SignWithAddressReq(_, _, _, _) => &mut self.payments.sign_with_address_req,
                    PaymentsCommand::SignWithAddressAck(_, _) => &mut self.payments.sign_with_address_ack,
                    PaymentsCommand::VerifyWithAddressReq(_, _, _, _) => &mut self.payments.verify_with_address_req,
                    PaymentsCommand::VerifyWithAddressAck(_, _) => &mut self.payments.verify_with_address_ack,
                }
            }
            Command::Cache(cmd) => {
                match cmd {
                    CacheCommand::GetSchema(_, _, _, _, _, _) => &mut self.cache.get_schema,
                    CacheCommand::GetSchemaContinue(_, _, _, _) => &mut self.cache.get_schema_continue,
                    CacheCommand::GetCredDef(_, _, _, _, _, _) => &mut self.cache.get_cred_def,
                    CacheCommand::GetCredDefContinue(_, _, _, _) => &mut self.cache.get_cred_def_continue,
                    CacheCommand::PurgeSchemaCache(_, _, _) => &mut self.cache.purge_schema_cache,
                    CacheCommand::PurgeCredDefCache(_, _, _) => &mut self.cache.purge_cred_def_cache,
                }
            }
            Command::Metrics(cmd) => {
                match cmd {
                    MetricsCommand::CollectMetrics(_) => &mut self.metrics.collect_metrics,
                }
            }
        }
    }
}