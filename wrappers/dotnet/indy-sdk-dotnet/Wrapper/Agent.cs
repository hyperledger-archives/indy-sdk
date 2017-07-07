using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for agent functions.
    /// </summary>
    public sealed class Agent : AsyncWrapperBase
    {
        private static ResultWithHandleDelegate AgentConnectCallback { get; }
        private static ResultWithHandleDelegate AgentListenCallback { get; }

        static Agent()
        {
            AgentConnectCallback = (xCommandHandle, err, handle) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<Connection>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(new Connection(handle));
            };
            
            AgentListenCallback = (xCommandHandle, err, handle) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<Listener>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(new Listener(handle));
            };
        }

        public static Task<Connection> AgentConnectAsync(Pool pool, Wallet wallet, string senderDid, string receiverDid, AgentMessageReceivedDelegate messageCallback)
        {
            var promise = new TaskCompletionSource<Connection>();

            var result = LibSovrin.sovrin_agent_connect(
                GetNextCommandHandle(),
                pool.Handle,
                wallet.Handle,
                senderDid,
                receiverDid,
                AgentConnectCallback, 
                messageCallback);

            CheckResult(result);

            return promise.Task;
        }

        public static Task<Listener> AgentListenAsync(string endpoint, AgentListenConnectionResultDelegate connectionCallback, AgentMessageReceivedDelegate messageCallback)
        {
            var promise = new TaskCompletionSource<Listener>();

            var result = LibSovrin.sovrin_agent_listen(
                GetNextCommandHandle(),
                endpoint,
                AgentListenCallback,
                connectionCallback,
                messageCallback);

            CheckResult(result);

            return promise.Task;
        }

        private static Task AgentAddIdentityAsync(IntPtr listenerHandle, IntPtr poolHandle, IntPtr walletHandle, string did)
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

        private static Task AgentRemoveIdentityAsync(IntPtr listenerHandle, IntPtr walletHandle, string did)
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

        private static Task AgentSendAsync(IntPtr connectionHandle, string message)
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

        private static Task AgentCloseConnectionAsync(IntPtr connectionHandle)
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

        private static Task AgentCloseListenerAsync(IntPtr listenerHandle)
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

        public class Connection
        {
            public IntPtr Handle { get; }

            internal Connection(IntPtr handle)
            {
                Handle = handle;
            }

            public Task SendAsync(string message) 
            {
			    return Agent.AgentSendAsync(Handle, message);
            }

            public Task CloseAsync() 
            {
    			return Agent.AgentCloseConnectionAsync(Handle);
            }
        }

        public class Listener
        {
            public IntPtr Handle { get; }

            internal Listener(IntPtr handle)
            {
                Handle = handle;
            }

            public Task AddIdentityAsync(Pool pool, Wallet wallet, String did)
            {
                return Agent.AgentAddIdentityAsync(Handle, pool.Handle, wallet.Handle, did);
            }

            public Task RemoveIdentityAsync(Wallet wallet, String did)
            {
                return Agent.AgentRemoveIdentityAsync(Handle, wallet.Handle, did);
            }

            public Task CloseAsync()
            {
			    return Agent.AgentCloseListenerAsync(Handle);
            }
        }

    }
}
