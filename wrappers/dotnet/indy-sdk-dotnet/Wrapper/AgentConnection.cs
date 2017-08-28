using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.IndyNativeMethods;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// A connection to an agent.
    /// </summary>
    public sealed class AgentConnection : AsyncWrapperBase, IDisposable
    {      
        /// <summary>
        /// Pending message events.
        /// </summary>
        private static IList<AgentMessageEvent> _events = new List<AgentMessageEvent>();

        /// <summary>
        /// Tasks waiting on message events.
        /// </summary>
        private static IList<Tuple<IntPtr, TaskCompletionSource<AgentMessageEvent>>> _eventWaiters = new List<Tuple<IntPtr, TaskCompletionSource<AgentMessageEvent>>>();

        /// <summary>
        /// Callback to use when an outgoing connection is established.
        /// </summary>
        private static AgentConnectionEstablishedDelegate _connectionEstablishedCallback = (xcommand_handle, err, connection_handle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<AgentConnection>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var connection = (AgentConnection)taskCompletionSource.Task.AsyncState;
            connection.Handle = connection_handle;

            taskCompletionSource.SetResult(connection);

        };

        /// <summary>
        /// Creates a connection to an agent.
        /// </summary>
        /// <param name="pool">The ledger pool that the destination DID is registered on.</param>
        /// <param name="wallet">The wallet containing the keys for the DIDs.</param>
        /// <param name="senderDid">The DID to use when initiating the connection.</param>
        /// <param name="receiverDid">The DID of the target of the connection.</param>
        /// <returns>An asynchronous task that returns a Connection result.</returns>
        public static Task<AgentConnection> ConnectAsync(Pool pool, Wallet wallet, string senderDid, string receiverDid)
        {
            var connection = new AgentConnection();

            var taskCompletionSource = new TaskCompletionSource<AgentConnection>(connection);
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_connect(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                senderDid,
                receiverDid,
                _connectionEstablishedCallback,
                connection.MessageReceivedCallback);

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
        /// Whether or not the connection is open.
        /// </summary>
        private bool _isOpen = true;

        /// <summary>
        /// Gets the handle for the connection.
        /// </summary>
        public IntPtr Handle { get; private set; }

        /// <summary>
        /// A reference to the callback that will handle received messages for the connection.
        /// </summary>
        private AgentMessageReceivedDelegate MessageReceivedCallback { get; }

        /// <summary>
        /// Initializes a new AgentConnection.
        /// </summary>
        internal AgentConnection()
        {
            MessageReceivedCallback = AddMessageReceivedEvent;
        }

        /// <summary>
        /// Initializes a new connection.
        /// </summary>
        /// <param name="handle">The handle for the connection.</param>
        internal AgentConnection(IntPtr handle) : this()
        {
            Handle = handle;
        }

        /// <summary>
        /// Adds an event for the connection.
        /// </summary>
        /// <param name="connection_handle">The handle for the connection.</param>
        /// <param name="err">The result of receiving the message.</param>
        /// <param name="message">The message.</param>
        internal void AddMessageReceivedEvent(IntPtr connection_handle, int err, string message)
        {
            _events.Add(new AgentMessageEvent(this, (ErrorCode)err, message));
            NotifyEventWaiters();
        }

        /// <summary>
        /// Sends a message to the connection.
        /// </summary>
        /// <param name="message">The message to send.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        public Task SendAsync(string message)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_send(
                commandHandle,
                Handle,
                message,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Waits for a message from the connection.
        /// </summary>
        /// <returns>An asynchronous task that resolves to MessageEvent when a message arrives.</returns>
        public Task<AgentMessageEvent> WaitForMessage()
        {
            var taskCompletionSource = new TaskCompletionSource<AgentMessageEvent>();
            var tuple = Tuple.Create(Handle, taskCompletionSource);
            _eventWaiters.Add(tuple);
            NotifyEventWaiters();

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Closes the connection.
        /// </summary>
        /// <returns>An asynchronous task that returns no value.</returns>
        public Task CloseAsync()
        {
            _isOpen = false;
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_close_connection(
                commandHandle,
                Handle,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Disposes of resources used by the connection.
        /// </summary>
        public void Dispose()
        {
            if (_isOpen)
                CloseAsync().Wait();
        }        
    }
}
