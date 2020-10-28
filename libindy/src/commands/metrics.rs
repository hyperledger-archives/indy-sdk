use std::collections::HashMap;
use indy_api_types::errors::prelude::*;
use indy_wallet::WalletService;
use std::rc::Rc;
use crate::commands::Command;
use enum_map::{enum_map, Enum, EnumMap};
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

const THREADPOOL_ACTIVE_COUNT: &str = "threadpool_active_count";
const THREADPOOL_QUEUED_COUNT: &str = "threadpool_queued_count";
const THREADPOOL_MAX_COUNT: &str = "threadpool_max_count";
const THREADPOOL_PANIC_COUNT: &str = "threadpool_panic_count";
const OPENED_WALLETS_COUNT: &str = "opened_wallets_count";
const OPENED_WALLET_IDS_COUNT: &str = "opened_wallet_ids_count";
const PENDING_FOR_IMPORT_WALLETS_COUNT: &str = "pending_for_import_wallets_count";
const PENDING_FOR_OPEN_WALLETS_COUNT: &str = "pending_for_open_wallets_count";

pub enum CommandIndexes {
    // IssuerCommand
    IssuerCommandCreateSchema,
    IssuerCommandCreateAndStoreCredentialDefinition,
    IssuerCommandCreateAndStoreCredentialDefinitionContinue,
    IssuerCommandRotateCredentialDefinitionStart,
    IssuerCommandRotateCredentialDefinitionStartComplete,
    IssuerCommandRotateCredentialDefinitionApply,
    IssuerCommandCreateAndStoreRevocationRegistry,
    IssuerCommandCreateCredentialOffer,
    IssuerCommandCreateCredential,
    IssuerCommandRevokeCredential,
    IssuerCommandMergeRevocationRegistryDeltas,
    // ProverCommand
    ProverCommandCreateMasterSecret,
    ProverCommandCreateCredentialRequest,
    ProverCommandSetCredentialAttrTagPolicy,
    ProverCommandGetCredentialAttrTagPolicy,
    ProverCommandStoreCredential,
    ProverCommandGetCredentials,
    ProverCommandGetCredential,
    ProverCommandDeleteCredential,
    ProverCommandSearchCredentials,
    ProverCommandFetchCredentials,
    ProverCommandCloseCredentialsSearch,
    ProverCommandGetCredentialsForProofReq,
    ProverCommandSearchCredentialsForProofReq,
    ProverCommandFetchCredentialForProofReq,
    ProverCommandCloseCredentialsSearchForProofReq,
    ProverCommandCreateProof,
    ProverCommandCreateRevocationState,
    ProverCommandUpdateRevocationState,
    // VerifierCommand
    VerifierCommandVerifyProof,
    VerifierCommandGenerateNonce,
    // AnoncredsCommand
    AnoncredsCommandToUnqualified,
    // BlobStorage
    BlobStorageCommandOpenReader,
    BlobStorageCommandOpenWriter,
    // CryptoCommand
    CryptoCommandCreateKey,
    CryptoCommandSetKeyMetadata,
    CryptoCommandGetKeyMetadata,
    CryptoCommandCryptoSign,
    CryptoCommandCryptoVerify,
    CryptoCommandAuthenticatedEncrypt,
    CryptoCommandAuthenticatedDecrypt,
    CryptoCommandAnonymousEncrypt,
    CryptoCommandAnonymousDecrypt,
    CryptoCommandPackMessage,
    CryptoCommandUnpackMessage,
    LedgerCommandSignAndSubmitRequest,
    // LedgerCommand
    LedgerCommandSubmitRequest,
    LedgerCommandSubmitAck,
    LedgerCommandSubmitAction,
    LedgerCommandSignRequest,
    LedgerCommandMultiSignRequest,
    LedgerCommandBuildGetDdoRequest,
    LedgerCommandBuildNymRequest,
    LedgerCommandBuildAttribRequest,
    LedgerCommandBuildGetAttribRequest,
    LedgerCommandBuildGetNymRequest,
    LedgerCommandParseGetNymResponse,
    LedgerCommandBuildSchemaRequest,
    LedgerCommandBuildGetSchemaRequest,
    LedgerCommandParseGetSchemaResponse,
    LedgerCommandBuildCredDefRequest,
    LedgerCommandBuildGetCredDefRequest,
    LedgerCommandParseGetCredDefResponse,
    LedgerCommandBuildNodeRequest,
    LedgerCommandBuildGetValidatorInfoRequest,
    LedgerCommandBuildGetTxnRequest,
    LedgerCommandBuildPoolConfigRequest,
    LedgerCommandBuildPoolRestartRequest,
    LedgerCommandBuildPoolUpgradeRequest,
    LedgerCommandBuildRevocRegDefRequest,
    LedgerCommandBuildGetRevocRegDefRequest,
    LedgerCommandParseGetRevocRegDefResponse,
    LedgerCommandBuildRevocRegEntryRequest,
    LedgerCommandBuildGetRevocRegRequest,
    LedgerCommandParseGetRevocRegResponse,
    LedgerCommandBuildGetRevocRegDeltaRequest,
    LedgerCommandParseGetRevocRegDeltaResponse,
    LedgerCommandRegisterSPParser,
    LedgerCommandGetResponseMetadata,
    LedgerCommandBuildAuthRuleRequest,
    LedgerCommandBuildAuthRulesRequest,
    LedgerCommandBuildGetAuthRuleRequest,
    LedgerCommandGetSchema,
    LedgerCommandGetSchemaContinue,
    LedgerCommandGetCredDef,
    LedgerCommandGetCredDefContinue,
    LedgerCommandBuildTxnAuthorAgreementRequest,
    LedgerCommandBuildDisableAllTxnAuthorAgreementsRequest,
    LedgerCommandBuildGetTxnAuthorAgreementRequest,
    LedgerCommandBuildAcceptanceMechanismRequests,
    LedgerCommandBuildGetAcceptanceMechanismsRequest,
    LedgerCommandAppendTxnAuthorAgreementAcceptanceToRequest,
    LedgerCommandAppendRequestEndorser,
    // PoolCommand
    PoolCommandCreate,
    PoolCommandDelete,
    PoolCommandOpen,
    PoolCommandOpenAck,
    PoolCommandList,
    PoolCommandClose,
    PoolCommandCloseAck,
    PoolCommandRefresh,
    PoolCommandRefreshAck,
    PoolCommandSetProtocolVersion,
    // DidCommand
    DidCommandCreateAndStoreMyDid,
    DidCommandReplaceKeysStart,
    DidCommandReplaceKeysApply,
    DidCommandStoreTheirDid,
    DidCommandGetMyDidWithMeta,
    DidCommandListMyDidsWithMeta,
    DidCommandKeyForDid,
    DidCommandKeyForLocalDid,
    DidCommandSetEndpointForDid,
    DidCommandGetEndpointForDid,
    DidCommandSetDidMetadata,
    DidCommandGetDidMetadata,
    DidCommandAbbreviateVerkey,
    DidCommandGetNymAck,
    DidCommandGetAttribAck,
    DidCommandQualifyDid,
    // WalletCommand
    WalletCommandRegisterWalletType,
    WalletCommandCreate,
    WalletCommandCreateContinue,
    WalletCommandOpen,
    WalletCommandOpenContinue,
    WalletCommandClose,
    WalletCommandDelete,
    WalletCommandDeleteContinue,
    WalletCommandExport,
    WalletCommandExportContinue,
    WalletCommandImport,
    WalletCommandImportContinue,
    WalletCommandGenerateKey,
    WalletCommandDeriveKey,
    // PairwiseCommand
    PairwiseCommandPairwiseExists,
    PairwiseCommandCreatePairwise,
    PairwiseCommandListPairwise,
    PairwiseCommandGetPairwise,
    PairwiseCommandSetPairwiseMetadata,
    // NonSecretsCommand
    NonSecretsCommandAddRecord,
    NonSecretsCommandUpdateRecordValue,
    NonSecretsCommandUpdateRecordTags,
    NonSecretsCommandAddRecordTags,
    NonSecretsCommandDeleteRecordTags,
    NonSecretsCommandDeleteRecord,
    NonSecretsCommandGetRecord,
    NonSecretsCommandOpenSearch,
    NonSecretsCommandFetchSearchNextRecords,
    NonSecretsCommandCloseSearch,
    // PaymentsCommand
    PaymentsCommandRegisterMethod,
    PaymentsCommandCreateAddress,
    PaymentsCommandCreateAddressAck,
    PaymentsCommandListAddresses,
    PaymentsCommandAddRequestFees,
    PaymentsCommandAddRequestFeesAck,
    PaymentsCommandParseResponseWithFees,
    PaymentsCommandParseResponseWithFeesAck,
    PaymentsCommandBuildGetPaymentSourcesRequest,
    PaymentsCommandBuildGetPaymentSourcesRequestAck,
    PaymentsCommandParseGetPaymentSourcesResponse,
    PaymentsCommandParseGetPaymentSourcesResponseAck,
    PaymentsCommandBuildPaymentReq,
    PaymentsCommandBuildPaymentReqAck,
    PaymentsCommandParsePaymentResponse,
    PaymentsCommandParsePaymentResponseAck,
    PaymentsCommandAppendTxnAuthorAgreementAcceptanceToExtra,
    PaymentsCommandBuildMintReq,
    PaymentsCommandBuildMintReqAck,
    PaymentsCommandBuildSetTxnFeesReq,
    PaymentsCommandBuildSetTxnFeesReqAck,
    PaymentsCommandBuildGetTxnFeesReq,
    PaymentsCommandBuildGetTxnFeesReqAck,
    PaymentsCommandParseGetTxnFeesResponse,
    PaymentsCommandParseGetTxnFeesResponseAck,
    PaymentsCommandBuildVerifyPaymentReq,
    PaymentsCommandBuildVerifyPaymentReqAck,
    PaymentsCommandParseVerifyPaymentResponse,
    PaymentsCommandParseVerifyPaymentResponseAck,
    PaymentsCommandGetRequestInfo,
    PaymentsCommandSignWithAddressReq,
    PaymentsCommandSignWithAddressAck,
    PaymentsCommandVerifyWithAddressReq,
    PaymentsCommandVerifyWithAddressAck,
    // CacheCommand
    CacheCommandGetSchema,
    CacheCommandGetSchemaContinue,
    CacheCommandGetCredDef,
    CacheCommandGetCredDefContinue,
    CacheCommandPurgeSchemaCache,
    CacheCommandPurgeCredDefCache,
    // MetricsCommand
    MetricsCommandCollectMetrics,
    // Exit
    Exit,
}

