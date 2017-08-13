using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.IndyNativeMethods;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Event raised by an Agent.
    /// </summary>
    public abstract class AgentEvent
    {
        /// <summary>
        /// Initializes a new AgentEvent.
        /// </summary>
        /// <param name="handle">The handle the event is for.</param>
        /// <param name="result">The result for the event.</param>
        public AgentEvent(IntPtr handle, ErrorCode result)
        {
            Handle = handle;
            Result = result;
        }

        /// <summary>
        /// Gets the handle of the owner of the message.
        /// </summary>
        public IntPtr Handle { get; }

        /// <summary>
        /// Gets the error result for the 
        /// </summary>
        public ErrorCode Result { get; }

        /// <summary>
        /// Gets whether or not the result was success.
        /// </summary>
        public bool IsSuccess
        {
            get { return Result == ErrorCode.Success; }
        }
    }

    /// <summary>
    /// Event used when a connection is established on a listener.
    /// </summary>
    public sealed class ConnectionEvent : AgentEvent
    {
        /// <summary>
        /// Initializes a new ConnectionEvent.
        /// </summary>
        /// <param name="listener">The listener the connection was established on.</param>
        /// <param name="result">The result of the opening the connection.</param>
        /// <param name="connection">The connection.</param>
        /// <param name="senderDid">The DID of the sender.</param>
        /// <param name="receiverDid">The DID of the receiver.</param>
        public ConnectionEvent(Agent.Listener listener, ErrorCode result, Agent.Connection connection, string senderDid, string receiverDid) :
            base(listener.Handle, result)
        {
            Connection = connection;
            SenderDid = senderDid;
            ReceiverDid = receiverDid;
            Listener = listener;
        }

        /// <summary>
        /// Gets the listener the connection was recevied on.
        /// </summary>
        public Agent.Listener Listener { get; }

        /// <summary>
        /// Gets the connection.
        /// </summary>
        public Agent.Connection Connection { get; }

        /// <summary>
        /// Gets the DID of the sender.
        /// </summary>
        public string SenderDid { get; }

        /// <summary>
        /// Gets the DID of the receiver.
        /// </summary>
        public string ReceiverDid { get; }
    }

    /// <summary>
    /// Event raised when a message is received.
    /// </summary>
    public sealed class MessageEvent : AgentEvent
    {
        /// <summary>
        /// Initializes a new MessageEvent.
        /// </summary>
        /// <param name="connection">The connection the message was recevied on.</param>
        /// <param name="result">The result of receiving the message.</param>
        /// <param name="message">The message.</param>
        public MessageEvent(Agent.Connection connection, ErrorCode result,  string message) :
            base(connection.Handle, result)
        {
            Connection = connection;
            Message = message;
        }

        /// <summary>
        /// Gets the connection the message was received on.
        /// </summary>
        public Agent.Connection Connection { get; }

        /// <summary>
        /// Gets the received message.
        /// </summary>
        public string Message { get; }
    }


    /// <summary>
    /// Wrapper class for agent functions.
    /// </summary>
    public sealed class Agent : AsyncWrapperBase
    {
        private static IList<ConnectionEvent> _connectionEvents = new List<ConnectionEvent>();
        private static IList<Tuple<IntPtr, TaskCompletionSource<ConnectionEvent>>> _connectionEventWaiters = new List<Tuple<IntPtr, TaskCompletionSource<ConnectionEvent>>>();

        private static IList<MessageEvent> _messageEvents = new List<MessageEvent>();
        private static IList<Tuple<IntPtr, TaskCompletionSource<MessageEvent>>> _messageEventWaiters = new List<Tuple<IntPtr, TaskCompletionSource<MessageEvent>>>();
        
        /// <summary>
        /// Callback to use when a connection receives a message.
        /// </summary>
        private static AgentMessageReceivedDelegate _messageReceivedCallback = (connectionHandle, err, message) =>
        {
            Connection connection;
            _connections.TryGetValue(connectionHandle, out connection);

            if (connection == null)
                return;

            _messageEvents.Add(new MessageEvent(connection, (ErrorCode)err, message));
            NotifyEventWaiters(_messageEvents, _messageEventWaiters);
        };

        /// <summary>
        /// Callback to use when am incoming connection is established to a listener.
        /// </summary>
        private static AgentListenerConnectionEstablishedDelegate _incomingConnectionEstablishedCallback = (listenerHandle, err, connectionHandle, senderDid, receiverDid) =>
        { 
            Listener listener;
            _listeners.TryGetValue(listenerHandle, out listener);

            if (listener == null)
                return;

            Debug.Assert(!_connections.ContainsKey(connectionHandle));

            var connection = new Connection(connectionHandle);
            _connections.Add(connectionHandle, connection);

            _connectionEvents.Add(new ConnectionEvent(listener, (ErrorCode)err, connection, senderDid, receiverDid));
            NotifyEventWaiters(_connectionEvents, _connectionEventWaiters);
        };

        /// <summary>
        /// Notifies any registered waiters if an message event is present for them.
        /// </summary>
        private static void NotifyEventWaiters<T>(IList<T> events, IList<Tuple<IntPtr, TaskCompletionSource<T>>> eventWaiters) where T: AgentEvent
        {
            for (var eventWaiterIndex = eventWaiters.Count - 1; eventWaiterIndex >= 0; eventWaiterIndex--)
            {
                var eventWaiter = eventWaiters[eventWaiterIndex];

                for (var eventIndex = events.Count - 1; eventIndex >= 0; eventIndex--)
                {
                    var theEvent = events[eventIndex];
                    var eventWaiterHandle = eventWaiter.Item1;
                    var eventWaiterTaskCompletionsource = eventWaiter.Item2;

                    if (eventWaiterHandle == theEvent.Handle)
                    {
                        eventWaiters.RemoveAt(eventWaiterIndex);
                        events.RemoveAt(eventIndex);
                        eventWaiterTaskCompletionsource.SetResult(theEvent);
                        return;
                    }
                }
            }
        }

        /// <summary>
        /// Map of connection handles to connections.
        /// </summary>
        private static IDictionary<IntPtr, Connection> _connections = new ConcurrentDictionary<IntPtr, Connection>();

        /// <summary>
        /// Map of listener handles to listeners.
        /// </summary>
        private static IDictionary<IntPtr, Listener> _listeners = new ConcurrentDictionary<IntPtr, Listener>();

        /// <summary>
        /// Callback to use when an outgoing connection is established.
        /// </summary>
        private static AgentConnectionEstablishedDelegate _outgoingConnectionEstablishedCallback = (xCommandHandle, err, connectionHandle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<Connection>(xCommandHandle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            Debug.Assert(!_connections.ContainsKey(connectionHandle));

            var connection = new Connection(connectionHandle);
            _connections.Add(connectionHandle, connection);

            taskCompletionSource.SetResult(connection);
        };

        /// <summary>
        /// Callback to use when a listener is created.
        /// </summary>
        private static AgentListenerCreatedDelegate _listenerCreatedCallback = (xCommandHandle, err, listenerHandle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<Listener>(xCommandHandle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            Debug.Assert(!_listeners.ContainsKey(listenerHandle));

            var listener = new Listener(listenerHandle);
            _listeners.Add(listenerHandle, listener);

            taskCompletionSource.SetResult(listener);
        };
       
        /// <summary>
        /// Creates a connection to an agent.
        /// </summary>
        /// <param name="pool">The ledger pool that the destination DID is registered on.</param>
        /// <param name="wallet">The wallet containing the keys for the DIDs.</param>
        /// <param name="senderDid">The DID to use when initiating the connection.</param>
        /// <param name="receiverDid">The DID of the target of the connection.</param>
        /// <returns>An asynchronous task that returns a Connection result.</returns>
        public static Task<Connection> AgentConnectAsync(Pool pool, Wallet wallet, string senderDid, string receiverDid)
        {
            var taskCompletionSource = new TaskCompletionSource<Connection>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_connect(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                senderDid,
                receiverDid,
                _outgoingConnectionEstablishedCallback,
                _messageReceivedCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a listener than can be connected to by other agents.
        /// </summary>
        /// <param name="endpoint">The endpoint the agent is to be exposed on.</param>
        /// <returns>An asynchronous task that returns a Listener result.</returns>
        public static Task<Listener> AgentListenAsync(string endpoint)
        {
            var taskCompletionSource = new TaskCompletionSource<Listener>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_listen(
                commandHandle,
                endpoint,
                _listenerCreatedCallback,
                _incomingConnectionEstablishedCallback,
                _messageReceivedCallback);

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

            var result = IndyNativeMethods.indy_agent_add_identity(
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

            var result = IndyNativeMethods.indy_agent_remove_identity(
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

            var result = IndyNativeMethods.indy_agent_send(
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

            var result = IndyNativeMethods.indy_agent_close_connection(
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

            var result = IndyNativeMethods.indy_agent_close_listener(
                commandHandle,
                listener.Handle,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Marker interface for agent connections and listeners.
        /// </summary>
        public interface IAgentCommunicator
        {
            /// <summary>
            /// The handle of the communicator.
            /// </summary>
            IntPtr Handle { get; }
        }

        /// <summary>
        /// A connection to an agent.
        /// </summary>
        public sealed class Connection : IAgentCommunicator, IDisposable
        {
        

            private bool _isOpen = true;

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
            /// Waits for a message from the connection.
            /// </summary>
            /// <returns>An asynchronous task that resolves to MessageEvent when a message arrives.</returns>
            public Task<MessageEvent> WaitForMessage()
            {
                var taskCompletionSource = new TaskCompletionSource<MessageEvent>();
                var tuple = Tuple.Create(Handle, taskCompletionSource);
                _messageEventWaiters.Add(tuple);
                NotifyEventWaiters(_messageEvents, _messageEventWaiters);

                return taskCompletionSource.Task;
            }            

            /// <summary>
            /// Closes the connection.
            /// </summary>
            /// <returns>An asynchronous task that returns no value.</returns>
            public Task CloseAsync() 
            {
                _isOpen = false;
                return Agent.AgentCloseConnectionAsync(this);                
            }

            /// <summary>
            /// Disposes of resources used by the connection.
            /// </summary>
            public void Dispose()
            {
                if(_isOpen)
                    CloseAsync().Wait();
            }
        }

        /// <summary>
        /// A listener that can receive connections from an agent.
        /// </summary>
        public sealed class Listener : IAgentCommunicator, IDisposable
        {
            private bool _isOpen = true;

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
            /// Waits for a connection to be established with the listener.
            /// </summary>
            /// <returns>An asynchronous task that resolves to ConnectionEvent when a connection is established.</returns>
            public Task<ConnectionEvent> WaitForConnection()
            {
                var taskCompletionSource = new TaskCompletionSource<ConnectionEvent>();
                var tuple = Tuple.Create(Handle, taskCompletionSource);
                _connectionEventWaiters.Add(tuple);
                NotifyEventWaiters(_connectionEvents, _connectionEventWaiters);

                return taskCompletionSource.Task;
            }

            /// <summary>
            /// Closes the listener.
            /// </summary>
            /// <returns>An asynchronous task that returns no value.</returns>
            public Task CloseAsync()
            {
                _isOpen = false;
                return Agent.AgentCloseListenerAsync(this);                
            }

            /// <summary>
            /// Disposes of resources used by the listener.
            /// </summary>
            public void Dispose()
            {
                if(_isOpen)
                    CloseAsync().Wait();
            }
        }
    }
}
