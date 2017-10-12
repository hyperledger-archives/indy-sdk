using System;
using System.Runtime.InteropServices;

namespace Hyperledger.Indy
{
    

    /// <summary>
    /// PInvoke import of C-Callable SDK library functions and associated delegates.
    /// </summary>
    internal static class IndyNativeMethods
    {
        private const string NATIVE_LIB_NAME = "indy";

        /// <summary>
        /// Delegate for callbacks that only include the success or failure of command execution.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        internal delegate void NoValueDelegate(int xcommand_handle, int err);
        

        // pool.rs

        /// <summary>
        /// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="config_name">Name of the pool ledger configuration.</param>
        /// <param name="config">Pool configuration json. if NULL, then default config will be used.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi,  BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_pool_ledger_config(int command_handle, string config_name, string config, NoValueDelegate cb);

        /// <summary>
        /// Deletes created pool ledger configuration.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="config_name">Name of the pool ledger configuration to delete.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_delete_pool_ledger_config(int command_handle, string config_name, NoValueDelegate cb);

        /// <summary>
        /// Opens pool ledger and performs connecting to pool nodes.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="config_name">Name of the pool ledger configuration.</param>
        /// <param name="config">Runtime pool configuration json. If null the default configuration will be used.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_open_pool_ledger(int command_handle, string config_name, string config, OpenPoolLedgerResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_open_pool_ledger function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="pool_handle">The handle for the opened pool.</param>
        internal delegate void OpenPoolLedgerResultDelegate(int xcommand_handle, int err, IntPtr pool_handle);

        /// <summary>
        /// Refreshes a local copy of a pool ledger and updates pool nodes connections.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="handle">Pool handle returned by indy_open_pool_ledger</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_refresh_pool_ledger(int command_handle, IntPtr handle, NoValueDelegate cb);

        /// <summary>
        /// Closes opened pool ledger, opened nodes connections and frees allocated resources.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="handle">pool handle returned by indy_open_pool_ledger.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_close_pool_ledger(int command_handle, IntPtr handle, NoValueDelegate cb);

        // wallet.rs

        /// <summary>
        /// Registers custom wallet implementation.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="xtype">Wallet type name.</param>
        /// <param name="create">WalletType create operation handler</param>
        /// <param name="open">WalletType open operation handler</param>
        /// <param name="set">Wallet set operation handler</param>
        /// <param name="get">Wallet get operation handler</param>
        /// <param name="get_not_expired">Wallet get_not_expired operation handler</param>
        /// <param name="list">Wallet list operation handler</param>
        /// <param name="close">Wallet close operation handler</param>
        /// <param name="delete">WalletType delete operation handler</param>
        /// <param name="free">Handler that allows to de-allocate strings allocated in caller code</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_register_wallet_type(int command_handle, string xtype, WalletTypeCreateDelegate create, WalletTypeOpenDelegate open, WalletTypeSetDelegate set, WalletTypeGetDelegate get, WalletTypeGetNotExpiredDelegate get_not_expired, WalletTypeListDelegate list, WalletTypeCloseDelegate close, WalletTypeDeleteDelegate delete, WalletTypeFreeDelegate free, NoValueDelegate cb);

        /// <summary>
        /// Delegate for the function called back to when a wallet of a custom type is created.
        /// </summary>
        /// <param name="name">The name of the wallet.</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials for the wallet.</param>
        internal delegate ErrorCode WalletTypeCreateDelegate(string name, string config, string credentials);

        /// <summary>
        /// Delegate for the function called back to when a wallet of a custom type is opened.
        /// </summary>
        /// <param name="name">The name of the wallet to open.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="runtime_config">The runtime configuration for the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <param name="handle">A handle to use when tracking the wallet instance.</param>
        internal delegate ErrorCode WalletTypeOpenDelegate(string name, string config, string runtime_config, string credentials, ref int handle);

        /// <summary>
        /// Delegate for the function called back to when value is set on a wallet of a custom type.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        /// <param name="key">The key of the value to set.</param>
        /// <param name="value">The value to set.</param>
        internal delegate ErrorCode WalletTypeSetDelegate(int handle, string key, string value);

