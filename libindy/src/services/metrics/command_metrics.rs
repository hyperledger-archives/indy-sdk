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

impl fmt::Display for CommandMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<usize> for CommandMetric {
    fn from(i: usize) -> Self {
        let conversion = num_traits::FromPrimitive::from_usize(i);
        if conversion.is_some() {
            conversion.unwrap()
        } else {
            panic!("Unable to convert from {}, unknown error code", i)
        }
    }
}

impl From<&IssuerCommand> for CommandMetric {
    fn from(cmd: &IssuerCommand) -> Self {
        match cmd {
            IssuerCommand::CreateSchema(_, _, _, _, _) => {
                CommandMetric::IssuerCommandCreateSchema
            }
            IssuerCommand::CreateAndStoreCredentialDefinition(_, _, _, _, _, _, _) => {
                CommandMetric::IssuerCommandCreateAndStoreCredentialDefinition
            }
            IssuerCommand::CreateAndStoreCredentialDefinitionContinue(_, _, _, _, _, _, _, _) => {
                CommandMetric::IssuerCommandCreateAndStoreCredentialDefinitionContinue
            }
            IssuerCommand::RotateCredentialDefinitionStart(_, _, _, _) => {
                CommandMetric::IssuerCommandRotateCredentialDefinitionStart
            }
            IssuerCommand::RotateCredentialDefinitionStartComplete(_, _, _, _, _, _, _) => {
                CommandMetric::IssuerCommandRotateCredentialDefinitionStartComplete
            }
            IssuerCommand::RotateCredentialDefinitionApply(_, _, _) => {
                CommandMetric::IssuerCommandRotateCredentialDefinitionApply
            }
            IssuerCommand::CreateAndStoreRevocationRegistry(_, _, _, _, _, _, _, _) => {
                CommandMetric::IssuerCommandCreateAndStoreRevocationRegistry
            }
            IssuerCommand::CreateCredentialOffer(_, _, _) => {
                CommandMetric::IssuerCommandCreateCredentialOffer
            }
            IssuerCommand::CreateCredential(_, _, _, _, _, _, _) => {
                CommandMetric::IssuerCommandCreateCredential
            }
            IssuerCommand::RevokeCredential(_, _, _, _, _) => {
                CommandMetric::IssuerCommandRevokeCredential
            }
            IssuerCommand::MergeRevocationRegistryDeltas(_, _, _) => {
                CommandMetric::IssuerCommandMergeRevocationRegistryDeltas
            }
        }
    }
}

impl From<&ProverCommand> for CommandMetric {
    fn from(cmd: &ProverCommand) -> Self {
        match cmd {
            ProverCommand::CreateMasterSecret(_, _, _) => { CommandMetric::ProverCommandCreateMasterSecret }
            ProverCommand::CreateCredentialRequest(_, _, _, _, _, _) => { CommandMetric::ProverCommandCreateCredentialRequest }
            ProverCommand::SetCredentialAttrTagPolicy(_, _, _, _, _) => { CommandMetric::ProverCommandSetCredentialAttrTagPolicy }
            ProverCommand::GetCredentialAttrTagPolicy(_, _, _) => { CommandMetric::ProverCommandGetCredentialAttrTagPolicy }
            ProverCommand::StoreCredential(_, _, _, _, _, _, _) => { CommandMetric::ProverCommandStoreCredential }
            ProverCommand::GetCredentials(_, _, _) => { CommandMetric::ProverCommandGetCredentials }
            ProverCommand::GetCredential(_, _, _) => { CommandMetric::ProverCommandGetCredential }
            ProverCommand::DeleteCredential(_, _, _) => { CommandMetric::ProverCommandDeleteCredential }
            ProverCommand::SearchCredentials(_, _, _) => { CommandMetric::ProverCommandSearchCredentials }
            ProverCommand::FetchCredentials(_, _, _) => { CommandMetric::ProverCommandFetchCredentials }
            ProverCommand::CloseCredentialsSearch(_, _) => { CommandMetric::ProverCommandCloseCredentialsSearch }
            ProverCommand::GetCredentialsForProofReq(_, _, _) => { CommandMetric::ProverCommandGetCredentialsForProofReq }
            ProverCommand::SearchCredentialsForProofReq(_, _, _, _) => { CommandMetric::ProverCommandSearchCredentialsForProofReq }
            ProverCommand::FetchCredentialForProofReq(_, _, _, _) => { CommandMetric::ProverCommandFetchCredentialForProofReq }
            ProverCommand::CloseCredentialsSearchForProofReq(_, _) => { CommandMetric::ProverCommandCloseCredentialsSearchForProofReq }
            ProverCommand::CreateProof(_, _, _, _, _, _, _, _) => { CommandMetric::ProverCommandCreateProof }
            ProverCommand::CreateRevocationState(_, _, _, _, _, _) => { CommandMetric::ProverCommandCreateRevocationState }
            ProverCommand::UpdateRevocationState(_, _, _, _, _, _, _) => { CommandMetric::ProverCommandUpdateRevocationState }
        }
    }
}

