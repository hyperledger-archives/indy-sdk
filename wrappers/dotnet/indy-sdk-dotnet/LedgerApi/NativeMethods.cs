﻿using System;
using System.Runtime.InteropServices;

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

        /// <summary>
        /// Builds a NYM request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="verkey">verification key</param>
        /// <param name="alias">Alias.</param>
        /// <param name="role">Role of a user NYM record</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_nym_request(int command_handle, string submitter_did, string target_did, string verkey, string alias, string role, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds an ATTRIB request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="hash">Hash of attribute data</param>
        /// <param name="raw">represented as json, where key is attribute name and value is it's value</param>
        /// <param name="enc">Encrypted attribute data</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_attrib_request(int command_handle, string submitter_did, string target_did, string hash, string raw, string enc, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a GET_ATTRIB request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="raw"> name (attribute name)</param>
        /// <param name="hash"></param>
        /// <param name="enc"></param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_attrib_request(int command_handle, string submitter_did, string target_did, string raw, string hash, string enc, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a GET_NYM request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_nym_request(int command_handle, string submitter_did, string target_did, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a SCHEMA request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="data"> name, version, type, attr_names (ip, port, keys)</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_schema_request(int command_handle, string submitter_did, string data, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Indies the build get schema request.
        /// </summary>
        /// <returns>The build get schema request.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="submitter_did">Submitter did.</param>
        /// <param name="id">Identifier.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_schema_request(int command_handle, string submitter_did, string id, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Indies the parse get schema response.
        /// </summary>
        /// <returns>The parse get schema response.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="get_schema_response">Get schema response.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_parse_get_schema_response(int command_handle, string get_schema_response, ParseResponseCompletedDelegate cb);

        internal delegate void ParseResponseCompletedDelegate(int xcommand_handle, int err, string object_id, string object_json);

        internal delegate void ParseRegistryResponseCompletedDelegate(int xcommand_handle, int err, string id, string object_json, ulong timestamp);

        /// <summary>
        /// Indies the build cred def request.
        /// </summary>
        /// <returns>The build cred def request.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="submitter_did">Submitter did.</param>
        /// <param name="data">Data.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_cred_def_request(int command_handle, string submitter_did, string data, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Indies the build get cred def request.
        /// </summary>
        /// <returns>The build get cred def request.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="submitter_did">Submitter did.</param>
        /// <param name="id">Identifier.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_cred_def_request(int command_handle, string submitter_did, string id, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Indies the parse get cred def response.
        /// </summary>
        /// <returns>The parse get cred def response.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="get_cred_def_response">Get cred def response.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_parse_get_cred_def_response(int command_handle, string get_cred_def_response, ParseResponseCompletedDelegate cb);

        /// <summary>
        /// Builds a NODE request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="data">id of a target NYM record</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_node_request(int command_handle, string submitter_did, string target_did, string data, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a GET_VALIDATOR_INFO request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_validator_info_request(int command_handle, string submitter_did, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a GET_TXN request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="ledger_type">(Optional) type of the ledger the requested transaction belongs to:
        ///     DOMAIN - used default,
        ///     POOL,
        ///     CONFIG
        ///     any number</param>
        /// <param name="seq_no">seq_no of transaction in ledger</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_txn_request(int command_handle, string submitter_did, string ledger_type, int seq_no, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a POOL_CONFIG request.
        /// </summary>
        /// <returns>Request result as json.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="writes">If set to <c>true</c> writes.</param>
        /// <param name="force">If set to <c>true</c> force.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_pool_config_request(int command_handle, string submitter_did, bool writes, bool force, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Action that pool has to do after received transaction.
        /// </summary>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="action">Action that pool has to do after received transaction.</param>
        /// <param name="datetime">Restart time in datetime format. Skip to restart as early as possible.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_pool_restart_request(int command_handle, string submitter_did, string action, string datetime, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a POOL_UPGRADE request.
        /// </summary>
        /// <returns>Request result as json.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="submitter_did">Submitter did.</param>
        /// <param name="name">Name.</param>
        /// <param name="version">Version.</param>
        /// <param name="action">Either start or cancel.</param>
        /// <param name="sha256">Sha256.</param>
        /// <param name="timeout">Timeout.</param>
        /// <param name="schedule">Schedule.</param>
        /// <param name="justification">Justification.</param>
        /// <param name="reinstall">If set to <c>true</c> reinstall.</param>
        /// <param name="force">If set to <c>true</c> force.</param>
        /// <param name="package">Package to be upgraded.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
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
    }
}