impl From<&IssuerCommand> for CommandIndexes {
    fn from(cmd: &IssuerCommand) -> Self {
        match cmd {
            IssuerCommand::CreateSchema(_, _, _, _, _) => {
                CommandIndexes::IssuerCommandCreateSchema
            }
            IssuerCommand::CreateAndStoreCredentialDefinition(_, _, _, _, _, _, _) => {
                CommandIndexes::IssuerCommandCreateAndStoreCredentialDefinition
            }
            IssuerCommand::CreateAndStoreCredentialDefinitionContinue(_, _, _, _, _, _, _, _) => {
                CommandIndexes::IssuerCommandCreateAndStoreCredentialDefinitionContinue
            }
            IssuerCommand::RotateCredentialDefinitionStart(_, _, _, _) => {
                CommandIndexes::IssuerCommandRotateCredentialDefinitionStart
            }
            IssuerCommand::RotateCredentialDefinitionStartComplete(_, _, _, _, _, _, _) => {
                CommandIndexes::IssuerCommandRotateCredentialDefinitionStartComplete
            }
            IssuerCommand::RotateCredentialDefinitionApply(_, _, _) => {
                CommandIndexes::IssuerCommandRotateCredentialDefinitionApply
            }
            IssuerCommand::CreateAndStoreRevocationRegistry(_, _, _, _, _, _, _, _) => {
                CommandIndexes::IssuerCommandCreateAndStoreRevocationRegistry
            }
            IssuerCommand::CreateCredentialOffer(_, _, _) => {
                CommandIndexes::IssuerCommandCreateCredentialOffer
            }
            IssuerCommand::CreateCredential(_, _, _, _, _, _, _) => {
                CommandIndexes::IssuerCommandCreateCredential
            }
            IssuerCommand::RevokeCredential(_, _, _, _, _) => {
                CommandIndexes::IssuerCommandRevokeCredential
            }
            IssuerCommand::MergeRevocationRegistryDeltas(_, _, _) => {
                CommandIndexes::IssuerCommandMergeRevocationRegistryDeltas
            }
        }
    }
}

