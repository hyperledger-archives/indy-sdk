using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
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
        /// Handles messages received on a connection.
        /// </summary>
        /// <param name="connection">The connection that received the message.</param>
        /// <param name="message">The received message.</param>
        public delegate void MessageReceivedHandler(Connection connection, string message);

        /// <summary>
        /// Handles the opening of a connection.
        /// </summary>
        /// <param name="listener">The listener that received the incoming connection.  If the connection is outgoing this will be null.</param>
        /// <param name="connection">The connection that was opened.</param>
        /// <param name="senderDid">The DID of the initator of the connection.</param>
        /// <param name="receiverDid">The DID of the destination of the connection.</param>
        /// <returns></returns>
        public delegate MessageReceivedHandler ConnectionOpenedHandler(Listener listener, Connection connection, string senderDid, string receiverDid);


        /// <summary>
        /// Map of connection handles to connections.
        /// </summary>
        private static IDictionary<IntPtr, Connection> _connections = new ConcurrentDictionary<IntPtr, Connection>();

        /// <summary>
        /// Map of listener handles to listeners.
        /// </summary>
        private static IDictionary<IntPtr, Listener> _listeners = new ConcurrentDictionary<IntPtr, Listener>();
        
        /// <summary>
        /// Map of command handles to message observers.
        /// </summary>
        private static IDictionary<int, MessageReceivedHandler> _messageReceivedHandlers = new ConcurrentDictionary<int, MessageReceivedHandler>();

        /// <summary>
        /// Map of command handles to connection observers.
        /// </summary>
        private static IDictionary<int, ConnectionOpenedHandler> _connectionOpenedHandlers = new ConcurrentDictionary<int, ConnectionOpenedHandler>();
                
        /// <summary>
        /// Callback to use when an outgoing connection is established.
        /// </summary>
        private static AgentConnectionEstablishedDelegate _connectionEstablishedCallback = (xCommandHandle, err, connectionHandle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<Connection>(xCommandHandle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            Debug.Assert(!_connections.ContainsKey(connectionHandle));

            var connection = new Connection(connectionHandle);
            _connections.Add(connectionHandle, connection);

            connection.MessageReceivedHandler = RemoveMessageReceivedHandler(xCommandHandle);

            taskCompletionSource.SetResult(connection);
        };

        /// <summary>
        /// Callback to use when a connection receives a message.
        /// </summary>
        private static AgentMessageReceivedDelegate _connectionMessageReceivedCallback = (connectionHandle, err, message) =>
        {
            CheckCallback(err);

            Connection connection;
            _connections.TryGetValue(connectionHandle, out connection);

            if (connection == null)
                return;

            var handler = connection.MessageReceivedHandler;
            handler(connection, message);
        };

        /// <summary>
        /// Callback to use when a listener is created.
        /// </summary>
        private static AgentListenerCreatedDelegate _listenerCreatedCallback = (xCommandHandle, err, listenerHandle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<Listener>(xCommandHandle);

            if(!CheckCallback(taskCompletionSource, err))
                return;

            Debug.Assert(!_listeners.ContainsKey(listenerHandle));

            var listener = new Listener(listenerHandle);
            _listeners.Add(listenerHandle, listener);

            listener.ConnectionOpenedHandler = RemoveConnectionOpenedHandler(xCommandHandle);
            taskCompletionSource.SetResult(listener);
        };

        /// <summary>
        /// Callback to use when am incoming connection is established to a listener.
        /// </summary>
        private static AgentListenerConnectionEstablishedDelegate _listenerConnectionEstablishedCallback = (listenerHandle, err, connectionHandle, senderDid, receiverDid) =>
        {
            CheckCallback(err);

            Listener listener;
            _listeners.TryGetValue(listenerHandle, out listener);

            if (listener == null)
                return;

            Debug.Assert(!_connections.ContainsKey(connectionHandle));

            var connection = new Connection(connectionHandle);
            _connections.Add(connectionHandle, connection);

            var handler = listener.ConnectionOpenedHandler;
            connection.MessageReceivedHandler = handler(listener, connection, senderDid, receiverDid);          
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

            var handler = connection.MessageReceivedHandler;
            handler(connection, message);
        };

        /// <summary>
        /// Adds a message received handler to track.
        /// </summary>
        /// <param name="commandHandle">The command handle to use when tracking the message observer.</param>
        /// <param name="handler">The handler to track.</param>
        /// <returns>The handle of the command the handler is associated with.</returns>
        private static void AddMessageReceivedHandler(int commandHandle, MessageReceivedHandler handler)
        {
            Debug.Assert(!_messageReceivedHandlers.ContainsKey(commandHandle));
            _messageReceivedHandlers.Add(commandHandle, handler);
        }

        /// <summary>
        /// Gets a MessageReceivedHandler by it's command handle and removes it from tracking.
        /// </summary>
        /// <param name="commandHandle">The command handle associated with the handler.</param>
        /// <returns>The handler associated with the command handle.</returns>
        private static MessageReceivedHandler RemoveMessageReceivedHandler(int commandHandle)
        {
            MessageReceivedHandler handler;
            _messageReceivedHandlers.TryGetValue(commandHandle, out handler);

            Debug.Assert(handler != null);

            _messageReceivedHandlers.Remove(commandHandle);

            return handler;
        }

        /// <summary>
        /// Adds a connection handler to track.
        /// </summary>
        /// <param name="commandHandle">The command handle to use when tracking the handler.</param>
        /// <param name="handler">The handler to track.</param>
        /// <returns>The handle of the command the handler is associated with.</returns>
        private static int AddConnectionOpenedHandler(int commandHandle, ConnectionOpenedHandler handler)
        {
            Debug.Assert(!_connectionOpenedHandlers.ContainsKey(commandHandle));
            _connectionOpenedHandlers.Add(commandHandle, handler);            

            return commandHandle;
        }

        /// <summary>
        /// Gets a ConnectionOpenedHandler by it's command handle and removes it from tracking.
        /// </summary>
        /// <param name="commandHandle">The command handle associated with the handle.</param>
        /// <returns>The handler associated with the command handle.</returns>
        private static ConnectionOpenedHandler RemoveConnectionOpenedHandler(int commandHandle)
        {
            ConnectionOpenedHandler handler;
            _connectionOpenedHandlers.TryGetValue(commandHandle, out handler);

            Debug.Assert(handler != null);

            _connectionOpenedHandlers.Remove(commandHandle);

            return handler;
        }

        /// <summary>
        /// Creates a connection to an agent.
        /// </summary>
        /// <param name="pool">The ledger pool that the destination DID is registered on.</param>
        /// <param name="wallet">The wallet containing the keys for the DIDs.</param>
        /// <param name="senderDid">The DID to use when initiating the connection.</param>
        /// <param name="receiverDid">The DID of the target of the connection.</param>
        /// <param name="messageReceivedHandler">The observer that will receive message events from the connection.</param>
        /// <returns>An asynchronous task that returns a Connection result.</returns>
        public static Task<Connection> AgentConnectAsync(Pool pool, Wallet wallet, string senderDid, string receiverDid, MessageReceivedHandler messageReceivedHandler)
        {
            var taskCompletionSource = new TaskCompletionSource<Connection>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);
            AddMessageReceivedHandler(commandHandle, messageReceivedHandler);

            var result = LibIndy.indy_agent_connect(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                senderDid,
                receiverDid,
                _connectionEstablishedCallback,
                _connectionMessageReceivedCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a listener than can be connected to by other agents.
        /// </summary>
        /// <param name="endpoint">The endpoint the agent is to be exposed on.</param>
        /// <param name="connectionOpenedHandler">The observer that will receive connection events from the listener.</param>
        /// <returns>An asynchronous task that returns a Listener result.</returns>
        public static Task<Listener> AgentListenAsync(string endpoint, ConnectionOpenedHandler connectionOpenedHandler)
        {
            var taskCompletionSource = new TaskCompletionSource<Listener>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);
            AddConnectionOpenedHandler(commandHandle, connectionOpenedHandler);

            var result = LibIndy.indy_agent_listen(
                commandHandle,
                endpoint,
                _listenerCreatedCallback,
                _listenerConnectionEstablishedCallback,
                _agentListenerMessageReceivedCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
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

            var result = LibIndy.indy_agent_add_identity(
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

            var result = LibIndy.indy_agent_remove_identity(
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

            var result = LibIndy.indy_agent_send(
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

            var result = LibIndy.indy_agent_close_connection(
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

            var result = LibIndy.indy_agent_close_listener(
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
            internal MessageReceivedHandler MessageReceivedHandler { get; set; }

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

            internal ConnectionOpenedHandler ConnectionOpenedHandler { get; set; }

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