        /// <summary>
        /// Delegate for the function called back to when value is requested from a wallet of a custom type.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value_ptr">The pointer to the value associated with the key.</param>
        internal delegate ErrorCode WalletTypeGetDelegate(int handle, string key, ref IntPtr value_ptr);

        /// <summary>
        /// Delegate for the function called back to when an unexpired value is requested from a wallet of a custom type.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value_ptr">The pointer to the value associated with the key.</param>
        internal delegate ErrorCode WalletTypeGetNotExpiredDelegate(int handle, string key, ref IntPtr value_ptr);

        /// <summary>
        /// Delegate for the function called back to when an list of values is requested from a wallet of a custom type.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        /// <param name="keyPrefix">The key prefix for the values requested.</param>
        /// <param name="values_json_ptr">The pointer to the values associated with the key prefix.</param>
        internal delegate ErrorCode WalletTypeListDelegate(int handle, string keyPrefix, ref IntPtr values_json_ptr);

        /// <summary>
        /// Delegate for the function called back to when a wallet of a custom type is closed.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        internal delegate ErrorCode WalletTypeCloseDelegate(int handle);

        /// <summary>
        /// Delegate for the function called back to when a wallet of a custom type is deleted.
        /// </summary>
        /// <param name="name">The name of the wallet being deleted</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        internal delegate ErrorCode WalletTypeDeleteDelegate(string name, string config, string credentials);

        /// <summary>
        /// Delegate for the function called back to when a value in a  wallet of a custom type is freed.
        /// </summary>
        /// <param name="handle">The handle of the wallet the action is being performed on.</param>
        /// <param name="value">A pointer to the value to be freed.</param>
        internal delegate ErrorCode WalletTypeFreeDelegate(int handle, IntPtr value);


        /// <summary>
        /// Creates a new secure wallet with the given unique name.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="pool_name">Name of the pool that corresponds to this wallet.</param>
        /// <param name="name">Name of the wallet.</param>
        /// <param name="xtype">Type of the wallet. Defaults to 'default'.</param>
        /// <param name="config">Wallet configuration json.</param>
        /// <param name="credentials">Wallet credentials json. </param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_wallet(int command_handle, string pool_name, string name, string xtype, string config, string credentials, NoValueDelegate cb);

        /// <summary>
        /// Opens the wallet with specific name.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="name">Name of the wallet.</param>
        /// <param name="runtime_config">Runtime wallet configuration json. if NULL, then default runtime_config will be used. </param>
        /// <param name="credentials">Wallet credentials json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <remarks>
        /// Wallet with corresponded name must be previously created with indy_create_wallet method.
        /// It is impossible to open wallet with the same name more than once.
        /// </remarks>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_open_wallet(int command_handle, string name, string runtime_config, string credentials, OpenWalletResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_open_wallet function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="wallet_handle">The handle for the opened wallet.</param>
        internal delegate void OpenWalletResultDelegate(int xcommand_handle, int err, IntPtr wallet_handle);

        /// <summary>
        /// Closes opened wallet and frees allocated resources.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="handle">wallet handle returned by indy_open_wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_close_wallet(int command_handle, IntPtr handle, NoValueDelegate cb);

        /// <summary>
        /// Deletes created wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="name">Name of the wallet to delete.</param>
        /// <param name="credentials">Wallet credentials json</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_delete_wallet(int command_handle, string name, string credentials, NoValueDelegate cb);

        // ledger.rs

        /// <summary>
        /// Delegate for callbacks used by functions that submit requests to the ledger.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="request_result_json">The result data.</param>
        internal delegate void SubmitRequestResultDelegate(int xcommand_handle, int err, string request_result_json);

        /// <summary>
        /// Delegate for callbacks used by functions that build requests destined for the ledger.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="request_json">The request that can be signed and submitted to the ledger.</param>
        internal delegate void BuildRequestResultDelegate(int xcommand_handle, int err, string request_json);

        

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
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_sign_and_submit_request(int command_handle, IntPtr pool_handle, IntPtr wallet_handle, string submitter_did, string request_json, SubmitRequestResultDelegate cb);

        /// <summary>
        /// Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="pool_handle">pool handle (created by open_pool_ledger).</param>
        /// <param name="request_json">Request data json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_submit_request(int command_handle, IntPtr pool_handle, string request_json, SubmitRequestResultDelegate cb);
        