impl From<&ProverCommand> for CommandIndexes {
    fn from(cmd: &ProverCommand) -> Self {
        match cmd {
            ProverCommand::CreateMasterSecret(_, _, _) => { CommandIndexes::ProverCommandCreateMasterSecret }
            ProverCommand::CreateCredentialRequest(_, _, _, _, _, _) => { CommandIndexes::ProverCommandCreateCredentialRequest }
            ProverCommand::SetCredentialAttrTagPolicy(_, _, _, _, _) => { CommandIndexes::ProverCommandSetCredentialAttrTagPolicy }
            ProverCommand::GetCredentialAttrTagPolicy(_, _, _) => { CommandIndexes::ProverCommandGetCredentialAttrTagPolicy }
            ProverCommand::StoreCredential(_, _, _, _, _, _, _) => { CommandIndexes::ProverCommandStoreCredential }
            ProverCommand::GetCredentials(_, _, _) => { CommandIndexes::ProverCommandGetCredentials }
            ProverCommand::GetCredential(_, _, _) => { CommandIndexes::ProverCommandGetCredential }
            ProverCommand::DeleteCredential(_, _, _) => { CommandIndexes::ProverCommandDeleteCredential }
            ProverCommand::SearchCredentials(_, _, _) => { CommandIndexes::ProverCommandSearchCredentials }
            ProverCommand::FetchCredentials(_, _, _) => { CommandIndexes::ProverCommandFetchCredentials }
            ProverCommand::CloseCredentialsSearch(_, _) => { CommandIndexes::ProverCommandCloseCredentialsSearch }
            ProverCommand::GetCredentialsForProofReq(_, _, _) => { CommandIndexes::ProverCommandGetCredentialsForProofReq }
            ProverCommand::SearchCredentialsForProofReq(_, _, _, _) => { CommandIndexes::ProverCommandSearchCredentialsForProofReq }
            ProverCommand::FetchCredentialForProofReq(_, _, _, _) => { CommandIndexes::ProverCommandFetchCredentialForProofReq }
            ProverCommand::CloseCredentialsSearchForProofReq(_, _) => { CommandIndexes::ProverCommandCloseCredentialsSearchForProofReq }
            ProverCommand::CreateProof(_, _, _, _, _, _, _, _) => { CommandIndexes::ProverCommandCreateProof }
            ProverCommand::CreateRevocationState(_, _, _, _, _, _) => { CommandIndexes::ProverCommandCreateRevocationState }
            ProverCommand::UpdateRevocationState(_, _, _, _, _, _, _) => { CommandIndexes::ProverCommandUpdateRevocationState }
        }
    }
}

impl From<&VerifierCommand> for CommandIndexes {
    fn from(cmd: &VerifierCommand) -> Self {
        match cmd {
            VerifierCommand::VerifyProof(_, _, _, _, _, _, _) => { CommandIndexes::VerifierCommandVerifyProof }
            VerifierCommand::GenerateNonce(_) => { CommandIndexes::VerifierCommandGenerateNonce }
        }
    }
}

