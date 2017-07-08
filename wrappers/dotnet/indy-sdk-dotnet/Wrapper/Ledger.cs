
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for ledger functions.
    /// </summary>
    public sealed class Ledger : AsyncWrapperBase
    {
        /// <summary>
        /// Gets the callback to use when a command that submits a message to the ledger completes.
        /// </summary>
        private static SubmitRequestResultDelegate SubmitRequestResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when a command that builds a request completes.
        /// </summary>
        private static BuildRequestResultDelegate BuildRequestResultCallback { get; }

        /// <summary>
        /// Static initializer
        /// </summary>
        static Ledger()
        {
            SubmitRequestResultCallback = (xCommandHandle, err, responseJson) => 
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(responseJson);
            };

            BuildRequestResultCallback = (xCommandHandle, err, requestJson) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(requestJson);
            };
        }

        /// <summary>
        /// Signs and submits a request to the ledger.
        /// </summary>
        /// <param name="pool">The ledger pool to submit to.</param>
        /// <param name="wallet">The wallet containing the keys to sign with.</param>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="requstJson">The request to sign and submit.</param>
        /// <returns>An asynchronous Task that returns the submit result.</returns>
        public static Task<string> SignAndSubmitRequestAsync(Pool pool, Wallet wallet, string submitterDid, string requstJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_sign_and_submit_request(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                submitterDid,
                requstJson,                
                SubmitRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Submits a pre-signed request to the ledger.
        /// </summary>
        /// <param name="pool">The ledger pool to submit to.</param>
        /// <param name="requstJson">The signed request to submit.</param>
        /// <returns>An asynchronous Task that returns the submit result.</returns>
        public static Task<string> SubmitRequestAsync(Pool pool, string requstJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_submit_request(
                commandHandle,
                pool.Handle,
                requstJson,
                SubmitRequestResultCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a ledger request to get a DDO.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The DID of the DDO to get.</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildGetDdoRequestAsync(string submitterDid, string targetDid)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_get_ddo_request(
                commandHandle,
                submitterDid,
                targetDid,
                BuildRequestResultCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a ledger request to store a NYM.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The DID the NYM belongs to.</param>
        /// <param name="verKey">The verification key.</param>
        /// <param name="alias">The alias.</param>
        /// <param name="role">The role.</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildNymRequestAsync(string submitterDid, string targetDid, string verKey, string alias, string role)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_nym_request(
                commandHandle,
                submitterDid,
                targetDid,
                verKey,
                alias,
                role,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a ledger request for storing an ATTRIB.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The DID the ATTRIB belongs to.</param>
        /// <param name="hash">The hash of the ATTRIB data.</param>
        /// <param name="raw">The raw JSON attribute data.</param>
        /// <param name="enc">The encrypted attribute data.</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildAttribRequestAsync(string submitterDid, string targetDid, string hash, string raw, string enc)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_attrib_request(
                commandHandle,
                submitterDid,
                targetDid,
                hash,
                raw,
                enc,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_ATTRIB ledger request.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The target DID.</param>
        /// <param name="data">The name of the attibute to get.</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildGetAttribRequestAsync(string submitterDid, string targetDid, string data)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_get_attrib_request(
                commandHandle,
                submitterDid,
                targetDid,
                data,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_NYM ledger request.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The target DID.</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildGetNymRequestAsync(string submitterDid, string targetDid)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_get_nym_request(
                commandHandle,
                submitterDid,
                targetDid,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a SCHEMA ledger request to store a schema.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="data">name, version, type, attr_names (ip, port, keys)</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildSchemaRequestAsync(string submitterDid, string data)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_schema_request(
                commandHandle,
                submitterDid,
                data,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_SCHEMA ledger request.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="dest">The DID of the destination.</param>
        /// <param name="data">name, version</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildGetSchemaRequestAsync(string submitterDid, string dest, string data)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_get_schema_request(
                commandHandle,
                submitterDid,
                dest,
                data,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a CLAIM_DEF ledger request.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="xref">Seq. number of schema</param>
        /// <param name="signatureType">signature type (only CL supported now)</param>
        /// <param name="data">components of a key in json: N, R, S, Z</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildClaimDefTxnAsync(string submitterDid, int xref, string signatureType, string data)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_claim_def_txn(
                commandHandle,
                submitterDid,
                xref,
                signatureType,
                data,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a GET_CLAIM_DEF ledger request.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="xref">Seq. number of schema</param>
        /// <param name="signatureType">signature type (only CL supported now)</param>
        /// <param name="origin">The issuer DID.</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildGetClaimDefTxnAsync(string submitterDid, int xref, string signatureType, string origin)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_get_claim_def_txn(
                commandHandle,
                submitterDid,
                xref,
                signatureType,
                origin,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds a NODE ledger request.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The target DID.</param>
        /// <param name="data">id of a target NYM record</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildNodeRequestAsync(string submitterDid, string targetDid, string data)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_node_request(
                commandHandle,
                submitterDid,
                targetDid,
                data,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}
