using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Threading.Tasks;
using static Hyperledger.Indy.LedgerApi.NativeMethods;

namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Provides methods for building messages suitable for submission to the ledger and
    /// methods for signing and submitting messages to the ledger.
    /// </summary>
    /// <remarks>
    /// <para>
    /// This class provides methods for generating messages for submission to the ledger; each 
    /// of these methods is prefixed with the word 'Build' and returns a JSON message which must be 
    /// signed and submitted to a node pool. These messages can be submitted to the ledger using the 
    /// <see cref="SignAndSubmitRequestAsync(Pool, Wallet, string, string)"/> or can be signed first 
    /// using the <see cref="SignRequestAsync(Wallet, string, string)"/> method then submitted later 
    /// using the <see cref="SubmitRequestAsync(Pool, string)"/> method.
    /// </para>
    /// </remarks>
    public static class Ledger
    {
        /// <summary>
        /// The 'Steward' NYM role.
        /// </summary>
        public const string NYM_ROLE_STEWARD = "STEWARD";

        /// <summary>
        /// The 'Trustee' NYM role.
        /// </summary>
        public const string NYM_ROLE_TRUSTEE = "TRUSTEE";

        /// <summary>
        /// The 'Trust Anchor' NYM role.
        /// </summary>
        public const string NYM_ROLE_TRUST_ANCHOR = "TRUST_ANCHOR";

        /// <summary>
        /// Gets the callback to use when a command that submits a message to the ledger completes.
        /// </summary>
        private static SubmitRequestCompletedDelegate _submitRequestCallback = (xcommand_handle, err, response_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(response_json);
        };

        /// <summary>
        /// Gets the callback to use when a command that builds a request completes.
        /// </summary>
        private static BuildRequestCompletedDelegate _buildRequestCallback = (xcommand_handle, err, request_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(request_json);
        };

        private static ParseResponseCompletedDelegate _parseResponseCallback = (xcommand_handle, err, id, object_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<ParseResponseResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new ParseResponseResult(id, object_json));
        };

        private static ParseRegistryResponseCompletedDelegate _parseRegistryResponseCallback = (xcommand_handle, err, id, object_json, timestamp) =>
        {
            var taskCompletionSource = PendingCommands.Remove<ParseRegistryResponseResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new ParseRegistryResponseResult(id, object_json, timestamp));
        };

        /// <summary>
        /// Gets the callback to use when the command for SignRequestAsync has completed.
        /// </summary>
        private static SignRequestCompletedDelegate _signRequestCallback = (xcommand_handle, err, signed_request_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(signed_request_json);
        };

        /// <summary>
        /// Signs a request message.
        /// </summary>
        /// <remarks>
        /// This method adds information associated with the submitter specified by the
        /// <paramref name="submitterDid"/> to the JSON provided in the <paramref name="requestJson"/> parameter
        /// then signs it with the submitter's signing key from the provided wallet.
        /// </remarks>
        /// <param name="wallet">The wallet to use for signing.</param>
        /// <param name="submitterDid">The DID of the submitter identity in the provided wallet.</param>
        /// <param name="requestJson">The request JSON to sign.</param>
        /// <returns>An asynchronous task that resolves to a <see cref="string"/> containing the signed 
        /// message.</returns>
        public static Task<string> SignRequestAsync(Wallet wallet, string submitterDid, string requestJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(requestJson, "requestJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = NativeMethods.indy_sign_request(
                commandHandle,
                wallet.Handle,
                submitterDid,
                requestJson,
                _signRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Signs and submits a request to the validator pool.
        /// </summary>
        /// <remarks>
        /// This method adds information associated with the submitter specified by the
        /// <paramref name="submitterDid"/> to the JSON provided in the <paramref name="requestJson"/> parameter
        /// then signs it with the submitter's signing key from the provided <paramref name="wallet"/> and sends the signed 
        /// request message to the specified validator <paramref name="pool"/>.   
        /// </remarks>
        /// <param name="pool">The validator pool to submit the request to.</param>
        /// <param name="wallet">The wallet containing the submitter keys to sign the request with.</param>
        /// <param name="submitterDid">The DID of the submitter identity.</param>
        /// <param name="requestJson">The request JSON to sign and submit.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a JSON <see cref="string"/> 
        /// containing the result of submission when the operation completes.</returns>
        public static Task<string> SignAndSubmitRequestAsync(Pool pool, Wallet wallet, string submitterDid, string requestJson)
        {
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(requestJson, "requestJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_sign_and_submit_request(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                submitterDid,
                requestJson,
                _submitRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Submits a request to the ledger.
        /// </summary>
        /// <remarks>
        /// This method publishes a message to the validator pool specified in the <paramref name="pool"/> parameter as-is 
        /// and assumes that the message was previously prepared for submission.  Requests can be signed prior to using this 
        /// call to the <see cref="SignRequestAsync(Wallet, string, string)"/> method, or messages can be 
        /// both signed and submitted using the <see cref="SignAndSubmitRequestAsync(Pool, Wallet, string, string)"/>
        /// method.
        /// </remarks>
        /// <param name="pool">The validator pool to submit the request to.</param>
        /// <param name="requestJson">The request to submit.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a JSON <see cref="string"/> 
        /// containing the results when the operation completes.</returns>
        public static Task<string> SubmitRequestAsync(Pool pool, string requestJson)
        {
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNullOrWhiteSpace(requestJson, "requestJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_submit_request(
                commandHandle,
                pool.Handle,
                requestJson,
                _submitRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a ledger request to get a DDO.
        /// </summary>
        /// <remarks>
        /// <para>
        /// This message builds a request message that is suitable for requesting a DDO from the ledger.
        /// </para>
        /// <para>
        /// The resulting message can be submitted to the ledger using the <see cref="SignAndSubmitRequestAsync(Pool, Wallet, string, string)"/>
        /// method or can be signed first using the <see cref="SignRequestAsync(Wallet, string, string)"/> 
        /// method then submitted later using the <see cref="SubmitRequestAsync(Pool, string)"/> method.
        /// </para>        
        /// </remarks>
        /// <param name="submitterDid">The DID of the party who will submit the request to the ledger.</param>
        /// <param name="targetDid">The DID of the DDO to get from the ledger.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON.</returns>
        public static Task<string> BuildGetDdoRequestAsync(string submitterDid, string targetDid)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(targetDid, "targetDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_ddo_request(
                commandHandle,
                submitterDid,
                targetDid,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a ledger request to store a NYM.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Builds a request message that is suitable for storing a NYM for the <paramref name="targetDid"/>
        /// on the ledger.
        /// </para>
        /// <para>
        /// Only the <paramref name="submitterDid"/> and <paramref name="targetDid"/> parameters
        /// are required, however the other parameters provide greater control over the process.  Normally
        /// the <paramref name="targetDid"/> and <paramref name="verKey"/> parameters would be from values
        /// generated by a prior call to <see cref="Did.CreateAndStoreMyDidAsync(Wallet, string)"/>.
        /// </para>
        /// <para>
        /// The <paramref name="role"/> parameter dictates what permissions the NYM will have - valid values
        /// are 'STEWARD' and 'TRUSTEE' and 'TRUST_ANCHOR'.
        /// </para>
        /// </remarks>
        /// <param name="submitterDid">The DID of the party who will submit the request to the ledger.</param>
        /// <param name="targetDid">The DID the NYM belongs to.</param>
        /// <param name="verKey">The verification key for the NYM.</param>
        /// <param name="alias">The alias for the NYM.</param>
        /// <param name="role">The role of the NYM.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildNymRequestAsync(string submitterDid, string targetDid, string verKey, string alias, string role)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(targetDid, "targetDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_nym_request(
                commandHandle,
                submitterDid,
                targetDid,
                verKey,
                alias,
                role,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a ledger request for storing an ATTRIB.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Builds a request message that is suitable for setting an attribute on the ledger.
        /// </para>
        /// <para>
        /// The <paramref name="submitterDid"/>, <paramref name="targetDid"/> are mandatory and
        /// any one of the <paramref name="hash"/>, <paramref name="raw"/> or <paramref name="enc"/> 
        /// parameters must also be provided, depending on what type of data should be stored.
        /// </para>
        /// </remarks>
        /// <param name="submitterDid">The DID of the party that will submit the request to the ledger.</param>
        /// <param name="targetDid">The DID the ATTRIB will belong to.</param>
        /// <param name="hash">The hash of the ATTRIB data.</param>
        /// <param name="raw">The raw JSON attribute data.</param>
        /// <param name="enc">The encrypted attribute data.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildAttribRequestAsync(string submitterDid, string targetDid, string hash, string raw, string enc)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(targetDid, "targetDid");

            if (string.IsNullOrWhiteSpace(hash) && string.IsNullOrWhiteSpace(submitterDid) && string.IsNullOrWhiteSpace(enc))
                throw new ArgumentException("At least one of the 'hash', 'submitterDid' or 'enc' parameters must have a value.");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_attrib_request(
                commandHandle,
                submitterDid,
                targetDid,
                hash,
                raw,
                enc,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_ATTRIB ledger request.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Builds a request message that is suitable for requesting an attribute from the 
        /// ledger.
        /// </para>
        /// </remarks>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The target DID.</param>
        /// <param name="raw">The name of the attribute to get.</param>
        /// <param name="hash"></param>
        /// <param name="enc"></param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildGetAttribRequestAsync(string submitterDid, string targetDid, string raw, string hash, string enc)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(targetDid, "targetDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_attrib_request(
                commandHandle,
                submitterDid,
                targetDid,
                raw,
                hash,
                enc,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_NYM ledger request.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Builds a request message that is suitable for requesting a NYM from the 
        /// ledger.
        /// </para>
        /// </remarks>
        /// <param name="submitterDid">The DID of the party submitting the request.</param>
        /// <param name="targetDid">The target DID.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildGetNymRequestAsync(string submitterDid, string targetDid)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(targetDid, "targetDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_nym_request(
                commandHandle,
                submitterDid,
                targetDid,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a SCHEMA ledger request to store a schema.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Builds a request message that is suitable for storing a schema on a 
        /// ledger.  Schema specify the data types and formats which are used to make up claims.
        /// </para>
        /// <para>
        /// The <paramref name="data"/> parameter must contain a JSON string with the members "name",
        /// "version" and "attr_names" that define the schema.  For example the following JSON describes
        /// a schema with the name "access" that is version 1.0 of the schema and specifies the attributes
        /// "ip", "port", and "keys":
        /// <code>
        /// {
        ///     "name":"access",
        ///     "version":"1.0",
        ///     "attr_names":["ip","port","keys"]
        /// }
        /// </code>
        /// </para>
        /// </remarks>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="data">The JSON schema.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildSchemaRequestAsync(string submitterDid, string data)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(data, "data");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_schema_request(
                commandHandle,
                submitterDid,
                data,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a SCHEMA request. Request to add Credential's schema.
        /// </summary>
        /// <returns>The get schema request async.</returns>
        /// <param name="submitterDid">DID of the submitter stored in secured Wallet..</param>
        /// <param name="schemaId">Schema ID in ledger</param>
        public static Task<string> BuildGetSchemaRequestAsync(string submitterDid, string schemaId)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(schemaId, "schemaId");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_schema_request(
                commandHandle,
                submitterDid,
                schemaId,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API.
        /// </summary>
        /// <returns>
        /// Schema Id and Schema json.
        /// {
        ///     id: identifier of schema
        ///     attrNames: array of attribute name strings
        ///     name: Schema's name string
        ///     version: Schema's version string
        ///     ver: Version of the Schema json
        /// }</returns>
        /// <param name="getSchemaResponse">response of GET_SCHEMA request.</param>
        public static Task<ParseResponseResult> ParseGetSchemaResponseAsync(string getSchemaResponse)
        {
            ParamGuard.NotNullOrWhiteSpace(getSchemaResponse, "getSchemaResponse");

            var taskCompletionSource = new TaskCompletionSource<ParseResponseResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_get_schema_response(
                commandHandle,
                getSchemaResponse,
                _parseResponseCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds an CRED_DEF request. Request to add a credential definition (in particular, public key),
        /// that Issuer creates for a particular Credential Schema.
        /// </summary>
        /// <returns>The cred def txn async.</returns>
        /// <param name="submitterDid">DID of the submitter stored in secured Wallet.</param>
        /// <param name="data">Credential definition json</param>
        public static Task<string> BuildCredDefRequestAsync(string submitterDid, string data)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(data, "data");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_cred_def_request(
                commandHandle,
                submitterDid,
                data,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_CRED_DEF request.Request to get a credential definition (in particular, public key),
        /// that Issuer creates for a particular Credential Schema.
        /// </summary>
        /// <returns>The get cred def request async.</returns>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="id">Identifier.</param>
        public static Task<string> BuildGetCredDefRequestAsync(string submitterDid, string id)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(id, "id");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_cred_def_request(
                commandHandle,
                submitterDid,
                id,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.
        /// </summary>
        /// <returns>The get cred def response async.</returns>
        /// <param name="getCredDefResponse">Get cred def response.</param>
        public static Task<ParseResponseResult> ParseGetCredDefResponseAsync(string getCredDefResponse)
        {
            ParamGuard.NotNullOrWhiteSpace(getCredDefResponse, "getCredDefResponse");

            var taskCompletionSource = new TaskCompletionSource<ParseResponseResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_get_cred_def_response(
                commandHandle,
                getCredDefResponse,
                _parseResponseCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a NODE ledger request.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The target DID.</param>
        /// <param name="data">id of a target NYM record</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildNodeRequestAsync(string submitterDid, string targetDid, string data)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(targetDid, "targetDid");
            ParamGuard.NotNullOrWhiteSpace(data, "data");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_node_request(
                commandHandle,
                submitterDid,
                targetDid,
                data,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_TXN request
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="data">seq_no of transaction in ledger</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildGetTxnRequestAsync(string submitterDid, int data)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_txn_request(
                commandHandle,
                submitterDid,
                data,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a POOL_CONFIG request.
        /// </summary>
        /// <returns>Request result as json.</returns>
        /// <param name="submitterDid">Id of Identity stored in secured Wallet.</param>
        /// <param name="writes">If set to <c>true</c> writes.</param>
        /// <param name="force">If set to <c>true</c> force.</param>
        public static Task<string> BuildPoolConfigRequestAsync(string submitterDid, bool writes, bool force)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_pool_config_request(
                commandHandle,
                submitterDid,
                writes,
                force,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a POOL_UPGRADE request.
        /// </summary>
        /// <returns>Request result as json.</returns>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="name">Name.</param>
        /// <param name="version">Version.</param>
        /// <param name="action">Either start or cancel</param>
        /// <param name="sha256">Sha256.</param>
        /// <param name="timeout">Timeout.</param>
        /// <param name="schedule">Schedule.</param>
        /// <param name="justification">Justification.</param>
        /// <param name="reinstall">If set to <c>true</c> reinstall.</param>
        /// <param name="force">If set to <c>true</c> force.</param>
        public static Task<string> BuildPoolUpgradeRequestAsync(string submitterDid, string name, string version, string action, string sha256, int timeout, string schedule, string justification, bool reinstall, bool force)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(name, "name");
            ParamGuard.NotNullOrWhiteSpace(version, "version");
            ParamGuard.NotNullOrWhiteSpace(action, "action");
            ParamGuard.NotNullOrWhiteSpace(sha256, "sha256");
            ParamGuard.NotNullOrWhiteSpace(schedule, "schedule");
            ParamGuard.NotNullOrWhiteSpace(justification, "justification");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_pool_upgrade_request(
                commandHandle,
                submitterDid,
                name,
                version,
                action,
                sha256,
                timeout,
                schedule,
                justification,
                reinstall,
                force,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
        /// to an exists credential definition.
        /// </summary>
        /// <returns>The revoc reg def request async.</returns>
        /// <param name="submitterDid">DID of the submitter stored in secured Wallet.</param>
        /// <param name="data">
        /// data: Revocation Registry data:
        ///     {
        ///         "id": string - ID of the Revocation Registry,
        ///         "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
        ///         "tag": string - Unique descriptive ID of the Registry,
        ///         "credDefId": string - ID of the corresponding CredentialDefinition,
        ///         "value": Registry-specific data {
        ///             "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
        ///             "maxCredNum": number - Maximum number of credentials the Registry can serve.
        ///             "tailsHash": string - Hash of tails.
        ///             "tailsLocation": string - Location of tails file.
        ///             "publicKeys": &lt;public_keys> - Registry's public key.
        ///         },
        ///         "ver": string - version of revocation registry definition json.
        ///     }.</param>
        public static Task<string> BuildRevocRegDefRequestAsync(string submitterDid, string data)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_revoc_reg_def_request(
                commandHandle,
                submitterDid,
                data,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
        /// that Issuer creates for a particular Credential Definition.
        /// </summary>
        /// <returns>Request result as json.</returns>
        /// <param name="submitterDid">DID of the read request sender..</param>
        /// <param name="id">ID of Revocation Registry Definition in ledger..</param>
        public static Task<string> BuildGetRevocRegDefRequestAsync(string submitterDid, string id)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(id, "id");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_revoc_reg_def_request(
                commandHandle,
                submitterDid,
                id,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format
        /// compatible with Anoncreds API.
        /// </summary>
        /// <returns>
        /// Revocation Registry Definition Id and Revocation Registry Definition json.
        /// {
        ///     "id": string - ID of the Revocation Registry,
        ///     "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
        ///     "tag": string - Unique descriptive ID of the Registry,
        ///     "credDefId": string - ID of the corresponding CredentialDefinition,
        ///     "value": Registry-specific data {
        ///         "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
        ///         "maxCredNum": number - Maximum number of credentials the Registry can serve.
        ///         "tailsHash": string - Hash of tails.
        ///         "tailsLocation": string - Location of tails file.
        ///         "publicKeys": &lt;public_keys> - Registry's public key.
        ///     },
        ///     "ver": string - version of revocation registry definition json.
        /// }.</returns>
        /// <param name="getRevocRegDefResponse">response of GET_REVOC_REG_DEF request..</param>
        public static Task<ParseResponseResult> ParseGetRevocRegDefResponseAsync(string getRevocRegDefResponse)
        {
            ParamGuard.NotNullOrWhiteSpace(getRevocRegDefResponse, "getRevocRegDefResponse");

            var taskCompletionSource = new TaskCompletionSource<ParseResponseResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_get_revoc_reg_def_response(
                commandHandle,
                getRevocRegDefResponse,
                _parseResponseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
        /// the new accumulator value and issued/revoked indices.
        /// This is just a delta of indices, not the whole list.
        /// So, it can be sent each time a new credential is issued/revoked.
        /// </summary>
        /// <returns>The revoc reg entry request async.</returns>
        /// <param name="submitterDid">DID of the submitter stored in secured Wallet.</param>
        /// <param name="revocRegDefId">ID of the corresponding RevocRegDef.</param>
        /// <param name="revDefType">Revocation Registry type (only CL_ACCUM is supported for now).</param>
        /// <param name="value">
        /// Registry-specific data: {
        ///     value: {
        ///         prevAccum: string - previous accumulator value.
        ///         accum: string - current accumulator value.
        ///         issued: array&lt;number> - an array of issued indices.
        ///         revoked: array&lt;number> an array of revoked indices.
        ///     },
        ///     ver: string - version revocation registry entry json
        ///
        /// }.</param>
        public static Task<string> BuildRevocRegEntryRequestAsync(string submitterDid, string revocRegDefId, string revDefType, string value)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(revocRegDefId, "revocRegDefId");
            ParamGuard.NotNullOrWhiteSpace(revDefType, "revDefType");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_revoc_reg_entry_request(
                commandHandle,
                submitterDid,
                revocRegDefId,
                revDefType,
                value,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
        /// by ID. The state is defined by the given timestamp.
        /// </summary>
        /// <returns>Request result as json..</returns>
        /// <param name="submitterDid">DID of the read request sender.</param>
        /// <param name="revocRegDefId">ID of the corresponding Revocation Registry Definition in ledger.</param>
        /// <param name="timestamp">Requested time represented as a total number of seconds from Unix Epoch</param>
        public static Task<string> BuildGetRevocRegRequestAsync(string submitterDid, string revocRegDefId, long timestamp)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(revocRegDefId, "revocRegDefId");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_revoc_reg_request(
                commandHandle,
                submitterDid,
                revocRegDefId,
                timestamp,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.
        /// </summary>
        /// <returns>
        /// Revocation Registry Definition Id, Revocation Registry json and Timestamp.
        /// {
        ///     "value": Registry-specific data {
        ///         "accum": string - current accumulator value.
        ///     },
        ///     "ver": string - version revocation registry json
        /// }
        /// </returns>
        /// <param name="getRevocRegResponse">response of GET_REVOC_REG request.</param>
        public static Task<ParseRegistryResponseResult> ParseGetRevocRegResponseAsync(string getRevocRegResponse)
        {
            ParamGuard.NotNullOrWhiteSpace(getRevocRegResponse, "getRevocRegResponse");

            var taskCompletionSource = new TaskCompletionSource<ParseRegistryResponseResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_get_revoc_reg_response(
                commandHandle,
                getRevocRegResponse,
                _parseRegistryResponseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
        /// The Delta is defined by from and to timestamp fields.
        /// If from is not specified, then the whole state till to will be returned.
        /// </summary>
        /// <returns>Request result as json.</returns>
        /// <param name="submitterDid">DID of the read request sender.</param>
        /// <param name="revocRegDefId">ID of the corresponding Revocation Registry Definition in ledger.</param>
        /// <param name="from">Requested time represented as a total number of seconds from Unix Epoch.</param>
        /// <param name="to">Requested time represented as a total number of seconds from Unix Epoch.</param>
        public static Task<string> BuildGetRevocRegDeltaRequestAsync(string submitterDid, string revocRegDefId, long from, long to)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(revocRegDefId, "id");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_revoc_reg_delta_request(
                commandHandle,
                submitterDid,
                revocRegDefId,
                from,
                to,
                _buildRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.
        /// </summary>
        /// <returns>
        /// Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
        /// {
        ///     "value": Registry-specific data {
        ///         prevAccum: string - previous accumulator value.
        ///         accum: string - current accumulator value.
        ///         issued: array&lt;number> - an array of issued indices.
        ///         revoked: array&lt;number> an array of revoked indices.
        ///     },
        ///     "ver": string - version revocation registry delta json
        /// }</returns>
        /// <param name="getRevocRegDeltaResponse">response of GET_REVOC_REG_DELTA request.</param>
        public static Task<ParseRegistryResponseResult> ParseGetRevocRegDeltaResponseAsync(string getRevocRegDeltaResponse)
        {
            ParamGuard.NotNullOrWhiteSpace(getRevocRegDeltaResponse, "getRevocRegDeltaResponse");

            var taskCompletionSource = new TaskCompletionSource<ParseRegistryResponseResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_get_revoc_reg_response(
                commandHandle,
                getRevocRegDeltaResponse,
                _parseRegistryResponseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}