impl From<&Command> for CommandIndexes {
    fn from(cmd: &Command) -> Self {
        match cmd {
            Command::Exit => { CommandIndexes::Exit }
            Command::Anoncreds(cmd) => {
                match cmd {
                    AnoncredsCommand::Issuer(cmd) => { cmd.into() }
                    AnoncredsCommand::Prover(_) => { cmd.into() }
                    AnoncredsCommand::Verifier(_) => { cmd.into() }
                    AnoncredsCommand::ToUnqualified(_, _) => { CommandIndexes::AnoncredsCommandToUnqualified }
                }
            }
            Command::BlobStorage(cmd) => {
                match cmd {
                    BlobStorageCommand::OpenReader(_, _, _) => { CommandIndexes::BlobStorageCommandOpenReader }
                    BlobStorageCommand::OpenWriter(_, _, _) => { CommandIndexes::BlobStorageCommandOpenWriter }
                }
            }
            Command::Crypto(cmd) => {
                match cmd {
                    CryptoCommand::CreateKey(_, _, _) => { CommandIndexes::CryptoCommandCreateKey }
                    CryptoCommand::SetKeyMetadata(_, _, _, _) => { CommandIndexes::CryptoCommandSetKeyMetadata }
                    CryptoCommand::GetKeyMetadata(_, _, _) => { CommandIndexes::CryptoCommandGetKeyMetadata }
                    CryptoCommand::CryptoSign(_, _, _, _) => { CommandIndexes::CryptoCommandCryptoSign }
                    CryptoCommand::CryptoVerify(_, _, _, _) => { CommandIndexes::CryptoCommandCryptoVerify }
                    CryptoCommand::AuthenticatedEncrypt(_, _, _, _, _) => { CommandIndexes::CryptoCommandAuthenticatedEncrypt }
                    CryptoCommand::AuthenticatedDecrypt(_, _, _, _) => { CommandIndexes::CryptoCommandAuthenticatedDecrypt }
                    CryptoCommand::AnonymousEncrypt(_, _, _) => { CommandIndexes::CryptoCommandAnonymousEncrypt }
                    CryptoCommand::AnonymousDecrypt(_, _, _, _) => { CommandIndexes::CryptoCommandAnonymousDecrypt }
                    CryptoCommand::PackMessage(_, _, _, _, _) => { CommandIndexes::CryptoCommandPackMessage }
                    CryptoCommand::UnpackMessage(_, _, _) => { CommandIndexes::CryptoCommandUnpackMessage }
                }
            }
            Command::Ledger(cmd) => {
                match cmd {
                    LedgerCommand::SignAndSubmitRequest(_, _, _, _, _) => { CommandIndexes::LedgerCommandSignAndSubmitRequest }
                    LedgerCommand::SubmitRequest(_, _, _) => { CommandIndexes::LedgerCommandSubmitRequest }
                    LedgerCommand::SubmitAck(_, _) => { CommandIndexes::LedgerCommandSubmitAck }
                    LedgerCommand::SubmitAction(_, _, _, _, _) => { CommandIndexes::LedgerCommandSubmitAction }
                    LedgerCommand::SignRequest(_, _, _, _) => { CommandIndexes::LedgerCommandSignRequest }
                    LedgerCommand::MultiSignRequest(_, _, _, _) => { CommandIndexes::LedgerCommandMultiSignRequest }
                    LedgerCommand::BuildGetDdoRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildGetDdoRequest }
                    LedgerCommand::BuildNymRequest(_, _, _, _, _, _) => { CommandIndexes::LedgerCommandBuildNymRequest }
                    LedgerCommand::BuildAttribRequest(_, _, _, _, _, _) => { CommandIndexes::LedgerCommandBuildAttribRequest }
                    LedgerCommand::BuildGetAttribRequest(_, _, _, _, _, _) => { CommandIndexes::LedgerCommandBuildGetAttribRequest }
                    LedgerCommand::BuildGetNymRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildGetNymRequest }
                    LedgerCommand::ParseGetNymResponse(_, _) => { CommandIndexes::LedgerCommandParseGetNymResponse }
                    LedgerCommand::BuildSchemaRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildSchemaRequest }
                    LedgerCommand::BuildGetSchemaRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildGetSchemaRequest }
                    LedgerCommand::ParseGetSchemaResponse(_, _) => { CommandIndexes::LedgerCommandParseGetSchemaResponse }
                    LedgerCommand::BuildCredDefRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildCredDefRequest }
                    LedgerCommand::BuildGetCredDefRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildGetCredDefRequest }
                    LedgerCommand::ParseGetCredDefResponse(_, _) => { CommandIndexes::LedgerCommandParseGetCredDefResponse }
                    LedgerCommand::BuildNodeRequest(_, _, _, _) => { CommandIndexes::LedgerCommandBuildNodeRequest }
                    LedgerCommand::BuildGetValidatorInfoRequest(_, _) => { CommandIndexes::LedgerCommandBuildGetValidatorInfoRequest }
                    LedgerCommand::BuildGetTxnRequest(_, _, _, _) => { CommandIndexes::LedgerCommandBuildGetTxnRequest }
                    LedgerCommand::BuildPoolConfigRequest(_, _, _, _) => { CommandIndexes::LedgerCommandBuildPoolConfigRequest }
                    LedgerCommand::BuildPoolRestartRequest(_, _, _, _) => { CommandIndexes::LedgerCommandBuildPoolRestartRequest }
                    LedgerCommand::BuildPoolUpgradeRequest(_, _, _, _, _, _, _, _, _, _, _, _) => { CommandIndexes::LedgerCommandBuildPoolUpgradeRequest }
                    LedgerCommand::BuildRevocRegDefRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildRevocRegDefRequest }
                    LedgerCommand::BuildGetRevocRegDefRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildGetRevocRegDefRequest }
                    LedgerCommand::ParseGetRevocRegDefResponse(_, _) => { CommandIndexes::LedgerCommandParseGetRevocRegDefResponse }
                    LedgerCommand::BuildRevocRegEntryRequest(_, _, _, _, _) => { CommandIndexes::LedgerCommandBuildRevocRegEntryRequest }
                    LedgerCommand::BuildGetRevocRegRequest(_, _, _, _) => { CommandIndexes::LedgerCommandBuildGetRevocRegRequest }
                    LedgerCommand::ParseGetRevocRegResponse(_, _) => { CommandIndexes::LedgerCommandParseGetRevocRegResponse }
                    LedgerCommand::BuildGetRevocRegDeltaRequest(_, _, _, _, _) => { CommandIndexes::LedgerCommandBuildGetRevocRegDeltaRequest }
                    LedgerCommand::ParseGetRevocRegDeltaResponse(_, _) => { CommandIndexes::LedgerCommandParseGetRevocRegDeltaResponse }
                    LedgerCommand::RegisterSPParser(_, _, _, _) => { CommandIndexes::LedgerCommandRegisterSPParser }
                    LedgerCommand::GetResponseMetadata(_, _) => { CommandIndexes::LedgerCommandGetResponseMetadata }
                    LedgerCommand::BuildAuthRuleRequest(_, _, _, _, _, _, _, _) => { CommandIndexes::LedgerCommandBuildAuthRuleRequest }
                    LedgerCommand::BuildAuthRulesRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildAuthRulesRequest }
                    LedgerCommand::BuildGetAuthRuleRequest(_, _, _, _, _, _, _) => { CommandIndexes::LedgerCommandBuildGetAuthRuleRequest }
                    LedgerCommand::GetSchema(_, _, _, _) => { CommandIndexes::LedgerCommandGetSchema }
                    LedgerCommand::GetSchemaContinue(_, _, _) => { CommandIndexes::LedgerCommandGetSchemaContinue }
                    LedgerCommand::GetCredDef(_, _, _, _) => { CommandIndexes::LedgerCommandGetCredDef }
                    LedgerCommand::GetCredDefContinue(_, _, _) => { CommandIndexes::LedgerCommandGetCredDefContinue }
                    LedgerCommand::BuildTxnAuthorAgreementRequest(_, _, _, _, _, _) => { CommandIndexes::LedgerCommandBuildTxnAuthorAgreementRequest }
                    LedgerCommand::BuildDisableAllTxnAuthorAgreementsRequest(_, _) => { CommandIndexes::LedgerCommandBuildDisableAllTxnAuthorAgreementsRequest }
                    LedgerCommand::BuildGetTxnAuthorAgreementRequest(_, _, _) => { CommandIndexes::LedgerCommandBuildGetTxnAuthorAgreementRequest }
                    LedgerCommand::BuildAcceptanceMechanismRequests(_, _, _, _, _) => { CommandIndexes::LedgerCommandBuildAcceptanceMechanismRequests }
                    LedgerCommand::BuildGetAcceptanceMechanismsRequest(_, _, _, _) => { CommandIndexes::LedgerCommandBuildGetAcceptanceMechanismsRequest }
                    LedgerCommand::AppendTxnAuthorAgreementAcceptanceToRequest(_, _, _, _, _, _, _) => { CommandIndexes::LedgerCommandAppendTxnAuthorAgreementAcceptanceToRequest }
                    LedgerCommand::AppendRequestEndorser(_, _, _) => { CommandIndexes::LedgerCommandAppendRequestEndorser }
                }
            }
            Command::Pool(cmd) => {
                match cmd {
                    PoolCommand::Create(_, _, _) => { CommandIndexes::PoolCommandCreate }
                    PoolCommand::Delete(_, _) => { CommandIndexes::PoolCommandDelete }
                    PoolCommand::Open(_, _, _) => { CommandIndexes::PoolCommandOpen }
                    PoolCommand::OpenAck(_, _, _) => { CommandIndexes::PoolCommandOpenAck }
                    PoolCommand::List(_) => { CommandIndexes::PoolCommandList }
                    PoolCommand::Close(_, _) => { CommandIndexes::PoolCommandClose }
                    PoolCommand::CloseAck(_, _) => { CommandIndexes::PoolCommandCloseAck }
                    PoolCommand::Refresh(_, _) => { CommandIndexes::PoolCommandRefresh }
                    PoolCommand::RefreshAck(_, _) => { CommandIndexes::PoolCommandRefreshAck }
                    PoolCommand::SetProtocolVersion(_, _) => { CommandIndexes::PoolCommandSetProtocolVersion }
                }
            }
            Command::Did(cmd) => {
                match cmd {
                    DidCommand::CreateAndStoreMyDid(_, _, _) => { CommandIndexes::DidCommandCreateAndStoreMyDid }
                    DidCommand::ReplaceKeysStart(_, _, _, _) => { CommandIndexes::DidCommandReplaceKeysStart }
                    DidCommand::ReplaceKeysApply(_, _, _) => { CommandIndexes::DidCommandReplaceKeysApply }
                    DidCommand::StoreTheirDid(_, _, _) => { CommandIndexes::DidCommandStoreTheirDid }
                    DidCommand::GetMyDidWithMeta(_, _, _) => { CommandIndexes::DidCommandGetMyDidWithMeta }
                    DidCommand::ListMyDidsWithMeta(_, _) => { CommandIndexes::DidCommandListMyDidsWithMeta }
                    DidCommand::KeyForDid(_, _, _, _) => { CommandIndexes::DidCommandKeyForDid }
                    DidCommand::KeyForLocalDid(_, _, _) => { CommandIndexes::DidCommandKeyForLocalDid }
                    DidCommand::SetEndpointForDid(_, _, _, _) => { CommandIndexes::DidCommandSetEndpointForDid }
                    DidCommand::GetEndpointForDid(_, _, _, _) => { CommandIndexes::DidCommandGetEndpointForDid }
                    DidCommand::SetDidMetadata(_, _, _, _) => { CommandIndexes::DidCommandSetDidMetadata }
                    DidCommand::GetDidMetadata(_, _, _) => { CommandIndexes::DidCommandGetDidMetadata }
                    DidCommand::AbbreviateVerkey(_, _, _) => { CommandIndexes::DidCommandAbbreviateVerkey }
                    DidCommand::GetNymAck(_, _, _, _) => { CommandIndexes::DidCommandGetNymAck }
                    DidCommand::GetAttribAck(_, _, _) => { CommandIndexes::DidCommandGetAttribAck }
                    DidCommand::QualifyDid(_, _, _, _) => { CommandIndexes::DidCommandQualifyDid }
                }
            }
            Command::Wallet(cmd) => {
                match cmd {
                    WalletCommand::RegisterWalletType(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => { CommandIndexes::WalletCommandRegisterWalletType }
                    WalletCommand::Create(_, _, _) => { CommandIndexes::WalletCommandCreate }
                    WalletCommand::CreateContinue(_, _, _, _, _) => { CommandIndexes::WalletCommandCreateContinue }
                    WalletCommand::Open(_, _, _) => { CommandIndexes::WalletCommandOpen }
                    WalletCommand::OpenContinue(_, _) => { CommandIndexes::WalletCommandOpenContinue }
                    WalletCommand::Close(_, _) => { CommandIndexes::WalletCommandClose }
                    WalletCommand::Delete(_, _, _) => { CommandIndexes::WalletCommandDelete }
                    WalletCommand::DeleteContinue(_, _, _, _, _) => { CommandIndexes::WalletCommandDeleteContinue }
                    WalletCommand::Export(_, _, _) => { CommandIndexes::WalletCommandExport }
                    WalletCommand::ExportContinue(_, _, _, _, _) => { CommandIndexes::WalletCommandExportContinue }
                    WalletCommand::Import(_, _, _, _) => { CommandIndexes::WalletCommandImport }
                    WalletCommand::ImportContinue(_, _, _, _, _) => { CommandIndexes::WalletCommandImportContinue }
                    WalletCommand::GenerateKey(_, _) => { CommandIndexes::WalletCommandGenerateKey }
                    WalletCommand::DeriveKey(_, _) => { CommandIndexes::WalletCommandDeriveKey }
                }
            }
            Command::Pairwise(cmd) => {
                match cmd {
                    PairwiseCommand::PairwiseExists(_, _, _) => { CommandIndexes::PairwiseCommandPairwiseExists }
                    PairwiseCommand::CreatePairwise(_, _, _, _, _) => { CommandIndexes::PairwiseCommandCreatePairwise }
                    PairwiseCommand::ListPairwise(_, _) => { CommandIndexes::PairwiseCommandListPairwise }
                    PairwiseCommand::GetPairwise(_, _, _) => { CommandIndexes::PairwiseCommandGetPairwise }
                    PairwiseCommand::SetPairwiseMetadata(_, _, _, _) => { CommandIndexes::PairwiseCommandSetPairwiseMetadata }
                }
            }
            Command::NonSecrets(cmd) => {
                match cmd {
                    NonSecretsCommand::AddRecord(_, _, _, _, _, _) => { CommandIndexes::NonSecretsCommandAddRecord }
                    NonSecretsCommand::UpdateRecordValue(_, _, _, _, _) => { CommandIndexes::NonSecretsCommandUpdateRecordValue }
                    NonSecretsCommand::UpdateRecordTags(_, _, _, _, _) => { CommandIndexes::NonSecretsCommandUpdateRecordTags }
                    NonSecretsCommand::AddRecordTags(_, _, _, _, _) => { CommandIndexes::NonSecretsCommandAddRecordTags }
                    NonSecretsCommand::DeleteRecordTags(_, _, _, _, _) => { CommandIndexes::NonSecretsCommandDeleteRecordTags }
                    NonSecretsCommand::DeleteRecord(_, _, _, _) => { CommandIndexes::NonSecretsCommandDeleteRecord }
                    NonSecretsCommand::GetRecord(_, _, _, _, _) => { CommandIndexes::NonSecretsCommandGetRecord }
                    NonSecretsCommand::OpenSearch(_, _, _, _, _) => { CommandIndexes::NonSecretsCommandOpenSearch }
                    NonSecretsCommand::FetchSearchNextRecords(_, _, _, _) => { CommandIndexes::NonSecretsCommandFetchSearchNextRecords }
                    NonSecretsCommand::CloseSearch(_, _) => { CommandIndexes::NonSecretsCommandCloseSearch }
                }
            }
            Command::Payments(cmd) => {
                match cmd {
                    PaymentsCommand::RegisterMethod(_, _, _) => { CommandIndexes::PaymentsCommandRegisterMethod }
                    PaymentsCommand::CreateAddress(_, _, _, _) => { CommandIndexes::PaymentsCommandCreateAddress }
                    PaymentsCommand::CreateAddressAck(_, _, _) => { CommandIndexes::PaymentsCommandCreateAddressAck }
                    PaymentsCommand::ListAddresses(_, _) => { CommandIndexes::PaymentsCommandListAddresses }
                    PaymentsCommand::AddRequestFees(_, _, _, _, _, _, _) => { CommandIndexes::PaymentsCommandAddRequestFees }
                    PaymentsCommand::AddRequestFeesAck(_, _) => { CommandIndexes::PaymentsCommandAddRequestFeesAck }
                    PaymentsCommand::ParseResponseWithFees(_, _, _) => { CommandIndexes::PaymentsCommandParseResponseWithFees }
                    PaymentsCommand::ParseResponseWithFeesAck(_, _) => { CommandIndexes::PaymentsCommandParseResponseWithFeesAck }
                    PaymentsCommand::BuildGetPaymentSourcesRequest(_, _, _, _, _) => { CommandIndexes::PaymentsCommandBuildGetPaymentSourcesRequest }
                    PaymentsCommand::BuildGetPaymentSourcesRequestAck(_, _) => { CommandIndexes::PaymentsCommandBuildGetPaymentSourcesRequestAck }
                    PaymentsCommand::ParseGetPaymentSourcesResponse(_, _, _) => { CommandIndexes::PaymentsCommandParseGetPaymentSourcesResponse }
                    PaymentsCommand::ParseGetPaymentSourcesResponseAck(_, _) => { CommandIndexes::PaymentsCommandParseGetPaymentSourcesResponseAck }
                    PaymentsCommand::BuildPaymentReq(_, _, _, _, _, _) => { CommandIndexes::PaymentsCommandBuildPaymentReq }
                    PaymentsCommand::BuildPaymentReqAck(_, _) => { CommandIndexes::PaymentsCommandBuildPaymentReqAck }
                    PaymentsCommand::ParsePaymentResponse(_, _, _) => { CommandIndexes::PaymentsCommandParsePaymentResponse }
                    PaymentsCommand::ParsePaymentResponseAck(_, _) => { CommandIndexes::PaymentsCommandParsePaymentResponseAck }
                    PaymentsCommand::AppendTxnAuthorAgreementAcceptanceToExtra(_, _, _, _, _, _, _) => { CommandIndexes::PaymentsCommandAppendTxnAuthorAgreementAcceptanceToExtra }
                    PaymentsCommand::BuildMintReq(_, _, _, _, _) => { CommandIndexes::PaymentsCommandBuildMintReq }
                    PaymentsCommand::BuildMintReqAck(_, _) => { CommandIndexes::PaymentsCommandBuildMintReqAck }
                    PaymentsCommand::BuildSetTxnFeesReq(_, _, _, _, _) => { CommandIndexes::PaymentsCommandBuildSetTxnFeesReq }
                    PaymentsCommand::BuildSetTxnFeesReqAck(_, _) => { CommandIndexes::PaymentsCommandBuildSetTxnFeesReqAck }
                    PaymentsCommand::BuildGetTxnFeesReq(_, _, _, _) => { CommandIndexes::PaymentsCommandBuildGetTxnFeesReq }
                    PaymentsCommand::BuildGetTxnFeesReqAck(_, _) => { CommandIndexes::PaymentsCommandBuildGetTxnFeesReqAck }
                    PaymentsCommand::ParseGetTxnFeesResponse(_, _, _) => { CommandIndexes::PaymentsCommandParseGetTxnFeesResponse }
                    PaymentsCommand::ParseGetTxnFeesResponseAck(_, _) => { CommandIndexes::PaymentsCommandParseGetTxnFeesResponseAck }
                    PaymentsCommand::BuildVerifyPaymentReq(_, _, _, _) => { CommandIndexes::PaymentsCommandBuildVerifyPaymentReq }
                    PaymentsCommand::BuildVerifyPaymentReqAck(_, _) => { CommandIndexes::PaymentsCommandBuildVerifyPaymentReqAck }
                    PaymentsCommand::ParseVerifyPaymentResponse(_, _, _) => { CommandIndexes::PaymentsCommandParseVerifyPaymentResponse }
                    PaymentsCommand::ParseVerifyPaymentResponseAck(_, _) => { CommandIndexes::PaymentsCommandParseVerifyPaymentResponseAck }
                    PaymentsCommand::GetRequestInfo(_, _, _, _) => { CommandIndexes::PaymentsCommandGetRequestInfo }
                    PaymentsCommand::SignWithAddressReq(_, _, _, _) => { CommandIndexes::PaymentsCommandSignWithAddressReq }
                    PaymentsCommand::SignWithAddressAck(_, _) => { CommandIndexes::PaymentsCommandSignWithAddressAck }
                    PaymentsCommand::VerifyWithAddressReq(_, _, _, _) => { CommandIndexes::PaymentsCommandVerifyWithAddressReq }
                    PaymentsCommand::VerifyWithAddressAck(_, _) => { CommandIndexes::PaymentsCommandVerifyWithAddressAck }
                }
            }
            Command::Cache(cmd) => {
                match cmd {
                    CacheCommand::GetSchema(_, _, _, _, _, _) => { CommandIndexes::CacheCommandGetSchema }
                    CacheCommand::GetSchemaContinue(_, _, _, _) => { CommandIndexes::CacheCommandGetSchemaContinue }
                    CacheCommand::GetCredDef(_, _, _, _, _, _) => { CommandIndexes::CacheCommandGetCredDef }
                    CacheCommand::GetCredDefContinue(_, _, _, _) => { CommandIndexes::CacheCommandGetCredDefContinue }
                    CacheCommand::PurgeSchemaCache(_, _, _) => { CommandIndexes::CacheCommandPurgeSchemaCache }
                    CacheCommand::PurgeCredDefCache(_, _, _) => { CommandIndexes::CacheCommandPurgeCredDefCache }
                }
            }
            Command::Metrics(cmd) => {
                match cmd { MetricsCommand::CollectMetrics(_) => { CommandIndexes::MetricsCommandCollectMetrics } }
            }
        }
    }
}


