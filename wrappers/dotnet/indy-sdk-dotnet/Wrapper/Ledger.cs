
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for ledger functions.
    /// </summary>
    public sealed class Ledger : AsyncWrapperBase
    {
        private static SubmitRequestResultDelegate SubmitRequestResultCallback { get; }
        private static BuildRequestResultDelegate BuildRequestResultCallback { get; }

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

        public static Task<string> BuildGetSchemaRequestAsync(string submitterDid, string data)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_get_schema_request(
                commandHandle,
                submitterDid,
                data,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        public static Task<string> BuildClaimDefTxnAsync(string submitterDid, string xref, string data)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_claim_def_txn(
                commandHandle,
                submitterDid,
                xref,
                data,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        public static Task<string> BuildGetClaimDefTxnAsync(string submitterDid, string xref)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var result = LibSovrin.sovrin_build_get_claim_def_txn(
                commandHandle,
                submitterDid,
                xref,
                BuildRequestResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

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
