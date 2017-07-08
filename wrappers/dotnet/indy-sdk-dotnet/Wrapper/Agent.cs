using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for agent functions.
    /// </summary>
    public sealed class Agent : AsyncWrapperBase
    {
        /// <summary>
        /// Gets the callback that will be used when an Agent connection is started.
        /// </summary>
        private static AgentConnectionCreatedDelegate AgentConnectCallback { get; }

        /// <summary>
        /// Gets the callback that will be used when an Agent listener is started.
        /// </summary>
        private static AgentListenerCreatedDelegate AgentListenCallback { get; }

        /// <summary>
        /// Initializes Agent callbacks.
        /// </summary>
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

        /// <summary>
        /// Creates a connection to an agent.
        /// </summary>
        /// <param name="pool">The ledger pool that the destination DID is registered on.</param>
        /// <param name="wallet">The wallet containing the keys for the DIDs.</param>
        /// <param name="senderDid">The DID to use when initiating the connection.</param>
        /// <param name="receiverDid">The DID of the target of the connection.</param>
        /// <param name="messageCallback">The callback to use when messages are received from the connection.</param>
        /// <returns>An asynchronous Task that returns an Agent.Connection instance.</returns>
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

        /// <summary>
        /// Creates a listener than can be connected to by other agents.
        /// </summary>
        /// <param name="endpoint">The endpoint the agent is to be exposed on.</param>
        /// <param name="connectionCallback">The callback to use when a connection is established by an agent.</param>
        /// <param name="messageCallback">The callback to use when a message is recevied from a connected agent.</param>
        /// <returns>An asynchronous Task that returns an Agent.Listener instance.</returns>
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

        /// <summary>
        /// Adds an identity to an agent listener.
        /// </summary>
        /// <param name="listener">The listener to add the identity to.</param>
        /// <param name="pool">The pool.</param>
        /// <param name="wallet">The wallet.</param>
        /// <param name="did">The DID of the identity to add.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        private static Task AgentAddIdentityAsync(Listener listener, Pool pool, Wallet wallet, string did)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_add_identity(
                GetNextCommandHandle(),
                listener.Handle,
                pool.Handle,
                wallet.Handle,
                did,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        /// <summary>
        /// Removes an identity from a listener.
        /// </summary>
        /// <param name="listener">The listener to remove the identity from.</param>
        /// <param name="wallet">The wallet.</param>
        /// <param name="did">The DID of the identity to remove.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        private static Task AgentRemoveIdentityAsync(Listener listener, Wallet wallet, string did)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_remove_identity(
                GetNextCommandHandle(),
                listener.Handle,
                wallet.Handle,
                did,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        /// <summary>
        /// Sends a message to a connection.
        /// </summary>
        /// <param name="connection">The connection to send the message to.</param>
        /// <param name="message">The message to send.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        private static Task AgentSendAsync(Connection connection, string message)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_send(
                GetNextCommandHandle(),
                connection.Handle,
                message,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        /// <summary>
        /// Closes a connection.
        /// </summary>
        /// <param name="connection">The connection to close.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        private static Task AgentCloseConnectionAsync(Connection connection)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_close_connection(
                GetNextCommandHandle(),
                connection.Handle,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        /// <summary>
        /// Closes a listener.
        /// </summary>
        /// <param name="listener">The listener to close.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        private static Task AgentCloseListenerAsync(Listener listener)
        {
            var promise = new TaskCompletionSource<bool>();

            var result = LibSovrin.sovrin_agent_close_listener(
                GetNextCommandHandle(),
                listener.Handle,
                ResultOnlyCallback
                );

            CheckResult(result);

            return promise.Task;
        }

        /// <summary>
        /// A connection to an agent.
        /// </summary>
        public class Connection
        {
            /// <summary>
            /// Gets the handle for the connection.
            /// </summary>
            public IntPtr Handle { get; }

            /// <summary>
            /// Initializes a new connection.
            /// </summary>
            /// <param name="handle">The handle for the connection.</param>
            internal Connection(IntPtr handle)
            {
                Handle = handle;
            }

            /// <summary>
            /// Sends a message to the connection.
            /// </summary>
            /// <param name="message">The message to send.</param>
            /// <returns>An asynchronous task that returns no value.</returns>
            public Task SendAsync(string message) 
            {
			    return Agent.AgentSendAsync(this, message);
            }

            /// <summary>
            /// Closes the connection.
            /// </summary>
            /// <returns>An asynchronous task that returns no value.</returns>
            public Task CloseAsync() 
            {
    			return Agent.AgentCloseConnectionAsync(this);
            }
        }

        /// <summary>
        /// A listener that can receive connections from an agent.
        /// </summary>
        public class Listener
        {
            /// <summary>
            /// Gets the handle for the listener.
            /// </summary>
            public IntPtr Handle { get; }

            /// <summary>
            /// Initializes a new Listener with the specified handle.
            /// </summary>
            /// <param name="handle">The handle for the listener.</param>
            internal Listener(IntPtr handle)
            {
                Handle = handle;
            }

            /// <summary>
            /// Adds an identity to the listener.
            /// </summary>
            /// <param name="pool">The pool.</param>
            /// <param name="wallet">The wallet that contains the identity.</param>
            /// <param name="did">The DID of the identity to add.</param>
            /// <returns>An asynchronous task that returns no value.</returns>
            public Task AddIdentityAsync(Pool pool, Wallet wallet, String did)
            {
                return Agent.AgentAddIdentityAsync(this, pool, wallet, did);
            }

            /// <summary>
            /// Removes an identity from the listener.
            /// </summary>
            /// <param name="wallet">The wallet that contains the identity.</param>
            /// <param name="did">The DID of the identity to remove. </param>
            /// <returns>An asynchronous task that returns no value.</returns>
            public Task RemoveIdentityAsync(Wallet wallet, String did)
            {
                return Agent.AgentRemoveIdentityAsync(this, wallet, did);
            }

            /// <summary>
            /// Closes the listener.
            /// </summary>
            /// <returns>An asynchronous task that returns no value.</returns>
            public Task CloseAsync()
            {
			    return Agent.AgentCloseListenerAsync(this);
            }
        }
    }
}