        /// <summary>
        /// Signs a request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle.</param>
        /// <param name="submitter_did">The DID of the submitter.</param>
        /// <param name="request_json">The request to sign.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_sign_request(int command_handle, IntPtr wallet_handle, string submitter_did, string request_json, SignRequestResultDelegate cb);

        /// <summary>
        /// Delegate for callbacks used by functions that sign requests.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="signed_request_json">The signed request data.</param>
        internal delegate void SignRequestResultDelegate(int xcommand_handle, int err, string signed_request_json);
        
        /// <summary>
        /// Builds a request to get a DDO.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_ddo_request(int command_handle, string submitter_did, string target_did, BuildRequestResultDelegate cb);

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
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_nym_request(int command_handle, string submitter_did, string target_did, string verkey, string alias, string role, BuildRequestResultDelegate cb);

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
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_attrib_request(int command_handle, string submitter_did, string target_did, string hash, string raw, string enc, BuildRequestResultDelegate cb);

        /// <summary>
        /// Builds a GET_ATTRIB request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="data"> name (attribute name)</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_attrib_request(int command_handle, string submitter_did, string target_did, string data, BuildRequestResultDelegate cb);

        /// <summary>
        /// Builds a GET_NYM request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_nym_request(int command_handle, string submitter_did, string target_did, BuildRequestResultDelegate cb);

        /// <summary>
        /// Builds a SCHEMA request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="data"> name, version, type, attr_names (ip, port, keys)</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_schema_request(int command_handle, string submitter_did, string data, BuildRequestResultDelegate cb);

        /// <summary>
        /// Builds a GET_SCHEMA request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="dest">Id of Identity stored in secured Wallet.</param>
        /// <param name="data">name, version</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_schema_request(int command_handle, string submitter_did, string dest, string data, BuildRequestResultDelegate cb);

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
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_claim_def_txn(int command_handle, string submitter_did, int xref, string signature_type, string data, BuildRequestResultDelegate cb);

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
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_claim_def_txn(int command_handle, string submitter_did, int xref, string signature_type, string origin, BuildRequestResultDelegate cb);

        /// <summary>
        /// Builds a NODE request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="target_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="data">id of a target NYM record</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_node_request(int command_handle, string submitter_did, string target_did, string data, BuildRequestResultDelegate cb);

        /// <summary>
        /// Builds a GET_TXN request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="submitter_did">Id of Identity stored in secured Wallet.</param>
        /// <param name="data">seq_no of transaction in ledger</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_build_get_txn_request(int command_handle, string submitter_did, int data, BuildRequestResultDelegate cb);

        // Signus.rs

        /// <summary>
        /// Creates keys (signing and encryption keys) for a new
        /// DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet)</param>
        /// <param name="did_json">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_and_store_my_did(int command_handle, IntPtr wallet_handle, string did_json, CreateAndStoreMyDidResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_create_and_store_my_did function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="did">The created DID.</param>
        /// <param name="verkey">The verification key for the signature.</param>
        /// <param name="pk">The public key for decryption.</param>
        internal delegate void CreateAndStoreMyDidResultDelegate(int xcommand_handle, int err, string did, string verkey, string pk);

        /// <summary>
        /// Generates new keys (signing and encryption keys) for an existing
        /// DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="did">Id of Identity stored in secured Wallet.</param>
        /// <param name="identity_json">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_replace_keys_start(int command_handle, IntPtr wallet_handle, string did, string identity_json, ReplaceKeysStartResultDelegate cb);
        
        /// <summary>
        /// Delegate for the function called back to by the indy_replace_keys_start function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="verkey">The key for verification of signature.</param>
        /// <param name="pk">The public key for decryption.</param>
        internal delegate void ReplaceKeysStartResultDelegate(int xcommand_handle, int err, string verkey, string pk);

        /// <summary>
        /// Apply temporary keys as main for an existing DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="did">Id of Identity stored in secured Wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_replace_keys_apply(int command_handle, IntPtr wallet_handle, string did, NoValueDelegate cb);

        /// <summary>
        /// Saves their DID for a pairwise connection in a secured Wallet,
        /// so that it can be used to verify transaction.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet)</param>
        /// <param name="identity_json">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_store_their_did(int command_handle, IntPtr wallet_handle, string identity_json, NoValueDelegate cb);

