using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.LedgerApi
{
    internal static class NativeMethods
    {
        /// <summary>
        /// Delegate for callbacks used by functions that submit requests to the ledger.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="request_result_json">The result data.</param>
        internal delegate void SubmitRequestCompletedDelegate(int xcommand_handle, int err, string request_result_json);

        /// <summary>
        /// Delegate for callbacks used by functions that build requests destined for the ledger.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="request_json">The request that can be signed and submitted to the ledger.</param>
        internal delegate void BuildRequestCompletedDelegate(int xcommand_handle, int err, string request_json);

        /// <summary>
        /// Signs and submits request message to validator pool.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="pool_handle">pool handle (created by open_pool_ledger).</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="request_json">Request data json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_sign_and_submit_request(int command_handle, int pool_handle, int wallet_handle, string submitter_did, string request_json, SubmitRequestCompletedDelegate cb);

        /// <summary>
        /// Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="pool_handle">pool handle (created by open_pool_ledger).</param>
        /// <param name="request_json">Request data json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_submit_request(int command_handle, int pool_handle, string request_json, SubmitRequestCompletedDelegate cb);

        /// <summary>
        /// Send action to particular nodes of validator pool.
        ///
        /// The list of requests can be send:
        ///     POOL_RESTART
        ///     GET_VALIDATOR_INFO
        /// </summary>
        /// <returns>The submit action.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="pool_handle">Pool handle.</param>
        /// <param name="request_json">Request json.</param>
        /// <param name="nodes">Nodes.</param>
        /// <param name="timeout">Timeout.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_submit_action(int command_handle, int pool_handle, string request_json, string nodes, int timeout, SubmitRequestCompletedDelegate cb);


        /// <summary>
        /// Signs a request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle.</param>
        /// <param name="submitter_did">The DID of the submitter.</param>
        /// <param name="request_json">The request to sign.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_sign_request(int command_handle, int wallet_handle, string submitter_did, string request_json, SignRequestCompletedDelegate cb);

        /// <summary>
        /// Multi signs request message.
        ///
        /// Adds submitter information to passed request json, signs it with submitter
        /// sign key (see wallet_sign).
        /// </summary>
        /// <returns>The multi sign request.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="submitter_did">Submitter did.</param>
        /// <param name="request_json">Request json.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_multi_sign_request(int command_handle, int wallet_handle, string submitter_did, string request_json, SignRequestCompletedDelegate cb);


        /// <summary>
        /// Delegate to be used on completion of calls to indy_sign_request.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="signed_request_json">The signed request data.</param>
        internal delegate void SignRequestCompletedDelegate(int xcommand_handle, int err, string signed_request_json);

        /// <summary>
        /// Builds a request to get a DDO.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_ddo_request(int command_handle, string submitter_did, string target_did, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_nym_request(int command_handle, string submitter_did, string target_did, string verkey, string alias, string role, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_attrib_request(int command_handle, string submitter_did, string target_did, string hash, string raw, string enc, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_attrib_request(int command_handle, string submitter_did, string target_did, string raw, string hash, string enc, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_nym_request(int command_handle, string submitter_did, string target_did, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_schema_request(int command_handle, string submitter_did, string data, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_schema_request(int command_handle, string submitter_did, string id, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_parse_get_schema_response(int command_handle, string get_schema_response, ParseResponseCompletedDelegate cb);

        internal delegate void ParseResponseCompletedDelegate(int xcommand_handle, int err, string object_id, string object_json);

        internal delegate void ParseRegistryResponseCompletedDelegate(int xcommand_handle, int err, string id, string object_json, ulong timestamp);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_cred_def_request(int command_handle, string submitter_did, string data, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_cred_def_request(int command_handle, string submitter_did, string id, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_parse_get_cred_def_response(int command_handle, string get_cred_def_response, ParseResponseCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_node_request(int command_handle, string submitter_did, string target_did, string data, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_validator_info_request(int command_handle, string submitter_did, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_txn_request(int command_handle, string submitter_did, string ledger_type, int seq_no, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_pool_config_request(int command_handle, string submitter_did, bool writes, bool force, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_pool_restart_request(int command_handle, string submitter_did, string action, string datetime, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_pool_upgrade_request(int command_handle, string submitter_did, string name, string version, string action, string sha256, int timeout, string schedule, string justification, bool reinstall, bool force, string package, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_revoc_reg_def_request(int command_handle, string submitter_did, string data, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_revoc_reg_def_request(int command_handle, string submitter_did, string id, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_parse_get_revoc_reg_def_response(int command_handle, string get_revoc_reg_def_response, ParseResponseCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_revoc_reg_entry_request(int command_handle, string submitter_did, string revoc_reg_def_id, string rev_def_type, string value, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_revoc_reg_request(int command_handle, string submitter_did, string revoc_reg_def_id, long timestamp, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_parse_get_revoc_reg_response(int command_handle, string get_revoc_reg_response, ParseRegistryResponseCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_revoc_reg_delta_request(int command_handle, string submitter_did, string revoc_reg_def_id, long from, long to, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_parse_get_revoc_reg_delta_response(int command_handle, string get_revoc_reg_delta_response, ParseRegistryResponseCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_response_metadata(int command_handle, string response, GetResponseMetadataCompletedDelegate cb);
        internal delegate void GetResponseMetadataCompletedDelegate(int xcommand_handle, int err, string response_metadata);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_auth_rule_request(int command_handle, string submitter_did, string txn_type, string action, string field, string old_value, string new_value, string constraint, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_auth_rules_request(int command_handle, string submitter_did, string rules, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_auth_rule_request(int command_handle, string submitter_did, string txn_type, string action, string field, string old_value, string new_value, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_txn_author_agreement_request(int command_handle, string submitter_did, string text, string version, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_txn_author_agreement_request(int command_handle, string submitter_did, string data, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_acceptance_mechanisms_request(int command_handle, string submitter_did, string aml, string version, string aml_context, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_acceptance_mechanisms_request(int command_handle, string submitter_did, long timestamp, string version, BuildRequestCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_append_txn_author_agreement_acceptance_to_request(int command_handle, string request_json, string text, string version, string taa_digest, string mechanism, ulong time, BuildRequestCompletedDelegate cb);
    }
}
