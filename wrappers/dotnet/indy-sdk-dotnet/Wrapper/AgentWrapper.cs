using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Async wrapper for agent functions.
    /// </summary>
    public sealed class AgentWrapper : AsyncWrapperBase
    {

        public static Task<IntPtr> AgentConnectAsync(IntPtr poolHandle, IntPtr walletHandle, string senderDid, string receiverDid, AgentMessageReceivedDelegate messageCallback)
        {
            var promise = new TaskCompletionSource<IntPtr>();

            var result = LibSovrin.sovrin_agent_connect(
                GetNextCommandHandle(),
                poolHandle,
                walletHandle,
                senderDid,
                receiverDid,
                ResultWithHandleCallback, 
                messageCallback);

            CheckResult(result);

            return promise.Task;
        }

        public static Task<IntPtr> AgentListenAsync(string endpoint, AgentListenConnectionResultDelegate connectionCallback, AgentMessageReceivedDelegate messageCallback)
        {
            var promise = new TaskCompletionSource<IntPtr>();

            var result = LibSovrin.sovrin_agent_listen(
                GetNextCommandHandle(),
                endpoint,
                ResultWithHandleCallback,
                connectionCallback,
                messageCallback);

            CheckResult(result);

            return promise.Task;
        }

        public static Task AgentAddIdentityAsync(IntPtr listenerHandle, IntPtr poolHandle, IntPtr walletHandle, string did)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_add_identity(
                GetNextCommandHandle(),
                listenerHandle,
                poolHandle,
                walletHandle,
                did,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        public static Task AgentRemoveIdentityAsync(IntPtr listenerHandle, IntPtr walletHandle, string did)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_remove_identity(
                GetNextCommandHandle(),
                listenerHandle,
                walletHandle,
                did,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        public static Task AgenSendAsync(IntPtr connectionHandle, string message)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_send(
                GetNextCommandHandle(),
                connectionHandle,
                message,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        public static Task AgenCloseConnectionAsync(IntPtr connectionHandle)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_close_connection(
                GetNextCommandHandle(),
                connectionHandle,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        public static Task AgenCloseListenerAsync(IntPtr listenerHandle)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_close_listener(
                GetNextCommandHandle(),
                listenerHandle,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }
    }
}
