using System;
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
        internal static extern int indy_sign_and_submit_request(int command_handle, IntPtr pool_handle, IntPtr wallet_handle, string submitter_did, string request_json, SubmitRequestCompletedDelegate cb);

        /// <summary>
        /// Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="pool_handle">pool handle (created by open_pool_ledger).</param>
        /// <param name="request_json">Request data json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_submit_request(int command_handle, IntPtr pool_handle, string request_json, SubmitRequestCompletedDelegate cb);

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
        internal static extern int indy_sign_request(int command_handle, IntPtr wallet_handle, string submitter_did, string request_json, SignRequestCompletedDelegate cb);

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
        /// Builds a GET_SCHEMA request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="dest">Id of Identity stored in secured Wallet.</param>
        /// <param name="data">name, version</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_schema_request(int command_handle, string submitter_did, string dest, string data, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds an CLAIM_DEF request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="xref">Seq. number of schema</param>
        /// <param name="signature_type">signature type (only CL supported now)</param>
        /// <param name="data">components of a key in json: N, R, S, Z</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_claim_def_txn(int command_handle, string submitter_did, int xref, string signature_type, string data, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a GET_CLAIM_DEF request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="xref">Seq. number of schema</param>
        /// <param name="signature_type">signature type (only CL supported now)</param>
        /// <param name="origin">issuer did</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_claim_def_txn(int command_handle, string submitter_did, int xref, string signature_type, string origin, BuildRequestCompletedDelegate cb);

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
        /// Builds a GET_TXN request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="data">seq_no of transaction in ledger</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_txn_request(int command_handle, string submitter_did, int data, BuildRequestCompletedDelegate cb);

        /// <summary>
        /// Builds a POOL_CONFIG request.
        /// </summary>
        /// <returns>Request result as json.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="writes">If set to <c>true</c> writes.</param>
        /// <param name="force">If set to <c>true</c> force.</param>
        /// <param name="cb">Callback that takes command result as parameter..</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_pool_config_request(int command_handle, string submitter_did, bool writes, bool force, BuildRequestCompletedDelegate cb);

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
        /// <param name="cb">Callback that takes command result as parameter..</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_pool_upgrade_request(int command_handle, string submitter_did, string name, string version, string action, string sha256, int timeout, string schedule, string justification, bool reinstall, bool force, BuildRequestCompletedDelegate cb);

    }
}