impl From<&VerifierCommand> for CommandMetric {
    fn from(cmd: &VerifierCommand) -> Self {
        match cmd {
            VerifierCommand::VerifyProof(_, _, _, _, _, _, _) => { CommandMetric::VerifierCommandVerifyProof }
            VerifierCommand::GenerateNonce(_) => { CommandMetric::VerifierCommandGenerateNonce }
        }
    }
}

impl From<&Command> for CommandMetric {
    fn from(cmd: &Command) -> Self {
        match cmd {
            Command::Exit => { CommandMetric::Exit }
            Command::Anoncreds(cmd) => {
                match cmd {
                    AnoncredsCommand::Issuer(cmd) => { cmd.into() }
                    AnoncredsCommand::Prover(cmd) => { cmd.into() }
                    AnoncredsCommand::Verifier(cmd) => { cmd.into() }
                    AnoncredsCommand::ToUnqualified(_, _) => { CommandMetric::AnoncredsCommandToUnqualified }
                }
            }
            Command::BlobStorage(cmd) => {
                match cmd {
                    BlobStorageCommand::OpenReader(_, _, _) => { CommandMetric::BlobStorageCommandOpenReader }
                    BlobStorageCommand::OpenWriter(_, _, _) => { CommandMetric::BlobStorageCommandOpenWriter }
                }
            }
            Command::Crypto(cmd) => {
                match cmd {
                    CryptoCommand::CreateKey(_, _, _) => { CommandMetric::CryptoCommandCreateKey }
                    CryptoCommand::SetKeyMetadata(_, _, _, _) => { CommandMetric::CryptoCommandSetKeyMetadata }
                    CryptoCommand::GetKeyMetadata(_, _, _) => { CommandMetric::CryptoCommandGetKeyMetadata }
                    CryptoCommand::CryptoSign(_, _, _, _) => { CommandMetric::CryptoCommandCryptoSign }
                    CryptoCommand::CryptoVerify(_, _, _, _) => { CommandMetric::CryptoCommandCryptoVerify }
                    CryptoCommand::AuthenticatedEncrypt(_, _, _, _, _) => { CommandMetric::CryptoCommandAuthenticatedEncrypt }
                    CryptoCommand::AuthenticatedDecrypt(_, _, _, _) => { CommandMetric::CryptoCommandAuthenticatedDecrypt }
                    CryptoCommand::AnonymousEncrypt(_, _, _) => { CommandMetric::CryptoCommandAnonymousEncrypt }
                    CryptoCommand::AnonymousDecrypt(_, _, _, _) => { CommandMetric::CryptoCommandAnonymousDecrypt }
                    CryptoCommand::PackMessage(_, _, _, _, _) => { CommandMetric::CryptoCommandPackMessage }
                    CryptoCommand::UnpackMessage(_, _, _) => { CommandMetric::CryptoCommandUnpackMessage }
                }
            }
            Command::Ledger(cmd) => {
                match cmd {
                    LedgerCommand::SignAndSubmitRequest(_, _, _, _, _) => { CommandMetric::LedgerCommandSignAndSubmitRequest }
                    LedgerCommand::SubmitRequest(_, _, _) => { CommandMetric::LedgerCommandSubmitRequest }
                    LedgerCommand::SubmitAck(_, _) => { CommandMetric::LedgerCommandSubmitAck }
                    LedgerCommand::SubmitAction(_, _, _, _, _) => { CommandMetric::LedgerCommandSubmitAction }
                    LedgerCommand::SignRequest(_, _, _, _) => { CommandMetric::LedgerCommandSignRequest }
                    LedgerCommand::MultiSignRequest(_, _, _, _) => { CommandMetric::LedgerCommandMultiSignRequest }
                    LedgerCommand::BuildGetDdoRequest(_, _, _) => { CommandMetric::LedgerCommandBuildGetDdoRequest }
                    LedgerCommand::BuildNymRequest(_, _, _, _, _, _) => { CommandMetric::LedgerCommandBuildNymRequest }
                    LedgerCommand::BuildAttribRequest(_, _, _, _, _, _) => { CommandMetric::LedgerCommandBuildAttribRequest }
                    LedgerCommand::BuildGetAttribRequest(_, _, _, _, _, _) => { CommandMetric::LedgerCommandBuildGetAttribRequest }
                    LedgerCommand::BuildGetNymRequest(_, _, _) => { CommandMetric::LedgerCommandBuildGetNymRequest }
                    LedgerCommand::ParseGetNymResponse(_, _) => { CommandMetric::LedgerCommandParseGetNymResponse }
                    LedgerCommand::BuildSchemaRequest(_, _, _) => { CommandMetric::LedgerCommandBuildSchemaRequest }
                    LedgerCommand::BuildGetSchemaRequest(_, _, _) => { CommandMetric::LedgerCommandBuildGetSchemaRequest }
                    LedgerCommand::ParseGetSchemaResponse(_, _) => { CommandMetric::LedgerCommandParseGetSchemaResponse }
                    LedgerCommand::BuildCredDefRequest(_, _, _) => { CommandMetric::LedgerCommandBuildCredDefRequest }
                    LedgerCommand::BuildGetCredDefRequest(_, _, _) => { CommandMetric::LedgerCommandBuildGetCredDefRequest }
                    LedgerCommand::ParseGetCredDefResponse(_, _) => { CommandMetric::LedgerCommandParseGetCredDefResponse }
                    LedgerCommand::BuildNodeRequest(_, _, _, _) => { CommandMetric::LedgerCommandBuildNodeRequest }
                    LedgerCommand::BuildGetValidatorInfoRequest(_, _) => { CommandMetric::LedgerCommandBuildGetValidatorInfoRequest }
                    LedgerCommand::BuildGetTxnRequest(_, _, _, _) => { CommandMetric::LedgerCommandBuildGetTxnRequest }
                    LedgerCommand::BuildPoolConfigRequest(_, _, _, _) => { CommandMetric::LedgerCommandBuildPoolConfigRequest }
                    LedgerCommand::BuildPoolRestartRequest(_, _, _, _) => { CommandMetric::LedgerCommandBuildPoolRestartRequest }
                    LedgerCommand::BuildPoolUpgradeRequest(_, _, _, _, _, _, _, _, _, _, _, _) => { CommandMetric::LedgerCommandBuildPoolUpgradeRequest }
                    LedgerCommand::BuildRevocRegDefRequest(_, _, _) => { CommandMetric::LedgerCommandBuildRevocRegDefRequest }
                    LedgerCommand::BuildGetRevocRegDefRequest(_, _, _) => { CommandMetric::LedgerCommandBuildGetRevocRegDefRequest }
                    LedgerCommand::ParseGetRevocRegDefResponse(_, _) => { CommandMetric::LedgerCommandParseGetRevocRegDefResponse }
                    LedgerCommand::BuildRevocRegEntryRequest(_, _, _, _, _) => { CommandMetric::LedgerCommandBuildRevocRegEntryRequest }
                    LedgerCommand::BuildGetRevocRegRequest(_, _, _, _) => { CommandMetric::LedgerCommandBuildGetRevocRegRequest }
                    LedgerCommand::ParseGetRevocRegResponse(_, _) => { CommandMetric::LedgerCommandParseGetRevocRegResponse }
                    LedgerCommand::BuildGetRevocRegDeltaRequest(_, _, _, _, _) => { CommandMetric::LedgerCommandBuildGetRevocRegDeltaRequest }
                    LedgerCommand::ParseGetRevocRegDeltaResponse(_, _) => { CommandMetric::LedgerCommandParseGetRevocRegDeltaResponse }
                    LedgerCommand::RegisterSPParser(_, _, _, _) => { CommandMetric::LedgerCommandRegisterSPParser }
                    LedgerCommand::GetResponseMetadata(_, _) => { CommandMetric::LedgerCommandGetResponseMetadata }
                    LedgerCommand::BuildAuthRuleRequest(_, _, _, _, _, _, _, _) => { CommandMetric::LedgerCommandBuildAuthRuleRequest }
                    LedgerCommand::BuildAuthRulesRequest(_, _, _) => { CommandMetric::LedgerCommandBuildAuthRulesRequest }
                    LedgerCommand::BuildGetAuthRuleRequest(_, _, _, _, _, _, _) => { CommandMetric::LedgerCommandBuildGetAuthRuleRequest }
                    LedgerCommand::GetSchema(_, _, _, _) => { CommandMetric::LedgerCommandGetSchema }
                    LedgerCommand::GetSchemaContinue(_, _, _) => { CommandMetric::LedgerCommandGetSchemaContinue }
                    LedgerCommand::GetCredDef(_, _, _, _) => { CommandMetric::LedgerCommandGetCredDef }
                    LedgerCommand::GetCredDefContinue(_, _, _) => { CommandMetric::LedgerCommandGetCredDefContinue }
                    LedgerCommand::BuildTxnAuthorAgreementRequest(_, _, _, _, _, _) => { CommandMetric::LedgerCommandBuildTxnAuthorAgreementRequest }
                    LedgerCommand::BuildDisableAllTxnAuthorAgreementsRequest(_, _) => { CommandMetric::LedgerCommandBuildDisableAllTxnAuthorAgreementsRequest }
                    LedgerCommand::BuildGetTxnAuthorAgreementRequest(_, _, _) => { CommandMetric::LedgerCommandBuildGetTxnAuthorAgreementRequest }
                    LedgerCommand::BuildAcceptanceMechanismRequests(_, _, _, _, _) => { CommandMetric::LedgerCommandBuildAcceptanceMechanismRequests }
                    LedgerCommand::BuildGetAcceptanceMechanismsRequest(_, _, _, _) => { CommandMetric::LedgerCommandBuildGetAcceptanceMechanismsRequest }
                    LedgerCommand::AppendTxnAuthorAgreementAcceptanceToRequest(_, _, _, _, _, _, _) => { CommandMetric::LedgerCommandAppendTxnAuthorAgreementAcceptanceToRequest }
                    LedgerCommand::AppendRequestEndorser(_, _, _) => { CommandMetric::LedgerCommandAppendRequestEndorser }
                    LedgerCommand::BuildGetFrozenLedgersRequest(_,_,) => { CommandMetric::LedgerCommandBuildGetFrozenLedgersRequest }
                    LedgerCommand::BuildLedgersFreezeRequest(_,_,_,) => { CommandMetric::LedgerCommandBuildLedgersFreezeRequest }
                }
            }
            Command::Pool(cmd) => {
                match cmd {
                    PoolCommand::Create(_, _, _) => { CommandMetric::PoolCommandCreate }
                    PoolCommand::Delete(_, _) => { CommandMetric::PoolCommandDelete }
                    PoolCommand::Open(_, _, _) => { CommandMetric::PoolCommandOpen }
                    PoolCommand::OpenAck(_, _, _) => { CommandMetric::PoolCommandOpenAck }
                    PoolCommand::List(_) => { CommandMetric::PoolCommandList }
                    PoolCommand::Close(_, _) => { CommandMetric::PoolCommandClose }
                    PoolCommand::CloseAck(_, _) => { CommandMetric::PoolCommandCloseAck }
                    PoolCommand::Refresh(_, _) => { CommandMetric::PoolCommandRefresh }
                    PoolCommand::RefreshAck(_, _) => { CommandMetric::PoolCommandRefreshAck }
                    PoolCommand::SetProtocolVersion(_, _) => { CommandMetric::PoolCommandSetProtocolVersion }
                }
            }
            Command::Did(cmd) => {
                match cmd {
                    DidCommand::CreateAndStoreMyDid(_, _, _) => { CommandMetric::DidCommandCreateAndStoreMyDid }
                    DidCommand::ReplaceKeysStart(_, _, _, _) => { CommandMetric::DidCommandReplaceKeysStart }
                    DidCommand::ReplaceKeysApply(_, _, _) => { CommandMetric::DidCommandReplaceKeysApply }
                    DidCommand::StoreTheirDid(_, _, _) => { CommandMetric::DidCommandStoreTheirDid }
                    DidCommand::GetMyDidWithMeta(_, _, _) => { CommandMetric::DidCommandGetMyDidWithMeta }
                    DidCommand::ListMyDidsWithMeta(_, _) => { CommandMetric::DidCommandListMyDidsWithMeta }
                    DidCommand::KeyForDid(_, _, _, _) => { CommandMetric::DidCommandKeyForDid }
                    DidCommand::KeyForLocalDid(_, _, _) => { CommandMetric::DidCommandKeyForLocalDid }
                    DidCommand::SetEndpointForDid(_, _, _, _) => { CommandMetric::DidCommandSetEndpointForDid }
                    DidCommand::GetEndpointForDid(_, _, _, _) => { CommandMetric::DidCommandGetEndpointForDid }
                    DidCommand::SetDidMetadata(_, _, _, _) => { CommandMetric::DidCommandSetDidMetadata }
                    DidCommand::GetDidMetadata(_, _, _) => { CommandMetric::DidCommandGetDidMetadata }
                    DidCommand::AbbreviateVerkey(_, _, _) => { CommandMetric::DidCommandAbbreviateVerkey }
                    DidCommand::GetNymAck(_, _, _, _) => { CommandMetric::DidCommandGetNymAck }
                    DidCommand::GetAttribAck(_, _, _) => { CommandMetric::DidCommandGetAttribAck }
                    DidCommand::QualifyDid(_, _, _, _) => { CommandMetric::DidCommandQualifyDid }
                }
            }
            Command::Wallet(cmd) => {
                match cmd {
                    WalletCommand::RegisterWalletType(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => { CommandMetric::WalletCommandRegisterWalletType }
                    WalletCommand::Create(_, _, _) => { CommandMetric::WalletCommandCreate }
                    WalletCommand::CreateContinue(_, _, _, _, _) => { CommandMetric::WalletCommandCreateContinue }
                    WalletCommand::Open(_, _, _) => { CommandMetric::WalletCommandOpen }
                    WalletCommand::OpenContinue(_, _) => { CommandMetric::WalletCommandOpenContinue }
                    WalletCommand::Close(_, _) => { CommandMetric::WalletCommandClose }
                    WalletCommand::Delete(_, _, _) => { CommandMetric::WalletCommandDelete }
                    WalletCommand::DeleteContinue(_, _, _, _, _) => { CommandMetric::WalletCommandDeleteContinue }
                    WalletCommand::Export(_, _, _) => { CommandMetric::WalletCommandExport }
                    WalletCommand::ExportContinue(_, _, _, _, _) => { CommandMetric::WalletCommandExportContinue }
                    WalletCommand::Import(_, _, _, _) => { CommandMetric::WalletCommandImport }
                    WalletCommand::ImportContinue(_, _, _, _, _) => { CommandMetric::WalletCommandImportContinue }
                    WalletCommand::GenerateKey(_, _) => { CommandMetric::WalletCommandGenerateKey }
                    WalletCommand::DeriveKey(_, _) => { CommandMetric::WalletCommandDeriveKey }
                }
            }
            Command::Pairwise(cmd) => {
                match cmd {
                    PairwiseCommand::PairwiseExists(_, _, _) => { CommandMetric::PairwiseCommandPairwiseExists }
                    PairwiseCommand::CreatePairwise(_, _, _, _, _) => { CommandMetric::PairwiseCommandCreatePairwise }
                    PairwiseCommand::ListPairwise(_, _) => { CommandMetric::PairwiseCommandListPairwise }
                    PairwiseCommand::GetPairwise(_, _, _) => { CommandMetric::PairwiseCommandGetPairwise }
                    PairwiseCommand::SetPairwiseMetadata(_, _, _, _) => { CommandMetric::PairwiseCommandSetPairwiseMetadata }
                }
            }
            Command::NonSecrets(cmd) => {
                match cmd {
                    NonSecretsCommand::AddRecord(_, _, _, _, _, _) => { CommandMetric::NonSecretsCommandAddRecord }
                    NonSecretsCommand::UpdateRecordValue(_, _, _, _, _) => { CommandMetric::NonSecretsCommandUpdateRecordValue }
                    NonSecretsCommand::UpdateRecordTags(_, _, _, _, _) => { CommandMetric::NonSecretsCommandUpdateRecordTags }
                    NonSecretsCommand::AddRecordTags(_, _, _, _, _) => { CommandMetric::NonSecretsCommandAddRecordTags }
                    NonSecretsCommand::DeleteRecordTags(_, _, _, _, _) => { CommandMetric::NonSecretsCommandDeleteRecordTags }
                    NonSecretsCommand::DeleteRecord(_, _, _, _) => { CommandMetric::NonSecretsCommandDeleteRecord }
                    NonSecretsCommand::GetRecord(_, _, _, _, _) => { CommandMetric::NonSecretsCommandGetRecord }
                    NonSecretsCommand::OpenSearch(_, _, _, _, _) => { CommandMetric::NonSecretsCommandOpenSearch }
                    NonSecretsCommand::FetchSearchNextRecords(_, _, _, _) => { CommandMetric::NonSecretsCommandFetchSearchNextRecords }
                    NonSecretsCommand::CloseSearch(_, _) => { CommandMetric::NonSecretsCommandCloseSearch }
                }
            }
            Command::Payments(cmd) => {
                match cmd {
                    PaymentsCommand::RegisterMethod(_, _, _) => { CommandMetric::PaymentsCommandRegisterMethod }
                    PaymentsCommand::CreateAddress(_, _, _, _) => { CommandMetric::PaymentsCommandCreateAddress }
                    PaymentsCommand::CreateAddressAck(_, _, _) => { CommandMetric::PaymentsCommandCreateAddressAck }
                    PaymentsCommand::ListAddresses(_, _) => { CommandMetric::PaymentsCommandListAddresses }
                    PaymentsCommand::AddRequestFees(_, _, _, _, _, _, _) => { CommandMetric::PaymentsCommandAddRequestFees }
                    PaymentsCommand::AddRequestFeesAck(_, _) => { CommandMetric::PaymentsCommandAddRequestFeesAck }
                    PaymentsCommand::ParseResponseWithFees(_, _, _) => { CommandMetric::PaymentsCommandParseResponseWithFees }
                    PaymentsCommand::ParseResponseWithFeesAck(_, _) => { CommandMetric::PaymentsCommandParseResponseWithFeesAck }
                    PaymentsCommand::BuildGetPaymentSourcesRequest(_, _, _, _, _) => { CommandMetric::PaymentsCommandBuildGetPaymentSourcesRequest }
                    PaymentsCommand::BuildGetPaymentSourcesRequestAck(_, _) => { CommandMetric::PaymentsCommandBuildGetPaymentSourcesRequestAck }
                    PaymentsCommand::ParseGetPaymentSourcesResponse(_, _, _) => { CommandMetric::PaymentsCommandParseGetPaymentSourcesResponse }
                    PaymentsCommand::ParseGetPaymentSourcesResponseAck(_, _) => { CommandMetric::PaymentsCommandParseGetPaymentSourcesResponseAck }
                    PaymentsCommand::BuildPaymentReq(_, _, _, _, _, _) => { CommandMetric::PaymentsCommandBuildPaymentReq }
                    PaymentsCommand::BuildPaymentReqAck(_, _) => { CommandMetric::PaymentsCommandBuildPaymentReqAck }
                    PaymentsCommand::ParsePaymentResponse(_, _, _) => { CommandMetric::PaymentsCommandParsePaymentResponse }
                    PaymentsCommand::ParsePaymentResponseAck(_, _) => { CommandMetric::PaymentsCommandParsePaymentResponseAck }
                    PaymentsCommand::AppendTxnAuthorAgreementAcceptanceToExtra(_, _, _, _, _, _, _) => { CommandMetric::PaymentsCommandAppendTxnAuthorAgreementAcceptanceToExtra }
                    PaymentsCommand::BuildMintReq(_, _, _, _, _) => { CommandMetric::PaymentsCommandBuildMintReq }
                    PaymentsCommand::BuildMintReqAck(_, _) => { CommandMetric::PaymentsCommandBuildMintReqAck }
                    PaymentsCommand::BuildSetTxnFeesReq(_, _, _, _, _) => { CommandMetric::PaymentsCommandBuildSetTxnFeesReq }
                    PaymentsCommand::BuildSetTxnFeesReqAck(_, _) => { CommandMetric::PaymentsCommandBuildSetTxnFeesReqAck }
                    PaymentsCommand::BuildGetTxnFeesReq(_, _, _, _) => { CommandMetric::PaymentsCommandBuildGetTxnFeesReq }
                    PaymentsCommand::BuildGetTxnFeesReqAck(_, _) => { CommandMetric::PaymentsCommandBuildGetTxnFeesReqAck }
                    PaymentsCommand::ParseGetTxnFeesResponse(_, _, _) => { CommandMetric::PaymentsCommandParseGetTxnFeesResponse }
                    PaymentsCommand::ParseGetTxnFeesResponseAck(_, _) => { CommandMetric::PaymentsCommandParseGetTxnFeesResponseAck }
                    PaymentsCommand::BuildVerifyPaymentReq(_, _, _, _) => { CommandMetric::PaymentsCommandBuildVerifyPaymentReq }
                    PaymentsCommand::BuildVerifyPaymentReqAck(_, _) => { CommandMetric::PaymentsCommandBuildVerifyPaymentReqAck }
                    PaymentsCommand::ParseVerifyPaymentResponse(_, _, _) => { CommandMetric::PaymentsCommandParseVerifyPaymentResponse }
                    PaymentsCommand::ParseVerifyPaymentResponseAck(_, _) => { CommandMetric::PaymentsCommandParseVerifyPaymentResponseAck }
                    PaymentsCommand::GetRequestInfo(_, _, _, _) => { CommandMetric::PaymentsCommandGetRequestInfo }
                    PaymentsCommand::SignWithAddressReq(_, _, _, _) => { CommandMetric::PaymentsCommandSignWithAddressReq }
                    PaymentsCommand::SignWithAddressAck(_, _) => { CommandMetric::PaymentsCommandSignWithAddressAck }
                    PaymentsCommand::VerifyWithAddressReq(_, _, _, _) => { CommandMetric::PaymentsCommandVerifyWithAddressReq }
                    PaymentsCommand::VerifyWithAddressAck(_, _) => { CommandMetric::PaymentsCommandVerifyWithAddressAck }
                }
            }
            Command::Cache(cmd) => {
                match cmd {
                    CacheCommand::GetSchema(_, _, _, _, _, _) => { CommandMetric::CacheCommandGetSchema }
                    CacheCommand::GetSchemaContinue(_, _, _, _) => { CommandMetric::CacheCommandGetSchemaContinue }
                    CacheCommand::GetCredDef(_, _, _, _, _, _) => { CommandMetric::CacheCommandGetCredDef }
                    CacheCommand::GetCredDefContinue(_, _, _, _) => { CommandMetric::CacheCommandGetCredDefContinue }
                    CacheCommand::PurgeSchemaCache(_, _, _) => { CommandMetric::CacheCommandPurgeSchemaCache }
                    CacheCommand::PurgeCredDefCache(_, _, _) => { CommandMetric::CacheCommandPurgeCredDefCache }
                }
            }
            Command::Metrics(cmd) => {
                match cmd { MetricsCommand::CollectMetrics(_) => { CommandMetric::MetricsCommandCollectMetrics } }
            }
        }
    }
}


#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive, VariantCount)]
#[repr(usize)]
pub enum CommandMetric {
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
    LedgerCommandBuildGetFrozenLedgersRequest,
    LedgerCommandBuildLedgersFreezeRequest,
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