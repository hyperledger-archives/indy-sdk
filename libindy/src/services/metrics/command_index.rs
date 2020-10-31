use variant_count::VariantCount;
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
use std::fmt;
use crate::commands::metrics::MetricsCommand;

impl fmt::Display for CommandIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<usize> for CommandIndex {
    fn from(i: usize) -> Self {
        let conversion = num_traits::FromPrimitive::from_usize(i);
        if conversion.is_some() {
            conversion.unwrap()
        } else {
            panic!("Unable to convert from {}, unknown error code", i)
        }
    }
}

impl From<&IssuerCommand> for CommandIndex {
    fn from(cmd: &IssuerCommand) -> Self {
        match cmd {
            IssuerCommand::CreateSchema(_, _, _, _, _) => {
                CommandIndex::IssuerCommandCreateSchema
            }
            IssuerCommand::CreateAndStoreCredentialDefinition(_, _, _, _, _, _, _) => {
                CommandIndex::IssuerCommandCreateAndStoreCredentialDefinition
            }
            IssuerCommand::CreateAndStoreCredentialDefinitionContinue(_, _, _, _, _, _, _, _) => {
                CommandIndex::IssuerCommandCreateAndStoreCredentialDefinitionContinue
            }
            IssuerCommand::RotateCredentialDefinitionStart(_, _, _, _) => {
                CommandIndex::IssuerCommandRotateCredentialDefinitionStart
            }
            IssuerCommand::RotateCredentialDefinitionStartComplete(_, _, _, _, _, _, _) => {
                CommandIndex::IssuerCommandRotateCredentialDefinitionStartComplete
            }
            IssuerCommand::RotateCredentialDefinitionApply(_, _, _) => {
                CommandIndex::IssuerCommandRotateCredentialDefinitionApply
            }
            IssuerCommand::CreateAndStoreRevocationRegistry(_, _, _, _, _, _, _, _) => {
                CommandIndex::IssuerCommandCreateAndStoreRevocationRegistry
            }
            IssuerCommand::CreateCredentialOffer(_, _, _) => {
                CommandIndex::IssuerCommandCreateCredentialOffer
            }
            IssuerCommand::CreateCredential(_, _, _, _, _, _, _) => {
                CommandIndex::IssuerCommandCreateCredential
            }
            IssuerCommand::RevokeCredential(_, _, _, _, _) => {
                CommandIndex::IssuerCommandRevokeCredential
            }
            IssuerCommand::MergeRevocationRegistryDeltas(_, _, _) => {
                CommandIndex::IssuerCommandMergeRevocationRegistryDeltas
            }
        }
    }
}

impl From<&ProverCommand> for CommandIndex {
    fn from(cmd: &ProverCommand) -> Self {
        match cmd {
            ProverCommand::CreateMasterSecret(_, _, _) => { CommandIndex::ProverCommandCreateMasterSecret }
            ProverCommand::CreateCredentialRequest(_, _, _, _, _, _) => { CommandIndex::ProverCommandCreateCredentialRequest }
            ProverCommand::SetCredentialAttrTagPolicy(_, _, _, _, _) => { CommandIndex::ProverCommandSetCredentialAttrTagPolicy }
            ProverCommand::GetCredentialAttrTagPolicy(_, _, _) => { CommandIndex::ProverCommandGetCredentialAttrTagPolicy }
            ProverCommand::StoreCredential(_, _, _, _, _, _, _) => { CommandIndex::ProverCommandStoreCredential }
            ProverCommand::GetCredentials(_, _, _) => { CommandIndex::ProverCommandGetCredentials }
            ProverCommand::GetCredential(_, _, _) => { CommandIndex::ProverCommandGetCredential }
            ProverCommand::DeleteCredential(_, _, _) => { CommandIndex::ProverCommandDeleteCredential }
            ProverCommand::SearchCredentials(_, _, _) => { CommandIndex::ProverCommandSearchCredentials }
            ProverCommand::FetchCredentials(_, _, _) => { CommandIndex::ProverCommandFetchCredentials }
            ProverCommand::CloseCredentialsSearch(_, _) => { CommandIndex::ProverCommandCloseCredentialsSearch }
            ProverCommand::GetCredentialsForProofReq(_, _, _) => { CommandIndex::ProverCommandGetCredentialsForProofReq }
            ProverCommand::SearchCredentialsForProofReq(_, _, _, _) => { CommandIndex::ProverCommandSearchCredentialsForProofReq }
            ProverCommand::FetchCredentialForProofReq(_, _, _, _) => { CommandIndex::ProverCommandFetchCredentialForProofReq }
            ProverCommand::CloseCredentialsSearchForProofReq(_, _) => { CommandIndex::ProverCommandCloseCredentialsSearchForProofReq }
            ProverCommand::CreateProof(_, _, _, _, _, _, _, _) => { CommandIndex::ProverCommandCreateProof }
            ProverCommand::CreateRevocationState(_, _, _, _, _, _) => { CommandIndex::ProverCommandCreateRevocationState }
            ProverCommand::UpdateRevocationState(_, _, _, _, _, _, _) => { CommandIndex::ProverCommandUpdateRevocationState }
        }
    }
}