        /// <summary>
        /// Signs a message by a signing key associated with my DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="did">signing DID</param>
        /// <param name="msg_raw">The message to be signed.</param>
        /// <param name="msg_len">The length of the message array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_sign(int command_handle, IntPtr wallet_handle, string did, byte[] msg_raw, int msg_len, SignResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_sign function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="signature_raw">The raw signature bytes.</param>
        /// <param name="signature_len">The length of the signature byte array.</param>
        internal delegate void SignResultDelegate(int xcommand_handle, int err, IntPtr signature_raw, int signature_len);

        /// <summary>
        /// Verify a signature created by a key associated with a DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="pool_handle">pool handle.</param>
        /// <param name="did">DID that signed the message</param>
        /// <param name="msg_raw">The message</param>
        /// <param name="msg_len">The length of the message array.</param>
        /// <param name="signature_raw">The signature.</param>
        /// <param name="signature_len">The length of the signature array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_verify_signature(int command_handle, IntPtr wallet_handle, IntPtr pool_handle, string did, byte[] msg_raw, int msg_len, byte[] signature_raw, int signature_len, VerifySignatureResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_verify_signature function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="valid">true if the signature is valid, otherwise false</param>
        internal delegate void VerifySignatureResultDelegate(int xcommand_handle, int err, bool valid);

        /// <summary>
        /// Encrypts a message by a public key associated with a DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="pool_handle"></param>
        /// <param name="my_did">encrypting DID</param>
        /// <param name="did">encrypting DID (??)</param>
        /// <param name="msg_raw">The message to encrypt.</param>
        /// <param name="msg_len">The length of the message byte array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_encrypt(int command_handle, IntPtr wallet_handle, IntPtr pool_handle, string my_did, string did, byte[] msg_raw, int msg_len, EncryptResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_encrypt function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="encrypted_msg_raw">The encrypted message as an array of bytes.</param>
        /// <param name="encrypted_msg_len">The length of the encrypted message byte array.</param>
        /// <param name="nonce_raw">The nonce as an array of bytes.</param>
        /// <param name="nonce_len">The length of the nonce byte array.</param>
        internal delegate void EncryptResultDelegate(int xcommand_handle, int err, IntPtr encrypted_msg_raw, int encrypted_msg_len, IntPtr nonce_raw, int nonce_len);

        /// <summary>
        /// Decrypts a message encrypted by a public key associated with my DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="my_did">DID</param>
        /// <param name="did">DID that signed the message</param>
        /// <param name="encrypted_msg_raw">encrypted message as a byte array.</param>
        /// <param name="encrypted_msg_len">The length of the message byte array.</param>
        /// <param name="nonce_raw">nonce that encrypted message as a byte array.</param>
        /// <param name="nonce_len">The length of the nonce byte array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_decrypt(int command_handle, IntPtr wallet_handle, string my_did, string did, byte[] encrypted_msg_raw, int encrypted_msg_len, byte[] nonce_raw, int nonce_len, DecryptResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_decrypt function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="decrypted_msg_raw">The decrypted message as an array of bytes.</param>
        /// <param name="decrypted_msg_len">The length of the decrypted message byte array.</param>
        internal delegate void DecryptResultDelegate(int xcommand_handle, int err, IntPtr decrypted_msg_raw, int decrypted_msg_len);

        // anoncreds.rs

        /// <summary>
        /// Create keys (both primary and revocation) for the given schema and signature type (currently only CL signature type is supported).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="issuer_did">a DID of the issuer signing claim_def transaction to the Ledger</param>
        /// <param name="schema_json">schema as a json</param>
        /// <param name="signature_type">signature type (optional). Currently only 'CL' is supported.</param>
        /// <param name="create_non_revoc">whether to request non-revocation claim.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_and_store_claim_def(int command_handle, IntPtr wallet_handle, string issuer_did, string schema_json, string signature_type, bool create_non_revoc, IssuerCreateAndStoreClaimDefResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_create_and_store_claim_def function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claim_def_json">claim definition json containing information about signature type, schema and issuer's public key.</param>
        internal delegate void IssuerCreateAndStoreClaimDefResultDelegate(int xcommand_handle, int err, string claim_def_json);