pub enum MetricsCommand {
    CollectMetrics(Box<dyn Fn(IndyResult<String>) + Send>)
}

const COMMANDS_COUNT: usize = MetricsStorage::commands_count();

pub struct MetricsStorage {
    queue_count: [u64; COMMANDS_COUNT],
    queue_duration: [u64; COMMANDS_COUNT],

    execute_count: [u64; COMMANDS_COUNT],
    execute_duration: [u64; COMMANDS_COUNT],
}

impl MetricsStorage {
    pub fn new() -> MetricsStorage {
        MetricsStorage {
            queue_count: [0; COMMANDS_COUNT],
            queue_duration: [0; COMMANDS_COUNT],

            execute_count: [0; COMMANDS_COUNT],
            execute_duration: [0; COMMANDS_COUNT],
        }
    }

    pub fn cmd_left_queue(&self, cmd: &Command, duration: u64) {
        let index = cmd_index(cmd);
        self.queue_count[index] += 1;
        self.queue_duration[index] += duration;
    }

    pub fn cmd_executed(&self, cmd: &Command, duration: u64) {
        let index = cmd_index(cmd);
        self.execute_count[index] += 1;
        self.execute_duration[index] += duration;
    }


    pub fn cmd_index(cmd: &Command) -> usize {
        let idx: CommandIndexes = cmd.into();
        idx as usize
    }
    pub fn cmd_name(index: usize) -> &'static str {
        CommandIndexes[index]
    }
    const fn commands_count() -> usize {
        CommandIndexes::
    }
}

