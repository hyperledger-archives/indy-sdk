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
        /// Builds a GET_SCHEMA ledger request.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Builds a request message that is suitable for requesting a schema from a ledger.
        /// </para>
        /// <para>
        /// The <paramref name="data"/> parameter must contain a JSON string with the members "name",
        /// and "version" specifying the schema to get.  For example the following JSON describes
        /// a request for a schema with the name "access" that is version 1.0:
        /// <code>
        /// {
        ///     "name":"access",
        ///     "version":"1.0"
        /// }
        /// </code>
        /// </para>
        /// </remarks>
        /// <param name="submitterDid">The DID of the party that will submit the request.</param>
        /// <param name="dest">The DID of the destination.</param>
        /// <param name="data">A JSON string specifying what schema to get.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildGetSchemaRequestAsync(string submitterDid, string dest, string data)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(dest, "dest");
            ParamGuard.NotNullOrWhiteSpace(data, "data");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_schema_request(
                commandHandle,
                submitterDid,
                dest,
                data,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a CLAIM_DEF ledger request.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Builds a request message that is suitable for storing a claim definition on a ledger.
        /// A claim definition is published by a claim issuer (e.g. a bank, passport office etc). It
        /// references the relevant schema, the issuer that published the claim definition, and the
        /// signature types used.
        /// </para>
        /// <para>
        /// The <paramref name="data"/> parameter expects a JSON string with the components of a key.  
        /// For example:
        /// <code>
        /// {
        ///     "primary":{
        ///         "n":"1",
        ///         "s":"2",
        ///         "rms":"3",
        ///         "r":{
        ///             "name":"1"
        ///          },
        ///          "rctxt":"1",
        ///          "z":"1"
        ///      }
        /// }
        /// </code>
        /// 
        /// TODO: Better example required.
        /// </para>
        /// <note type="note">The <paramref name="signatureType"/> parameter only accepts the value
        /// 'CL' at present.</note>
        /// </remarks>
        /// <param name="submitterDid">The DID of the party that will submit the request.</param>
        /// <param name="xref">The sequence number of schema.</param>
        /// <param name="signatureType">The signature type.</param>
        /// <param name="data">A JSON string with the components of a key.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildClaimDefTxnAsync(string submitterDid, int xref, string signatureType, string data)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(signatureType, "signatureType");
            ParamGuard.NotNullOrWhiteSpace(data, "data");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_claim_def_txn(
                commandHandle,
                submitterDid,
                xref,
                signatureType,
                data,
                _buildRequestCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_CLAIM_DEF ledger request.
        /// </summary>
        /// <remarks>
        /// Builds a request message that is suitable for getting a claim definition from a ledger.
        /// </remarks>
        /// <note type="note">The <paramref name="signatureType"/> parameter only accepts the value
        /// 'CL' at present.</note>
        /// <param name="submitterDid">The DID of the party that will submit the request.</param>
        /// <param name="xref">The sequence number of the schema the claim definition targets.</param>
        /// <param name="signatureType">The type of signature used in the claim definition.</param>
        /// <param name="origin">The DID of the issuer of the claim definition.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="string"/> 
        /// containing the request JSON. </returns>
        public static Task<string> BuildGetClaimDefTxnAsync(string submitterDid, int xref, string signatureType, string origin)
        {
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(signatureType, "signatureType");
            ParamGuard.NotNullOrWhiteSpace(origin, "origin");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_claim_def_txn(
                commandHandle,
                submitterDid,
                xref,
                signatureType,
                origin,
                _buildRequestCallback
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
    }
}