        /// <summary>
        /// Create a new revocation registry for the given claim definition.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="issuer_did">a DID of the issuer signing revoc_reg transaction to the Ledger</param>
        /// <param name="schema_seq_no">seq no of a schema transaction in Ledger</param>
        /// <param name="max_claim_num">maximum number of claims the new registry can process.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_and_store_revoc_reg(int command_handle, IntPtr wallet_handle, string issuer_did, int schema_seq_no, int max_claim_num, IssuerCreateAndStoreClaimRevocRegResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_create_and_store_revoc_reg function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="revoc_reg_json">Revoc registry json</param>
        internal delegate void IssuerCreateAndStoreClaimRevocRegResultDelegate(int xcommand_handle, int err, string revoc_reg_json);

        /// <summary>
        /// Signs a given claim for the given user by a given key (claim def).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="claim_req_json">a claim request with a blinded secret</param>
        /// <param name="claim_json">a claim containing attribute values for each of requested attribute names.</param>
        /// <param name="user_revoc_index">index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_claim(int command_handle, IntPtr wallet_handle, string claim_req_json, string claim_json, int user_revoc_index, IssuerCreateClaimResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_create_and_store_revoc_reg function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="revoc_reg_update_json">Revocation registry update json with a newly issued claim</param>
        /// <param name="xclaim_json">Claim json containing issued claim, issuer_did, schema_seq_no, and revoc_reg_seq_no
        /// used for issuance</param>
        internal delegate void IssuerCreateClaimResultDelegate(int xcommand_handle, int err, string revoc_reg_update_json, string xclaim_json);

        /// <summary>
        /// Revokes a user identified by a revoc_id in a given revoc-registry.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="issuer_did">The DID of the issuer.</param>
        /// <param name="schema_seq_no">The schema sequence number.</param>
        /// <param name="user_revoc_index">index of the user in the revocation registry</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_revoke_claim(int command_handle, IntPtr wallet_handle, string issuer_did, int schema_seq_no, int user_revoc_index, IssuerRevokeClaimResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_revoke_claim function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="revoc_reg_update_json">Revocation registry update json with a revoked claim</param>
        internal delegate void IssuerRevokeClaimResultDelegate(int xcommand_handle, int err, string revoc_reg_update_json);

        /// <summary>
        /// Stores a claim offer from the given issuer in a secure storage.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="claim_offer_json">claim offer as a json containing information about the issuer and a claim</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_store_claim_offer(int command_handle, IntPtr wallet_handle, string claim_offer_json, NoValueDelegate cb);

        /// <summary>
        /// Gets all stored claim offers (see prover_store_claim_offer).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="filter_json">optional filter to get claim offers for specific Issuer, claim_def or schema only only</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_claim_offers(int command_handle, IntPtr wallet_handle, string filter_json, ProverGetClaimOffersResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_get_claim_offers function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claim_offers_json">A json with a list of claim offers for the filter.</param>
        internal delegate void ProverGetClaimOffersResultDelegate(int xcommand_handle, int err, string claim_offers_json);

        /// <summary>
        /// Creates a master secret with a given name and stores it in the wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="master_secret_name">a new master secret name</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_master_secret(int command_handle, IntPtr wallet_handle, string master_secret_name, NoValueDelegate cb);

        /// <summary>
        /// Creates a clam request json for the given claim offer and stores it in a secure wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="prover_did">a DID of the prover</param>
        /// <param name="claim_offer_json">claim offer as a json containing information about the issuer and a claim</param>
        /// <param name="claim_def_json">claim definition json associated with issuer_did and schema_seq_no in the claim_offer</param>
        /// <param name="master_secret_name">the name of the master secret stored in the wallet</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_and_store_claim_req(int command_handle, IntPtr wallet_handle, string prover_did, string claim_offer_json, string claim_def_json, string master_secret_name, ProverCreateAndStoreClaimReqResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_create_and_store_claim_req function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claim_req_json">Claim request json.</param>
        internal delegate void ProverCreateAndStoreClaimReqResultDelegate(int xcommand_handle, int err, string claim_req_json);

        /// <summary>
        /// Updates the claim by a master secret and stores in a secure wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="claims_json">claim json</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_store_claim(int command_handle, IntPtr wallet_handle, string claims_json, NoValueDelegate cb);