pub struct MetricsCommandExecutor {
    wallet_service: Rc<WalletService>,
    pub metric_storage: MetricsStorage,
}

impl MetricsCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> MetricsCommandExecutor {
        MetricsCommandExecutor {
            wallet_service,
            metric_storage: MetricsStorage::new(),
        }
    }

    pub fn execute(&self, command: MetricsCommand) {
        match command {
            MetricsCommand::CollectMetrics(cb) => {
                debug!(target: "metrics_command_executor", "CollectMetrics command received");
                cb(self._collect());
            }
        };
    }


    fn _collect(&self) -> IndyResult<String> {
        trace!("_collect >>>");
        let mut metrics_map: String = {
            let tp_instance = crate::commands::THREADPOOL.lock().unwrap();
            json!({
                THREADPOOL_ACTIVE_COUNT: tp_instance.active_count(),
                // (THREADPOOL_QUEUED_COUNT, tp_instance.queued_count()),
                // (THREADPOOL_MAX_COUNT, tp_instance.max_count()),
                // (THREADPOOL_PANIC_COUNT, tp_instance.panic_count()),
                // (OPENED_WALLETS_COUNT, self.wallet_service.get_wallets_count()),
                // (OPENED_WALLET_IDS_COUNT, self.wallet_service.get_wallet_ids_count()),
                // (PENDING_FOR_IMPORT_WALLETS_COUNT, self.wallet_service.get_pending_for_import_count()),
                // (PENDING_FOR_OPEN_WALLETS_COUNT, self.wallet_service.get_pending_for_open_count())
            }).to_string()
        };
        serde_json::Map::new();
        //
        // for index in (0..MetricsStorage::commands_count()).rev() {
        //     metrics_map.insert(format!("{}_execute_count", MetricsStorage::cmd_name(index)).as_str(),
        //                        self.metric_storage.execute_count[index]);
        //     metrics_map.insert(format!("{}_execute_duration", MetricsStorage::cmd_name(index)).as_str(),
        //                        self.metric_storage.execute_duration[index]);
        //     metrics_map.insert(format!("{}_queue_count", MetricsStorage::cmd_name(index)).as_str(),
        //                        self.metric_storage.queue_count[index]);
        //     metrics_map.insert(format!("{}_queue_duration", MetricsStorage::cmd_name(index)).as_str(),
        //                        self.metric_storage.queue_duration[index]);
        // }
        let res = serde_json::to_string(&metrics_map).unwrap();

        trace!("_collect <<< res: {:?}", res);
        debug!("collecting metrics from command thread");
        Ok(res)
    }
}
