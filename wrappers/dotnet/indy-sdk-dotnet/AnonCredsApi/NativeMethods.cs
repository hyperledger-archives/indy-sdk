using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.AnonCredsApi
{
    internal static class NativeMethods
    {
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_schema(int command_handle, string issuer_did, string name, string version, string attrs, IssuerCreateSchemaCompletedDelegate cb);
        internal delegate void IssuerCreateSchemaCompletedDelegate(int xcommand_handle, int err, string schema_id, string schema_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_and_store_credential_def(int command_handle, int wallet_handle, string issuer_did, string schema_json, string tag, string signature_type, string config_json, IssuerCreateAndStoreCredentialDefCompletedDelegate cb);
        internal delegate void IssuerCreateAndStoreCredentialDefCompletedDelegate(int xcommand_handle, int err, string cred_def_id, string cred_def_json);
        
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_rotate_credential_def_start(int command_handle, int wallet_handle, string cred_def_id, string config_json, IssuerRotateCredentialDefStartCompletedDelegate cb);
        internal delegate void IssuerRotateCredentialDefStartCompletedDelegate(int xcommand_handle, int err, string cred_def_json);
        
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_rotate_credential_def_apply(int command_handle, int wallet_handle, string cred_def_id, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_and_store_revoc_reg(int command_handle, int wallet_handle, string issuer_did, string revoc_def_type, string tag, string cred_def_id, string config_json, int tails_writer_handle, IssuerCreateAndStoreRevocRegCompletedDelegate cb);
        internal delegate void IssuerCreateAndStoreRevocRegCompletedDelegate(int xcommand_handle, int err, string revoc_reg_id, string revoc_reg_def_json, string revoc_reg_entry_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_credential_offer(int command_handle, int wallet_handle, string cred_def_id, IssuerCreateCredentialOfferCompletedDelegate cb);
        internal delegate void IssuerCreateCredentialOfferCompletedDelegate(int xcommand_handle, int err, string cred_offer_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_credential(int command_handle, int wallet_handle, string cred_offer_json, string cred_req_json, string cred_values_json, string rev_reg_id, int blob_storage_reader_handle, IssuerCreateCredentialCompletedDelegate cb);
        internal delegate void IssuerCreateCredentialCompletedDelegate(int xcommand_handle, int err, string cred_json, string cred_revoc_id, string revoc_reg_delta_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_revoke_credential(int command_handle, int wallet_handle, int blob_storage_reader_cfg_handle, string rev_reg_id, string cred_revoc_id, IssuerRevokeCredentialCompletedDelegate cb);
        internal delegate void IssuerRevokeCredentialCompletedDelegate(int xcommand_handle, int err, string revoc_reg_delta_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_merge_revocation_registry_deltas(int command_handle, string rev_reg_delta_json, string other_rev_reg_delta_json, IssuerMergeRevocationRegistryDeltasCompletedDelegate cb);
        internal delegate void IssuerMergeRevocationRegistryDeltasCompletedDelegate(int xcommand_handle, int err, string merged_rev_reg_delta);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_master_secret(int command_handle, int wallet_handle, string master_secret_id, ProverCreateMasterSecretCompletedDelegate cb);
        internal delegate void ProverCreateMasterSecretCompletedDelegate(int xcommand_handle, int err, string out_master_secret_id);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_credential_req(int command_handle, int wallet_handle, string prover_did, string cred_offer_json, string cred_def_json, string master_secret_id, ProverCreateCredentialReqCompletedDelegate cb);
        internal delegate void ProverCreateCredentialReqCompletedDelegate(int xcommand_handle, int err, string cred_req_json, string cred_req_metadata);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_store_credential(int command_handle, int wallet_handle, string cred_id, string cred_req_metadata_json, string cred_json, string cred_def_json, string rev_reg_def_json, ProverStoreCredentialCompletedDelegate cb);
        internal delegate void ProverStoreCredentialCompletedDelegate(int xcommand_handle, int err, string out_cred_id);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_credential(int command_handle, int wallet_handle, string cred_id, ProverGetCredentialCompletedDelegate cb);
        internal delegate void ProverGetCredentialCompletedDelegate(int xcommand_handle, int err, string credential_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_credentials(int command_handle, int wallet_handle, string filter_json, ProverGetCredentialsCompletedDelegate cb);
        internal delegate void ProverGetCredentialsCompletedDelegate(int xcommand_handle, int err, string matched_credentials_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_delete_credential(int command_handle, int wallet_handle, string cred_id, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_search_credentials(int command_handle, int wallet_handle, string query_json, ProverSearchCredentialsCompletedDelegate cb);
        internal delegate void ProverSearchCredentialsCompletedDelegate(int xcommand_handle, int err, int search_handle, int total_count);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_fetch_credentials(int command_handle, int search_handle, int count, ProverFetchCredentialsCompletedDelegate cb);
        internal delegate void ProverFetchCredentialsCompletedDelegate(int xcommand_handle, int err, string credentials_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_close_credentials_search(int command_handle, int search_handle, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_credentials_for_proof_req(int command_handle, int wallet_handle, string proof_request_json, ProverGetCredentialsForProofCompletedDelegate cb);
        internal delegate void ProverGetCredentialsForProofCompletedDelegate(int xcommand_handle, int err, string credentials_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_search_credentials_for_proof_req(int command_handle, int wallet_handle, string proof_request_json, string extra_query_json, ProverSearchCredentialsForProofReqCompletedDelegate cb);
        internal delegate void ProverSearchCredentialsForProofReqCompletedDelegate(int xcommand_handle, int err, int search_handle);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_fetch_credentials_for_proof_req(int command_handle, int search_handle, string item_referent, int count, ProverFetchCredentialsForProofReqCompletedDelegate cb);
        internal delegate void ProverFetchCredentialsForProofReqCompletedDelegate(int xcommand_handle, int err, string credentials_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_close_credentials_search_for_proof_req(int command_handle, int search_handle, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_proof(int command_handle, int wallet_handle, string proof_req_json, string requested_credentials_json, string master_secret_id, string schemas_json, string credential_defs_json, string rev_states_json, ProverCreateProofCompletedDelegate cb);
        internal delegate void ProverCreateProofCompletedDelegate(int xcommand_handle, int err, string proof_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_verifier_verify_proof(int command_handle, string proof_request_json, string proof_json, string schemas_json, string credential_defs_json, string rev_reg_defs_json, string rev_regs_json, VerifierVerifyProofCompletedDelegate cb);
        internal delegate void VerifierVerifyProofCompletedDelegate(int xcommand_handle, int err, bool valid);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_revocation_state(int command_handle, int blob_storage_reader_handle, string rev_reg_def_json, string rev_reg_delta_json, long timestamp, string cred_rev_id, CreateRevocationStateCompletedDelegate cb);
        internal delegate void CreateRevocationStateCompletedDelegate(int xcommand_handle, int err, string rev_state_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_update_revocation_state(int command_handle, int blob_storage_reader_handle, string rev_state_json, string rev_reg_def_json, string rev_reg_delta_json, long timestamp, string cred_rev_id, UpdateRevocationStateCompletedDelegate cb);
        internal delegate void UpdateRevocationStateCompletedDelegate(int xcommand_handle, int err, string updated_rev_state_json);
    
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_generate_nonce(int command_handle, GenerateNonceCompletedDelegate cb);
        internal delegate void GenerateNonceCompletedDelegate(int xcommand_handle, int err, string nonce);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_to_unqualified(int command_handle, string entity, ToUnqualifiedCompletedDelegate cb);
        internal delegate void ToUnqualifiedCompletedDelegate(int xcommand_handle, int err, string res);
    }
}