        /// <summary>
        /// Gets human readable claims according to the filter.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="filter_json">filter for claims</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_claims(int command_handle, IntPtr wallet_handle, string filter_json, ProverGetClaimsResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_get_claims function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claims_json">claims json</param>
        internal delegate void ProverGetClaimsResultDelegate(int xcommand_handle, int err, string claims_json);

        /// <summary>
        /// Gets human readable claims matching the given proof request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="proof_request_json">proof request json</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_claims_for_proof_req(int command_handle, IntPtr wallet_handle, string proof_request_json, ProverGetClaimsForProofResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_get_claims_for_proof_req function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claims_json">json with claims for the given pool request.</param>
        internal delegate void ProverGetClaimsForProofResultDelegate(int xcommand_handle, int err, string claims_json);

        /// <summary>
        /// Creates a proof according to the given proof request
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="proof_req_json">proof request json as come from the verifier</param>
        /// <param name="requested_claims_json">either a claim or self-attested attribute for each requested attribute</param>
        /// <param name="schemas_json">all schema jsons participating in the proof request</param>
        /// <param name="master_secret_name">master secret name</param>
        /// <param name="claim_defs_json">all claim definition jsons participating in the proof request</param>
        /// <param name="revoc_regs_json">all revocation registry jsons participating in the proof request</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_proof(int command_handle, IntPtr wallet_handle, string proof_req_json, string requested_claims_json, string schemas_json, string master_secret_name, string claim_defs_json, string revoc_regs_json, ProverCreateProofResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_create_proof function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="proof_json">Proof json.</param>
        internal delegate void ProverCreateProofResultDelegate(int xcommand_handle, int err, string proof_json);

        /// <summary>
        /// Verifies a proof (of multiple claim).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="proof_request_json">initial proof request as sent by the verifier</param>
        /// <param name="proof_json">proof json</param>
        /// <param name="schemas_json">all schema jsons participating in the proof</param>
        /// <param name="claim_defs_jsons">all claim definition jsons participating in the proof</param>
        /// <param name="revoc_regs_json">all revocation registry jsons participating in the proof</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_verifier_verify_proof(int command_handle, string proof_request_json, string proof_json, string schemas_json, string claim_defs_jsons, string revoc_regs_json, VerifierVerifyProofResultDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_verifier_verify_proof function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="valid">true if the proof is valid, otherwise false</param>
        internal delegate void VerifierVerifyProofResultDelegate(int xcommand_handle, int err, bool valid);

        // agent.rs

        /// <summary>
        /// Delegate for the agent functions that receive messages.
        /// </summary>
        /// <param name="xconnection_handle">The handle for the connection the message was recevied on.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="message">The received message.</param>
        internal delegate void AgentMessageReceivedDelegate(IntPtr xconnection_handle, int err, string message);
        
        /// <summary>
        /// Establishes agent to agent connection.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="pool_handle">Pool handle (created by open_pool_ledger).</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="sender_did">Id of sender Identity stored in secured Wallet.</param>
        /// <param name="receiver_did">Id of receiver Identity.</param>
        /// <param name="connection_cb">The function that will be called when the connection has been created.</param>
        /// <param name="message_cb">The function that will be called when a message is received.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_agent_connect(int command_handle, IntPtr pool_handle, IntPtr wallet_handle, string sender_did, string receiver_did, AgentConnectionEstablishedDelegate connection_cb, AgentMessageReceivedDelegate message_cb);

        /// <summary>
        /// Delegate for Agent callbacks that return a connection.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="connection_handle">The handle to the connection.</param>
        internal delegate void AgentConnectionEstablishedDelegate(int xcommand_handle, int err, IntPtr connection_handle);

        /// <summary>
        /// Starts listening for agent connections.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="endpoint">endpoint to use in starting listener.</param>
        /// <param name="listener_cb">The function that will be called when the listener has been created.</param>
        /// <param name="connection_cb">Callback that will be called after establishing of incoming connection.</param>
        /// <param name="message_cb">Callback that will be called on receiving of an incoming message.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_agent_listen(int command_handle, string endpoint, AgentListenerCreatedDelegate listener_cb, AgentListenerConnectionEstablishedDelegate connection_cb, AgentMessageReceivedDelegate message_cb);

