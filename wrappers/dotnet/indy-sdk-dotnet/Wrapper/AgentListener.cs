using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.IndyNativeMethods;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// A listener that can receive connections from an agent.
    /// </summary>
    public sealed class AgentListener : AsyncWrapperBase, IDisposable
    {
        /// <summary>
        /// Pending connection events.
        /// </summary>
        private static IList<AgentConnectionEvent> _events = new List<AgentConnectionEvent>();

        /// <summary>
        /// Tasks waiting on connection events.
        /// </summary>
        private static IList<Tuple<IntPtr, TaskCompletionSource<AgentConnectionEvent>>> _eventWaiters = new List<Tuple<IntPtr, TaskCompletionSource<AgentConnectionEvent>>>();

        /// <summary>
        /// Callback to use when a listener is created.
        /// </summary>
        private static AgentListenerCreatedDelegate _listenerCreatedCallback = (xcommand_handle, err, listener_handle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<AgentListener>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var listener = (AgentListener)taskCompletionSource.Task.AsyncState;
            listener.Handle = listener_handle;

            taskCompletionSource.SetResult(listener);
        };

        /// <summary>
        /// Creates a listener than can be connected to by other agents.
        /// </summary>
        /// <param name="endpoint">The endpoint the agent is to be exposed on.</param>
        /// <returns>An asynchronous task that returns a Listener result.</returns>
        public static Task<AgentListener> ListenAsync(string endpoint)
        {
            var listener = new AgentListener();

            var taskCompletionSource = new TaskCompletionSource<AgentListener>(listener);
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_listen(
                commandHandle,
                endpoint,
                _listenerCreatedCallback,
                listener.ConnectionEstablishedCallback,
                listener.MessageReceivedCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Notifies any registered waiters if an message event is present for them.
        /// </summary>
        private static void NotifyEventWaiters()
        {
            for (var eventWaiterIndex = _eventWaiters.Count - 1; eventWaiterIndex >= 0; eventWaiterIndex--)
            {
                var eventWaiter = _eventWaiters[eventWaiterIndex];

                for (var eventIndex = _events.Count - 1; eventIndex >= 0; eventIndex--)
                {
                    var theEvent = _events[eventIndex];
                    var eventWaiterHandle = eventWaiter.Item1;
                    var eventWaiterTaskCompletionsource = eventWaiter.Item2;

                    if (eventWaiterHandle == theEvent.Handle)
                    {
                        _eventWaiters.RemoveAt(eventWaiterIndex);
                        _events.RemoveAt(eventIndex);
                        eventWaiterTaskCompletionsource.SetResult(theEvent);
                        return;
                    }
                }
            }
        }

        /// <summary>
        /// Connections opened on the listener.
        /// </summary>
        private IDictionary<IntPtr, AgentConnection> _connections = new ConcurrentDictionary<IntPtr, AgentConnection>();

        /// <summary>
        /// Whether or not the listener is open.
        /// </summary>
        private bool _isOpen = true;

        /// <summary>
        /// Gets the handle for the listener.
        /// </summary>
        public IntPtr Handle { get; private set; }

        /// <summary>
        /// Reference to the handle for processing message received events.
        /// </summary>
        private AgentMessageReceivedDelegate MessageReceivedCallback { get; }

        /// <summary>
        /// Reference to the handler for processing connection established events.
        /// </summary>
        private AgentListenerConnectionEstablishedDelegate ConnectionEstablishedCallback { get; }

        /// <summary>
        /// Callback to use when am incoming connection is established to a listener.
        /// </summary>
        private void ConnectionEstablishedHandler(IntPtr listener_handle, int err, IntPtr connection_handle, string sender_did, string receiver_did)
        {
            Debug.Assert(!_connections.ContainsKey(connection_handle));

            var connection = new AgentConnection(connection_handle);
            _connections.Add(connection_handle, connection);

            _events.Add(new AgentConnectionEvent(this, (ErrorCode)err, connection, sender_did, receiver_did));
            NotifyEventWaiters();
        }

        /// <summary>
        /// Handles message received events.
        /// </summary>
        /// <param name="connection_handle">The connection handle</param>
        /// <param name="err">The result of receiving the message.</param>
        /// <param name="message">The message.</param>
        private void MessageReceivedHandler(IntPtr connection_handle, int err, string message)
        {
            AgentConnection connection;
            _connections.TryGetValue(connection_handle, out connection);

            if (connection == null)
                return;

            connection.AddMessageReceivedEvent(connection_handle, err, message);
        }

        /// <summary>
        /// Initializes a new Listener with the specified handle.
        /// </summary>
        internal AgentListener()
        {
            MessageReceivedCallback = MessageReceivedHandler;
            ConnectionEstablishedCallback = ConnectionEstablishedHandler;
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
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_add_identity(
                commandHandle,
                Handle,
                pool.Handle,
                wallet.Handle,
                did,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Removes an identity from the listener.
        /// </summary>
        /// <param name="wallet">The wallet that contains the identity.</param>
        /// <param name="did">The DID of the identity to remove. </param>
        /// <returns>An asynchronous task that returns no value.</returns>
        public Task RemoveIdentityAsync(Wallet wallet, String did)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_remove_identity(
                commandHandle,
                Handle,
                wallet.Handle,
                did,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Waits for a connection to be established with the listener.
        /// </summary>
        /// <returns>An asynchronous task that resolves to ConnectionEvent when a connection is established.</returns>
        public Task<AgentConnectionEvent> WaitForConnection()
        {
            var taskCompletionSource = new TaskCompletionSource<AgentConnectionEvent>();
            var tuple = Tuple.Create(Handle, taskCompletionSource);
            _eventWaiters.Add(tuple);
            NotifyEventWaiters();

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Closes the listener.
        /// </summary>
        /// <returns>An asynchronous task that returns no value.</returns>
        public Task CloseAsync()
        {
            _isOpen = false;
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_close_listener(
                commandHandle,
                Handle,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Disposes of resources used by the listener.
        /// </summary>
        public void Dispose()
        {
            if (_isOpen)
                CloseAsync().Wait();
        }
    }
}