impl From<&VerifierCommand> for CommandIndex {
    fn from(cmd: &VerifierCommand) -> Self {
        match cmd {
            VerifierCommand::VerifyProof(_, _, _, _, _, _, _) => { CommandIndex::VerifierCommandVerifyProof }
            VerifierCommand::GenerateNonce(_) => { CommandIndex::VerifierCommandGenerateNonce }
        }
    }
}

impl From<&Command> for CommandIndex {
    fn from(cmd: &Command) -> Self {
        match cmd {
            Command::Exit => { CommandIndex::Exit }
            Command::Anoncreds(cmd) => {
                match cmd {
                    AnoncredsCommand::Issuer(cmd) => { cmd.into() }
                    AnoncredsCommand::Prover(cmd) => { cmd.into() }
                    AnoncredsCommand::Verifier(cmd) => { cmd.into() }
                    AnoncredsCommand::ToUnqualified(_, _) => { CommandIndex::AnoncredsCommandToUnqualified }
                }
            }
            Command::BlobStorage(cmd) => {
                match cmd {
                    BlobStorageCommand::OpenReader(_, _, _) => { CommandIndex::BlobStorageCommandOpenReader }
                    BlobStorageCommand::OpenWriter(_, _, _) => { CommandIndex::BlobStorageCommandOpenWriter }
                }
            }
            Command::Crypto(cmd) => {
                match cmd {
                    CryptoCommand::CreateKey(_, _, _) => { CommandIndex::CryptoCommandCreateKey }
                    CryptoCommand::SetKeyMetadata(_, _, _, _) => { CommandIndex::CryptoCommandSetKeyMetadata }
                    CryptoCommand::GetKeyMetadata(_, _, _) => { CommandIndex::CryptoCommandGetKeyMetadata }
                    CryptoCommand::CryptoSign(_, _, _, _) => { CommandIndex::CryptoCommandCryptoSign }
                    CryptoCommand::CryptoVerify(_, _, _, _) => { CommandIndex::CryptoCommandCryptoVerify }
                    CryptoCommand::AuthenticatedEncrypt(_, _, _, _, _) => { CommandIndex::CryptoCommandAuthenticatedEncrypt }
                    CryptoCommand::AuthenticatedDecrypt(_, _, _, _) => { CommandIndex::CryptoCommandAuthenticatedDecrypt }
                    CryptoCommand::AnonymousEncrypt(_, _, _) => { CommandIndex::CryptoCommandAnonymousEncrypt }
                    CryptoCommand::AnonymousDecrypt(_, _, _, _) => { CommandIndex::CryptoCommandAnonymousDecrypt }
                    CryptoCommand::PackMessage(_, _, _, _, _) => { CommandIndex::CryptoCommandPackMessage }
                    CryptoCommand::UnpackMessage(_, _, _) => { CommandIndex::CryptoCommandUnpackMessage }
                }
            }
            Command::Ledger(cmd) => {
                match cmd {
                    LedgerCommand::SignAndSubmitRequest(_, _, _, _, _) => { CommandIndex::LedgerCommandSignAndSubmitRequest }
                    LedgerCommand::SubmitRequest(_, _, _) => { CommandIndex::LedgerCommandSubmitRequest }
                    LedgerCommand::SubmitAck(_, _) => { CommandIndex::LedgerCommandSubmitAck }
                    LedgerCommand::SubmitAction(_, _, _, _, _) => { CommandIndex::LedgerCommandSubmitAction }
                    LedgerCommand::SignRequest(_, _, _, _) => { CommandIndex::LedgerCommandSignRequest }
                    LedgerCommand::MultiSignRequest(_, _, _, _) => { CommandIndex::LedgerCommandMultiSignRequest }
                    LedgerCommand::BuildGetDdoRequest(_, _, _) => { CommandIndex::LedgerCommandBuildGetDdoRequest }
                    LedgerCommand::BuildNymRequest(_, _, _, _, _, _) => { CommandIndex::LedgerCommandBuildNymRequest }
                    LedgerCommand::BuildAttribRequest(_, _, _, _, _, _) => { CommandIndex::LedgerCommandBuildAttribRequest }
                    LedgerCommand::BuildGetAttribRequest(_, _, _, _, _, _) => { CommandIndex::LedgerCommandBuildGetAttribRequest }
                    LedgerCommand::BuildGetNymRequest(_, _, _) => { CommandIndex::LedgerCommandBuildGetNymRequest }
                    LedgerCommand::ParseGetNymResponse(_, _) => { CommandIndex::LedgerCommandParseGetNymResponse }
                    LedgerCommand::BuildSchemaRequest(_, _, _) => { CommandIndex::LedgerCommandBuildSchemaRequest }
                    LedgerCommand::BuildGetSchemaRequest(_, _, _) => { CommandIndex::LedgerCommandBuildGetSchemaRequest }
                    LedgerCommand::ParseGetSchemaResponse(_, _) => { CommandIndex::LedgerCommandParseGetSchemaResponse }
                    LedgerCommand::BuildCredDefRequest(_, _, _) => { CommandIndex::LedgerCommandBuildCredDefRequest }
                    LedgerCommand::BuildGetCredDefRequest(_, _, _) => { CommandIndex::LedgerCommandBuildGetCredDefRequest }
                    LedgerCommand::ParseGetCredDefResponse(_, _) => { CommandIndex::LedgerCommandParseGetCredDefResponse }
                    LedgerCommand::BuildNodeRequest(_, _, _, _) => { CommandIndex::LedgerCommandBuildNodeRequest }
                    LedgerCommand::BuildGetValidatorInfoRequest(_, _) => { CommandIndex::LedgerCommandBuildGetValidatorInfoRequest }
                    LedgerCommand::BuildGetTxnRequest(_, _, _, _) => { CommandIndex::LedgerCommandBuildGetTxnRequest }
                    LedgerCommand::BuildPoolConfigRequest(_, _, _, _) => { CommandIndex::LedgerCommandBuildPoolConfigRequest }
                    LedgerCommand::BuildPoolRestartRequest(_, _, _, _) => { CommandIndex::LedgerCommandBuildPoolRestartRequest }
                    LedgerCommand::BuildPoolUpgradeRequest(_, _, _, _, _, _, _, _, _, _, _, _) => { CommandIndex::LedgerCommandBuildPoolUpgradeRequest }
                    LedgerCommand::BuildRevocRegDefRequest(_, _, _) => { CommandIndex::LedgerCommandBuildRevocRegDefRequest }
                    LedgerCommand::BuildGetRevocRegDefRequest(_, _, _) => { CommandIndex::LedgerCommandBuildGetRevocRegDefRequest }
                    LedgerCommand::ParseGetRevocRegDefResponse(_, _) => { CommandIndex::LedgerCommandParseGetRevocRegDefResponse }
                    LedgerCommand::BuildRevocRegEntryRequest(_, _, _, _, _) => { CommandIndex::LedgerCommandBuildRevocRegEntryRequest }
                    LedgerCommand::BuildGetRevocRegRequest(_, _, _, _) => { CommandIndex::LedgerCommandBuildGetRevocRegRequest }
                    LedgerCommand::ParseGetRevocRegResponse(_, _) => { CommandIndex::LedgerCommandParseGetRevocRegResponse }
                    LedgerCommand::BuildGetRevocRegDeltaRequest(_, _, _, _, _) => { CommandIndex::LedgerCommandBuildGetRevocRegDeltaRequest }
                    LedgerCommand::ParseGetRevocRegDeltaResponse(_, _) => { CommandIndex::LedgerCommandParseGetRevocRegDeltaResponse }
                    LedgerCommand::RegisterSPParser(_, _, _, _) => { CommandIndex::LedgerCommandRegisterSPParser }
                    LedgerCommand::GetResponseMetadata(_, _) => { CommandIndex::LedgerCommandGetResponseMetadata }
                    LedgerCommand::BuildAuthRuleRequest(_, _, _, _, _, _, _, _) => { CommandIndex::LedgerCommandBuildAuthRuleRequest }
                    LedgerCommand::BuildAuthRulesRequest(_, _, _) => { CommandIndex::LedgerCommandBuildAuthRulesRequest }
                    LedgerCommand::BuildGetAuthRuleRequest(_, _, _, _, _, _, _) => { CommandIndex::LedgerCommandBuildGetAuthRuleRequest }
                    LedgerCommand::GetSchema(_, _, _, _) => { CommandIndex::LedgerCommandGetSchema }
                    LedgerCommand::GetSchemaContinue(_, _, _) => { CommandIndex::LedgerCommandGetSchemaContinue }
                    LedgerCommand::GetCredDef(_, _, _, _) => { CommandIndex::LedgerCommandGetCredDef }
                    LedgerCommand::GetCredDefContinue(_, _, _) => { CommandIndex::LedgerCommandGetCredDefContinue }
                    LedgerCommand::BuildTxnAuthorAgreementRequest(_, _, _, _, _, _) => { CommandIndex::LedgerCommandBuildTxnAuthorAgreementRequest }
                    LedgerCommand::BuildDisableAllTxnAuthorAgreementsRequest(_, _) => { CommandIndex::LedgerCommandBuildDisableAllTxnAuthorAgreementsRequest }
                    LedgerCommand::BuildGetTxnAuthorAgreementRequest(_, _, _) => { CommandIndex::LedgerCommandBuildGetTxnAuthorAgreementRequest }
                    LedgerCommand::BuildAcceptanceMechanismRequests(_, _, _, _, _) => { CommandIndex::LedgerCommandBuildAcceptanceMechanismRequests }
                    LedgerCommand::BuildGetAcceptanceMechanismsRequest(_, _, _, _) => { CommandIndex::LedgerCommandBuildGetAcceptanceMechanismsRequest }
                    LedgerCommand::AppendTxnAuthorAgreementAcceptanceToRequest(_, _, _, _, _, _, _) => { CommandIndex::LedgerCommandAppendTxnAuthorAgreementAcceptanceToRequest }
                    LedgerCommand::AppendRequestEndorser(_, _, _) => { CommandIndex::LedgerCommandAppendRequestEndorser }
                }
            }
            Command::Pool(cmd) => {
                match cmd {
                    PoolCommand::Create(_, _, _) => { CommandIndex::PoolCommandCreate }
                    PoolCommand::Delete(_, _) => { CommandIndex::PoolCommandDelete }
                    PoolCommand::Open(_, _, _) => { CommandIndex::PoolCommandOpen }
                    PoolCommand::OpenAck(_, _, _) => { CommandIndex::PoolCommandOpenAck }
                    PoolCommand::List(_) => { CommandIndex::PoolCommandList }
                    PoolCommand::Close(_, _) => { CommandIndex::PoolCommandClose }
                    PoolCommand::CloseAck(_, _) => { CommandIndex::PoolCommandCloseAck }
                    PoolCommand::Refresh(_, _) => { CommandIndex::PoolCommandRefresh }
                    PoolCommand::RefreshAck(_, _) => { CommandIndex::PoolCommandRefreshAck }
                    PoolCommand::SetProtocolVersion(_, _) => { CommandIndex::PoolCommandSetProtocolVersion }
                }
            }
            Command::Did(cmd) => {
                match cmd {
                    DidCommand::CreateAndStoreMyDid(_, _, _) => { CommandIndex::DidCommandCreateAndStoreMyDid }
                    DidCommand::ReplaceKeysStart(_, _, _, _) => { CommandIndex::DidCommandReplaceKeysStart }
                    DidCommand::ReplaceKeysApply(_, _, _) => { CommandIndex::DidCommandReplaceKeysApply }
                    DidCommand::StoreTheirDid(_, _, _) => { CommandIndex::DidCommandStoreTheirDid }
                    DidCommand::GetMyDidWithMeta(_, _, _) => { CommandIndex::DidCommandGetMyDidWithMeta }
                    DidCommand::ListMyDidsWithMeta(_, _) => { CommandIndex::DidCommandListMyDidsWithMeta }
                    DidCommand::KeyForDid(_, _, _, _) => { CommandIndex::DidCommandKeyForDid }
                    DidCommand::KeyForLocalDid(_, _, _) => { CommandIndex::DidCommandKeyForLocalDid }
                    DidCommand::SetEndpointForDid(_, _, _, _) => { CommandIndex::DidCommandSetEndpointForDid }
                    DidCommand::GetEndpointForDid(_, _, _, _) => { CommandIndex::DidCommandGetEndpointForDid }
                    DidCommand::SetDidMetadata(_, _, _, _) => { CommandIndex::DidCommandSetDidMetadata }
                    DidCommand::GetDidMetadata(_, _, _) => { CommandIndex::DidCommandGetDidMetadata }
                    DidCommand::AbbreviateVerkey(_, _, _) => { CommandIndex::DidCommandAbbreviateVerkey }
                    DidCommand::GetNymAck(_, _, _, _) => { CommandIndex::DidCommandGetNymAck }
                    DidCommand::GetAttribAck(_, _, _) => { CommandIndex::DidCommandGetAttribAck }
                    DidCommand::QualifyDid(_, _, _, _) => { CommandIndex::DidCommandQualifyDid }
                }
            }
            Command::Wallet(cmd) => {
                match cmd {
                    WalletCommand::RegisterWalletType(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => { CommandIndex::WalletCommandRegisterWalletType }
                    WalletCommand::Create(_, _, _) => { CommandIndex::WalletCommandCreate }
                    WalletCommand::CreateContinue(_, _, _, _, _) => { CommandIndex::WalletCommandCreateContinue }
                    WalletCommand::Open(_, _, _) => { CommandIndex::WalletCommandOpen }
                    WalletCommand::OpenContinue(_, _) => { CommandIndex::WalletCommandOpenContinue }
                    WalletCommand::Close(_, _) => { CommandIndex::WalletCommandClose }
                    WalletCommand::Delete(_, _, _) => { CommandIndex::WalletCommandDelete }
                    WalletCommand::DeleteContinue(_, _, _, _, _) => { CommandIndex::WalletCommandDeleteContinue }
                    WalletCommand::Export(_, _, _) => { CommandIndex::WalletCommandExport }
                    WalletCommand::ExportContinue(_, _, _, _, _) => { CommandIndex::WalletCommandExportContinue }
                    WalletCommand::Import(_, _, _, _) => { CommandIndex::WalletCommandImport }
                    WalletCommand::ImportContinue(_, _, _, _, _) => { CommandIndex::WalletCommandImportContinue }
                    WalletCommand::GenerateKey(_, _) => { CommandIndex::WalletCommandGenerateKey }
                    WalletCommand::DeriveKey(_, _) => { CommandIndex::WalletCommandDeriveKey }
                }
            }
            Command::Pairwise(cmd) => {
                match cmd {
                    PairwiseCommand::PairwiseExists(_, _, _) => { CommandIndex::PairwiseCommandPairwiseExists }
                    PairwiseCommand::CreatePairwise(_, _, _, _, _) => { CommandIndex::PairwiseCommandCreatePairwise }
                    PairwiseCommand::ListPairwise(_, _) => { CommandIndex::PairwiseCommandListPairwise }
                    PairwiseCommand::GetPairwise(_, _, _) => { CommandIndex::PairwiseCommandGetPairwise }
                    PairwiseCommand::SetPairwiseMetadata(_, _, _, _) => { CommandIndex::PairwiseCommandSetPairwiseMetadata }
                }
            }
            Command::NonSecrets(cmd) => {
                match cmd {
                    NonSecretsCommand::AddRecord(_, _, _, _, _, _) => { CommandIndex::NonSecretsCommandAddRecord }
                    NonSecretsCommand::UpdateRecordValue(_, _, _, _, _) => { CommandIndex::NonSecretsCommandUpdateRecordValue }
                    NonSecretsCommand::UpdateRecordTags(_, _, _, _, _) => { CommandIndex::NonSecretsCommandUpdateRecordTags }
                    NonSecretsCommand::AddRecordTags(_, _, _, _, _) => { CommandIndex::NonSecretsCommandAddRecordTags }
                    NonSecretsCommand::DeleteRecordTags(_, _, _, _, _) => { CommandIndex::NonSecretsCommandDeleteRecordTags }
                    NonSecretsCommand::DeleteRecord(_, _, _, _) => { CommandIndex::NonSecretsCommandDeleteRecord }
                    NonSecretsCommand::GetRecord(_, _, _, _, _) => { CommandIndex::NonSecretsCommandGetRecord }
                    NonSecretsCommand::OpenSearch(_, _, _, _, _) => { CommandIndex::NonSecretsCommandOpenSearch }
                    NonSecretsCommand::FetchSearchNextRecords(_, _, _, _) => { CommandIndex::NonSecretsCommandFetchSearchNextRecords }
                    NonSecretsCommand::CloseSearch(_, _) => { CommandIndex::NonSecretsCommandCloseSearch }
                }
            }
            Command::Payments(cmd) => {
                match cmd {
                    PaymentsCommand::RegisterMethod(_, _, _) => { CommandIndex::PaymentsCommandRegisterMethod }
                    PaymentsCommand::CreateAddress(_, _, _, _) => { CommandIndex::PaymentsCommandCreateAddress }
                    PaymentsCommand::CreateAddressAck(_, _, _) => { CommandIndex::PaymentsCommandCreateAddressAck }
                    PaymentsCommand::ListAddresses(_, _) => { CommandIndex::PaymentsCommandListAddresses }
                    PaymentsCommand::AddRequestFees(_, _, _, _, _, _, _) => { CommandIndex::PaymentsCommandAddRequestFees }
                    PaymentsCommand::AddRequestFeesAck(_, _) => { CommandIndex::PaymentsCommandAddRequestFeesAck }
                    PaymentsCommand::ParseResponseWithFees(_, _, _) => { CommandIndex::PaymentsCommandParseResponseWithFees }
                    PaymentsCommand::ParseResponseWithFeesAck(_, _) => { CommandIndex::PaymentsCommandParseResponseWithFeesAck }
                    PaymentsCommand::BuildGetPaymentSourcesRequest(_, _, _, _, _) => { CommandIndex::PaymentsCommandBuildGetPaymentSourcesRequest }
                    PaymentsCommand::BuildGetPaymentSourcesRequestAck(_, _) => { CommandIndex::PaymentsCommandBuildGetPaymentSourcesRequestAck }
                    PaymentsCommand::ParseGetPaymentSourcesResponse(_, _, _) => { CommandIndex::PaymentsCommandParseGetPaymentSourcesResponse }
                    PaymentsCommand::ParseGetPaymentSourcesResponseAck(_, _) => { CommandIndex::PaymentsCommandParseGetPaymentSourcesResponseAck }
                    PaymentsCommand::BuildPaymentReq(_, _, _, _, _, _) => { CommandIndex::PaymentsCommandBuildPaymentReq }
                    PaymentsCommand::BuildPaymentReqAck(_, _) => { CommandIndex::PaymentsCommandBuildPaymentReqAck }
                    PaymentsCommand::ParsePaymentResponse(_, _, _) => { CommandIndex::PaymentsCommandParsePaymentResponse }
                    PaymentsCommand::ParsePaymentResponseAck(_, _) => { CommandIndex::PaymentsCommandParsePaymentResponseAck }
                    PaymentsCommand::AppendTxnAuthorAgreementAcceptanceToExtra(_, _, _, _, _, _, _) => { CommandIndex::PaymentsCommandAppendTxnAuthorAgreementAcceptanceToExtra }
                    PaymentsCommand::BuildMintReq(_, _, _, _, _) => { CommandIndex::PaymentsCommandBuildMintReq }
                    PaymentsCommand::BuildMintReqAck(_, _) => { CommandIndex::PaymentsCommandBuildMintReqAck }
                    PaymentsCommand::BuildSetTxnFeesReq(_, _, _, _, _) => { CommandIndex::PaymentsCommandBuildSetTxnFeesReq }
                    PaymentsCommand::BuildSetTxnFeesReqAck(_, _) => { CommandIndex::PaymentsCommandBuildSetTxnFeesReqAck }
                    PaymentsCommand::BuildGetTxnFeesReq(_, _, _, _) => { CommandIndex::PaymentsCommandBuildGetTxnFeesReq }
                    PaymentsCommand::BuildGetTxnFeesReqAck(_, _) => { CommandIndex::PaymentsCommandBuildGetTxnFeesReqAck }
                    PaymentsCommand::ParseGetTxnFeesResponse(_, _, _) => { CommandIndex::PaymentsCommandParseGetTxnFeesResponse }
                    PaymentsCommand::ParseGetTxnFeesResponseAck(_, _) => { CommandIndex::PaymentsCommandParseGetTxnFeesResponseAck }
                    PaymentsCommand::BuildVerifyPaymentReq(_, _, _, _) => { CommandIndex::PaymentsCommandBuildVerifyPaymentReq }
                    PaymentsCommand::BuildVerifyPaymentReqAck(_, _) => { CommandIndex::PaymentsCommandBuildVerifyPaymentReqAck }
                    PaymentsCommand::ParseVerifyPaymentResponse(_, _, _) => { CommandIndex::PaymentsCommandParseVerifyPaymentResponse }
                    PaymentsCommand::ParseVerifyPaymentResponseAck(_, _) => { CommandIndex::PaymentsCommandParseVerifyPaymentResponseAck }
                    PaymentsCommand::GetRequestInfo(_, _, _, _) => { CommandIndex::PaymentsCommandGetRequestInfo }
                    PaymentsCommand::SignWithAddressReq(_, _, _, _) => { CommandIndex::PaymentsCommandSignWithAddressReq }
                    PaymentsCommand::SignWithAddressAck(_, _) => { CommandIndex::PaymentsCommandSignWithAddressAck }
                    PaymentsCommand::VerifyWithAddressReq(_, _, _, _) => { CommandIndex::PaymentsCommandVerifyWithAddressReq }
                    PaymentsCommand::VerifyWithAddressAck(_, _) => { CommandIndex::PaymentsCommandVerifyWithAddressAck }
                }
            }
            Command::Cache(cmd) => {
                match cmd {
                    CacheCommand::GetSchema(_, _, _, _, _, _) => { CommandIndex::CacheCommandGetSchema }
                    CacheCommand::GetSchemaContinue(_, _, _, _) => { CommandIndex::CacheCommandGetSchemaContinue }
                    CacheCommand::GetCredDef(_, _, _, _, _, _) => { CommandIndex::CacheCommandGetCredDef }
                    CacheCommand::GetCredDefContinue(_, _, _, _) => { CommandIndex::CacheCommandGetCredDefContinue }
                    CacheCommand::PurgeSchemaCache(_, _, _) => { CommandIndex::CacheCommandPurgeSchemaCache }
                    CacheCommand::PurgeCredDefCache(_, _, _) => { CommandIndex::CacheCommandPurgeCredDefCache }
                }
            }
            Command::Metrics(cmd) => {
                match cmd { MetricsCommand::CollectMetrics(_) => { CommandIndex::MetricsCommandCollectMetrics } }
            }
        }
    }
}


#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive, VariantCount)]
#[repr(i32)]
pub enum CommandIndex {
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

