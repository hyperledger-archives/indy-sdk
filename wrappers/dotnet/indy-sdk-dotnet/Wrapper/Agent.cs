using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.LibIndy;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for agent functions.
    /// </summary>
    public sealed class Agent : AsyncWrapperBase
    {
        /// <summary>
        /// Map of connection handles to connections.
        /// </summary>
        private static IDictionary<IntPtr, Agent.Connection> _connections = new ConcurrentDictionary<IntPtr, Agent.Connection>();

        /// <summary>
        /// Map of listener handles to listeners.
        /// </summary>
        private static IDictionary<IntPtr, Agent.Listener> _listeners = new ConcurrentDictionary<IntPtr, Agent.Listener>();
        
        /// <summary>
        /// Map of command handles to listener observers.
        /// </summary>
        private static IDictionary<int, AgentObservers.ListenerObserver> _listenerObservers = new ConcurrentDictionary<int, AgentObservers.ListenerObserver>();

        /// <summary>
        /// Map of command handles to connection observers.
        /// </summary>
        private static IDictionary<int, AgentObservers.ConnectionObserver> _connectionObservers = new ConcurrentDictionary<int, AgentObservers.ConnectionObserver>();
                
        /// <summary>
        /// Callback to use when a connection is established.
        /// </summary>
        private static AgentConnectionEstablishedDelegate _connectionEstablishedCallback = (xCommandHandle, err, connectionHandle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<Agent.Connection>(xCommandHandle);

            if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                return;

            Debug.Assert(!_connections.ContainsKey(connectionHandle));

            var connection = new Agent.Connection(connectionHandle);
            _connections.Add(connectionHandle, connection);

            var connectionObserver = RemoveConnectionObserver(xCommandHandle);
            var messageObserver = connectionObserver.OnConnection(null, connection, null, null);
            connection.MessageObserver = messageObserver;

            taskCompletionSource.SetResult(connection);
        };

        /// <summary>
        /// Callback to use when a connection receives a message.
        /// </summary>
        private static AgentMessageReceivedDelegate _connectionMessageReceivedCallback = (connectionHandle, err, message) =>
        {
            CheckCallback(err);

            var connection = _connections[connectionHandle];

            if (connection == null)
                return;

            var messageObserver = connection.MessageObserver;
            messageObserver.OnMessage(connection, message);
        };

        /// <summary>
        /// Callback to use when a listener is created.
        /// </summary>
        private static AgentListenerCreatedDelegate _listenerCreatedCallback = (xCommandHandle, err, listenerHandle) =>
        {
            CheckCallback(err);

            Debug.Assert(!_listeners.ContainsKey(listenerHandle));

            var listener = new Agent.Listener(listenerHandle);
            _listeners.Add(listenerHandle, listener);

            var listenerObserver = RemoveListenerObserver(xCommandHandle);
            var connectionObserver = listenerObserver.OnListener(listener);
            listener.ConnectionObserver = connectionObserver;
        };

        /// <summary>
        /// Callback to use when a connection is established to a listener.
        /// </summary>
        private static AgentListenerConnectionEstablishedDelegate _listenerConnectionEstablishedCallback = (listenerHandle, err, connectionHandle, senderDid, receiverDid) =>
        {
            CheckCallback(err);

            Debug.Assert(!_listeners.ContainsKey(listenerHandle));

            var listener = _listeners[listenerHandle];

            if (listener == null)
                return;

            Debug.Assert(!_connections.ContainsKey(connectionHandle));

            var connection = new Agent.Connection(connectionHandle);
            _connections.Add(connectionHandle, connection);

            var connectionObserver = listener.ConnectionObserver;
            var messageObserver = connectionObserver.OnConnection(listener, connection, senderDid, receiverDid);
            connection.MessageObserver = messageObserver;            
        };

        /// <summary>
        /// Callback to use when a message is received by a listener.
        /// </summary>
        private static AgentMessageReceivedDelegate _agentListenerMessageReceivedCallback = (connectionHandle, err, message) =>
        {
            CheckCallback(err);

            var connection = _connections[connectionHandle];

            if (connection == null)
                return;

            var messageObserver = connection.MessageObserver;
            messageObserver.OnMessage(connection, message);
        };

        /// <summary>
        /// Adds a listener observer to track.
        /// </summary>
        /// <param name="listenerObserver">The listener observer to track.</param>
        /// <returns>The handle of the command the listener observer is associated with.</returns>
        private static int AddListenerObserver(AgentObservers.ListenerObserver listenerObserver)
        {
            var commandHandle = GetNextCommandHandle();
            Debug.Assert(!_listenerObservers.ContainsKey(commandHandle));
            _listenerObservers.Add(commandHandle, listenerObserver);

            return commandHandle;
        }

        /// <summary>
        /// Gets a listener observer by it's command handle and removes it from tracking.
        /// </summary>
        /// <param name="commandHandle">The command handle associated with the listener observer.</param>
        /// <returns>The listener observer associated with the command handle.</returns>
        private static AgentObservers.ListenerObserver RemoveListenerObserver(int commandHandle)
        {
            AgentObservers.ListenerObserver observer;
            _listenerObservers.TryGetValue(commandHandle, out observer);

            Debug.Assert(observer != null);

            _listenerObservers.Remove(commandHandle);

            return observer;
        }

        /// <summary>
        /// Adds a connection observer to track.
        /// </summary>
        /// <param name="connectionObserver">The connection observer to track.</param>
        /// <returns>The handle of the command the connection observer is associated with.</returns>
        private static int AddConnectionObserver(AgentObservers.ConnectionObserver connectionObserver)
        {
            int commandHandle = GetNextCommandHandle();
            Debug.Assert(!_connectionObservers.ContainsKey(commandHandle));
            _connectionObservers.Add(commandHandle, connectionObserver);            

            return commandHandle;
        }

        /// <summary>
        /// Gets a connection observer by it's command handle and removes it from tracking.
        /// </summary>
        /// <param name="commandHandle">The command handle associated with the connection observer.</param>
        /// <returns>The connection observer associated with the command handle.</returns>
        private static AgentObservers.ConnectionObserver RemoveConnectionObserver(int commandHandle)
        {
            AgentObservers.ConnectionObserver observer;
            _connectionObservers.TryGetValue(commandHandle, out observer);

            Debug.Assert(observer != null);

            _connectionObservers.Remove(commandHandle);

            return observer;
        }        

        /// <summary>
        /// Creates a connection to an agent.
        /// </summary>
        /// <param name="pool">The ledger pool that the destination DID is registered on.</param>
        /// <param name="wallet">The wallet containing the keys for the DIDs.</param>
        /// <param name="senderDid">The DID to use when initiating the connection.</param>
        /// <param name="receiverDid">The DID of the target of the connection.</param>
        /// <param name="connectionObserver">The observer that will receive events from the connection.</param>
        public static void AgentConnect(Pool pool, Wallet wallet, string senderDid, string receiverDid, AgentObservers.ConnectionObserver connectionObserver)
        {
            var commandHandle = AddConnectionObserver(connectionObserver);

            var result = LibIndy.sovrin_agent_connect(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                senderDid,
                receiverDid,
                _connectionEstablishedCallback,
                _connectionMessageReceivedCallback);

            CheckResult(result);
        }

        /// <summary>
        /// Creates a listener than can be connected to by other agents.
        /// </summary>
        /// <param name="endpoint">The endpoint the agent is to be exposed on.</param>
        /// <param name="listenerObserver">The observer that will receive events from the listener.</param>
        public static void AgentListen(string endpoint, AgentObservers.ListenerObserver listenerObserver)
        {
            var commandHandle = AddListenerObserver(listenerObserver);

            var result = LibIndy.sovrin_agent_listen(
                commandHandle,
                endpoint,
                _listenerCreatedCallback,
                _listenerConnectionEstablishedCallback,
                _agentListenerMessageReceivedCallback);

            CheckResult(result);
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
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_agent_add_identity(
                commandHandle,
                listener.Handle,
                pool.Handle,
                wallet.Handle,
                did,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
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
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_agent_remove_identity(
                commandHandle,
                listener.Handle,
                wallet.Handle,
                did,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sends a message to a connection.
        /// </summary>
        /// <param name="connection">The connection to send the message to.</param>
        /// <param name="message">The message to send.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        private static Task AgentSendAsync(Connection connection, string message)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_agent_send(
                commandHandle,
                connection.Handle,
                message,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Closes a connection.
        /// </summary>
        /// <param name="connection">The connection to close.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        private static Task AgentCloseConnectionAsync(Connection connection)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_agent_close_connection(
                commandHandle,
                connection.Handle,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Closes a listener.
        /// </summary>
        /// <param name="listener">The listener to close.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        private static Task AgentCloseListenerAsync(Listener listener)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_agent_close_listener(
                commandHandle,
                listener.Handle,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
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
            internal AgentObservers.MessageObserver MessageObserver { get; set; }

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

            internal AgentObservers.ConnectionObserver ConnectionObserver { get; set; }

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
