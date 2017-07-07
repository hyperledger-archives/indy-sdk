using System;
using System.Runtime.InteropServices;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Lightest weight import of libsovrin library functions exposed as-is.
    /// </summary>
    public static class LibSovrin
    {
        /// <summary>
        /// Delegate for use with callbacks that only include the result of the callback.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        public delegate void ResultOnlyDelegate(IntPtr xcommand_handle, int err);
        
        /// <summary>
        /// Delegate for use with callbacks that return a handle to another object.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="handle">The handle to the object.</param>
        public delegate void ResultWithHandleDelegate(IntPtr xcommand_handle, int err, IntPtr handle);

        // pool.rs

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_create_pool_ledger_config(IntPtr command_handle, string config_name, string config, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_delete_pool_ledger_config(IntPtr command_handle, string config_name, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_open_pool_ledger(IntPtr command_handle, string config_name, string config, ResultWithHandleDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_refresh_pool_ledger(IntPtr command_handle, IntPtr handle, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_close_pool_ledger(IntPtr command_handle, IntPtr handle, ResultOnlyDelegate cb);

        // wallet.rs

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_create_wallet(IntPtr command_handle, string pool_name, string name, string xtype, string config, string credentials, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_open_wallet(IntPtr command_handle, string name, string runtime_config, string credentials, ResultWithHandleDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_close_wallet(IntPtr command_handle, IntPtr handle, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_delete_wallet(IntPtr command_handle, string name, string credentials, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_wallet_set_seq_no_for_value(IntPtr command_handle, IntPtr wallet_handle, string wallet_key, ResultOnlyDelegate cb);

        // ledger.rs

        public delegate void SubmitRequestResultDelegate(IntPtr xcommand_handle, int err, string request_result_json);
        public delegate void BuildRequestResultDelegate(IntPtr xcommand_handle, int err, string request_json);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_sign_and_submit_request(IntPtr command_handle, IntPtr pool_handle, IntPtr wallet_handle, string submitter_did, string request_json, SubmitRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_submit_request(IntPtr command_handle, IntPtr pool_handle, string request_json, SubmitRequestResultDelegate cb);
        
        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_get_ddo_request(IntPtr command_handle, string submitter_did, string target_did, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_nym_request(IntPtr command_handle, string submitter_did, string target_did, string verkey, string alias, string role, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_attrib_request(IntPtr command_handle, string submitter_did, string target_did, string hash, string raw, string enc, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_get_attrib_request(IntPtr command_handle, string submitter_did, string target_did, string data, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_get_nym_request(IntPtr command_handle, string submitter_did, string target_did, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_schema_request(IntPtr command_handle, string submitter_did, string data, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_get_schema_request(IntPtr command_handle, string submitter_did, string data, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_claim_def_txn(IntPtr command_handle, string submitter_did, string xref, string data, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_get_claim_def_txn(IntPtr command_handle, string submitter_did, string xref, BuildRequestResultDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_build_node_request(IntPtr command_handle, string submitter_did, string target_did, string data, BuildRequestResultDelegate cb);

        // signus.rs

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_create_and_store_my_did(IntPtr command_handle, IntPtr wallet_handle, string did_json, CreateAndStoreMyDidResultDelegate cb);
        public delegate void CreateAndStoreMyDidResultDelegate(IntPtr xcommand_handle, int err, string did, string verkey, string pk);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_replace_keys(IntPtr command_handle, IntPtr wallet_handle, string did, string identity_json, ReplaceKeysResultDelegate cb);
        public delegate void ReplaceKeysResultDelegate(IntPtr xcommand_handle, int err, string verkey, string pk);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_store_their_did(IntPtr command_handle, IntPtr wallet_handle, string identity_json, ResultOnlyDelegate cb);
        
        [DllImport("Sovrin.dll")]
        public static extern int sovrin_sign(IntPtr command_handle, IntPtr wallet_handle, string did, string msg, SignResultDelegate cb);
        public delegate void SignResultDelegate(IntPtr xcommand_handle, int err, string signature);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_verify_signature(IntPtr command_handle, IntPtr wallet_handle, IntPtr pool_handle, string did, string signed_msg, VerifySignatureResultDelegate cb);
        public delegate void VerifySignatureResultDelegate(IntPtr xcommand_handle, int err, bool valid);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_encrypt(IntPtr command_handle, IntPtr wallet_handle, string did, string msg, EncryptResultDelegate cb);
        public delegate void EncryptResultDelegate(IntPtr xcommand_handle, int err, string encrypted_msg, string nonce);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_decrypt(IntPtr command_handle, IntPtr wallet_handle, string did, string encrypted_msg, DecryptResultDelegate cb);
        public delegate void DecryptResultDelegate(IntPtr xcommand_handle, int err, string decrypted_msg);

        // anoncreds.rs

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_issuer_create_and_store_claim_def(IntPtr command_handle, IntPtr wallet_handle, string schema_json, string signature_type, bool create_non_revoc, IssuerCreateAndStoreClaimDefResultDelegate cb);
        public delegate void IssuerCreateAndStoreClaimDefResultDelegate(IntPtr xcommand_handle, int err, string claim_def_json, string claim_def_uuid);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_issuer_create_and_store_revoc_reg(IntPtr command_handle, IntPtr wallet_handle, int claim_def_seq_no, int max_claim_num, IssuerCreateAndStoreClaimRevocRegResultDelegate cb);
        public delegate void IssuerCreateAndStoreClaimRevocRegResultDelegate(IntPtr xcommand_handle, int err, string claim_def_json, string claim_def_uuid);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_issuer_create_claim(IntPtr command_handle, IntPtr wallet_handle, string claim_req_json, string claim_json, int revoc_reg_seq_no, int user_revoc_index, IssuerCreateClaimResultDelegate cb);
        public delegate void IssuerCreateClaimResultDelegate(IntPtr xcommand_handle, int err, string revoc_reg_update_json, string xclaim_json);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_issuer_revoke_claim(IntPtr command_handle, IntPtr wallet_handle, int claim_def_seq_no, int revoc_reg_seq_no, int user_revoc_index, IssuerRevokeClaimResultDelegate cb);
        public delegate void IssuerRevokeClaimResultDelegate(IntPtr xcommand_handle, int err, string revoc_reg_update_json);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_prover_store_claim_offer(IntPtr command_handle, IntPtr wallet_handle, string claim_offer_json, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_prover_get_claim_offers(IntPtr command_handle, IntPtr wallet_handle, string filter_json, ProverGetClaimOffersResultDelegate cb);
        public delegate void ProverGetClaimOffersResultDelegate(IntPtr xcommand_handle, int err, string claim_offers_json);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_prover_create_master_secret(IntPtr command_handle, IntPtr wallet_handle, string master_secret_name, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_prover_create_and_store_claim_req(IntPtr command_handle, IntPtr wallet_handle, string prover_did, string claim_offer_json, string claim_def_json, string master_secret_name, ProverCreateAndStoreClaimReqResultDelegate cb);
        public delegate void ProverCreateAndStoreClaimReqResultDelegate(IntPtr xcommand_handle, int err, string claim_req_json);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_prover_store_claim(IntPtr command_handle, IntPtr wallet_handle, string claims_json, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_prover_get_claims(IntPtr command_handle, IntPtr wallet_handle, string filter_json, ProverGetClaimsResultDelegate cb);
        public delegate void ProverGetClaimsResultDelegate(IntPtr xcommand_handle, int err, string claims_json);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_prover_get_claims_for_proof_req(IntPtr command_handle, IntPtr wallet_handle, string proof_request_json, ProverGetClaimsForProofResultDelegate cb);
        public delegate void ProverGetClaimsForProofResultDelegate(IntPtr xcommand_handle, int err, string claims_json);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_prover_create_proof(IntPtr command_handle, IntPtr wallet_handle, string proof_req_json, string requested_claims_json, string schemas_json, string master_secret_name, string claim_defs_json, string revoc_regs_json, ProverCreateProofResultDelegate cb);
        public delegate void ProverCreateProofResultDelegate(IntPtr xcommand_handle, int err, string proof_json);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_verifier_verify_proof(IntPtr command_handle, IntPtr wallet_handle, string proof_request_json, string proof_json, string schemas_json, string claim_defs_jsons, string revoc_regs_json, VerifierVerifyProofResultDelegate cb);
        public delegate void VerifierVerifyProofResultDelegate(IntPtr xcommand_handle, int err, bool valid);

        // agent.rs
        public delegate void AgentMessageReceivedDelegate(IntPtr xcommand_handle, int err, string message);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_agent_connect(IntPtr command_handle, IntPtr pool_handle, IntPtr wallet_handle, string sender_did, string receiver_did, ResultWithHandleDelegate connection_cb, AgentMessageReceivedDelegate message_cb);
        
        [DllImport("Sovrin.dll")]
        public static extern int sovrin_agent_listen(IntPtr command_handle, string endpoint, ResultWithHandleDelegate listener_cb, AgentListenConnectionResultDelegate connection_cb, AgentMessageReceivedDelegate message_cb);
        public delegate void AgentListenConnectionResultDelegate(IntPtr xcommand_handle, int err, IntPtr connection_handle, string sender_did, string reciever_did);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_agent_add_identity(IntPtr command_handle, IntPtr listener_handle, IntPtr pool_handle, IntPtr wallet_handle, string did, ResultOnlyDelegate add_identity_cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_agent_remove_identity(IntPtr command_handle, IntPtr listener_handle, IntPtr wallet_handle, string did, ResultOnlyDelegate rm_identity_cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_agent_send(IntPtr command_handle, IntPtr connection_handle, string message, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_agent_close_connection(IntPtr command_handle, IntPtr connection_handle, ResultOnlyDelegate cb);

        [DllImport("Sovrin.dll")]
        public static extern int sovrin_agent_close_listener(IntPtr command_handle, IntPtr listener_handle, ResultOnlyDelegate cb);
    }
}