        /// <summary>
        /// Delegate for Agent callbacks that return a listener.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="listener_handle">The handle to the listener.</param>
        internal delegate void AgentListenerCreatedDelegate(int xcommand_handle, int err, IntPtr listener_handle);

        /// <summary>
        /// Delegate for when an agent listener receives a connection.
        /// </summary>
        /// <param name="xlistener_handle">The handle for the listener the connection was created on.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="connection_handle">Connection handle to use for messages sending and mapping of incomming messages to to this connection.</param>
        /// <param name="sender_did">Id of sender Identity stored in secured Wallet.</param>
        /// <param name="reciever_did">Id of receiver Identity.</param>
        internal delegate void AgentListenerConnectionEstablishedDelegate(IntPtr xlistener_handle, int err, IntPtr connection_handle, string sender_did, string reciever_did);

        /// <summary>
        /// Add identity to listener.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="listener_handle">listener handle (created by indy_agent_listen).</param>
        /// <param name="pool_handle">pool handle (created by open_pool_ledger).</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="did">DID of identity.</param>
        /// <param name="add_identity_cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_agent_add_identity(int command_handle, IntPtr listener_handle, IntPtr pool_handle, IntPtr wallet_handle, string did, NoValueDelegate add_identity_cb);

        /// <summary>
        /// Remove identity from listener.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="listener_handle">listener handle (created by indy_agent_listen).</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="did">DID of identity.</param>
        /// <param name="rm_identity_cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_agent_remove_identity(int command_handle, IntPtr listener_handle, IntPtr wallet_handle, string did, NoValueDelegate rm_identity_cb);

        /// <summary>
        /// Sends message to connected agent.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="connection_handle">Connection handle returned by indy_agent_connect or indy_agent_listen calls.</param>
        /// <param name="message"> Message to send.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_agent_send(int command_handle, IntPtr connection_handle, string message, NoValueDelegate cb);

        /// <summary>
        /// Closes agent connection.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="connection_handle">Connection handle returned by indy_agent_connect or indy_agent_listen calls.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_agent_close_connection(int command_handle, IntPtr connection_handle, NoValueDelegate cb);

        /// <summary>
        /// Closes agent connection.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="listener_handle">Listener handle returned by indy_agent_listen call.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_agent_close_listener(int command_handle, IntPtr listener_handle, NoValueDelegate cb);


        // pairwise.rs

        /// <summary>
        /// Checks whether a pairwise exists.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="their_did">encrypted DID</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_is_pairwise_exists(int command_handle, IntPtr wallet_handle, string their_did, IsPairwiseExistsDelegate cb);

        /// <summary>
        /// Delegate for pairwise exists that indicates whether or not a pairwise exists.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="exists">Whether or not the pairwise exists.</param>
        internal delegate void IsPairwiseExistsDelegate(int xcommand_handle, int err, bool exists);

        /// <summary>
        /// Creates pairwise.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="their_did">encrypted DID</param>
        /// <param name="my_did">encrypted DID</param>
        /// <param name="metadata">Optional: extra information for pairwise</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_pairwise(int command_handle, IntPtr wallet_handle, string their_did, string my_did, string metadata, NoValueDelegate cb);

        /// <summary>
        /// Get list of saved pairwise.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_list_pairwise(int command_handle, IntPtr wallet_handle, ListPairwiseDelegate cb);

        /// <summary>
        /// Delegate for listing saved pairwise.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="list_pairwise">list of saved pairwise</param>
        internal delegate void ListPairwiseDelegate(int xcommand_handle, int err, string list_pairwise);

        /// <summary>
        /// Gets pairwise information for specific their_did.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="their_did">encrypted DID</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_pairwise(int command_handle, IntPtr wallet_handle, string their_did, GetPairwiseDelegate cb);

        /// <summary>
        /// Delegate for getting a saved pairwise.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="pairwise_info_json">did info associated with their did</param>
        internal delegate void GetPairwiseDelegate(int xcommand_handle, int err, string pairwise_info_json);

        /// <summary>
        /// Save some data in the Wallet for pairwise associated with Did.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="their_did">encrypted DID</param>
        /// <param name="metadata">some extra information for pairwise</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_pairwise_metadata(int command_handle, IntPtr wallet_handle, string their_did, string metadata, NoValueDelegate cb);
    }
}