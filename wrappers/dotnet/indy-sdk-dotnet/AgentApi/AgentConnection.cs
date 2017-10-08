using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using static Hyperledger.Indy.IndyNativeMethods;

namespace Hyperledger.Indy.AgentApi
{
    /// <summary>
    /// Represents a connection between two agents and exposes static methods for opening an outbound
    /// connection to an agent.
    /// </summary>
    /// <remarks>
    /// <para>AgentConnection instances are created in one of two ways; by using the 
    /// <see cref="ConnectAsync"/> static method of this class to establish an outbound connection or when
    /// an <see cref="AgentListener"/> receives an incoming connection.
    /// </para>
    /// <para>Messages received on a connection result in an <see cref="AgentMessageEvent"/> being raised
    /// asynchronously for each message and these events can be obtained by calling the 
    /// <see cref="WaitForMessageAsync"/> method, which will return a <see cref="Task{AgentMessageEvent}"/> that will resolve to
    /// the first received event.  
    /// </para>
    /// <para>When a connection is no longer required it must be closed using its <see cref="CloseAsync"/> 
    /// method.  AgentConnection instances will automatically be closed when the connection is disposed 
    /// so ideally the connection will be opened in a <c>using</c> block.
    /// </para>
    /// </remarks>
    /// <seealso cref="AgentListener"/>
    /// <seealso cref="AgentMessageEvent"/>
    public sealed class AgentConnection : IDisposable
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
            var taskCompletionSource = PendingCommands.Remove<AgentConnection>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var connection = (AgentConnection)taskCompletionSource.Task.AsyncState;
            connection.Handle = connection_handle;
            connection._requiresClose = true;

            taskCompletionSource.SetResult(connection);
        };

        /// <summary>
        /// Creates a new outgoing connection.
        /// </summary>
        /// <remarks>
        /// <para>When establishing a connection two DIDs are required, one for the sender identity (who is
        /// initiating the connection) and the other for the receiver identity (who the connection is being 
        /// established with).
        /// </para>
        /// <para>The <see cref="Wallet"/> provided when creating the connection must contain information about
        /// the sender identity which must have been added using the <see cref="Signus.CreateAndStoreMyDidAsync(Wallet, string)"/> 
        /// method prior to attempting to create the connection.
        /// </para>
        /// <para>The identity information for the receiver can also be stored in the wallet using
        /// the <see cref="Signus.StoreTheirDidAsync(Wallet, string)"/> method, however if no record is
        /// present in the wallet the identity information will be established from the ledger in the 
        /// provided node <see cref="Pool"/> and will automatically be cached in the provided wallet.
        /// </para>
        /// </remarks>
        /// <seealso cref="Pool"/>
        /// <seealso cref="Wallet"/>
        /// <seealso cref="Signus"/>
        /// <param name="pool">The node pool that the destination DID is registered on.</param>
        /// <param name="wallet">The wallet containing the sender (and optionally receiver) DID.</param>
        /// <param name="senderDid">The DID of the identity imitating the connection.</param>
        /// <param name="receiverDid">The DID of the identity the connection is to be established with.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an AgentConnection instance 
        /// when the connection has been established.</returns>
        public static Task<AgentConnection> ConnectAsync(Pool pool, Wallet wallet, string senderDid, string receiverDid)
        {
            var connection = new AgentConnection();

            var taskCompletionSource = new TaskCompletionSource<AgentConnection>(connection);
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_connect(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                senderDid,
                receiverDid,
                _connectionEstablishedCallback,
                connection.MessageReceivedCallback);

            CallbackHelper.CheckResult(result);

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
        /// Whether or not the close function has been called.
        /// </summary>
        private bool _requiresClose = false;

        /// <summary>
        /// Gets the handle for the connection.
        /// </summary>
        internal IntPtr Handle { get; private set; }

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
        /// <remarks>
        /// <note type="note">Messages sent to a connection are automatically encrypted for the receiver 
        /// prior to sending.
        /// </note>
        /// </remarks>
        /// <param name="message">The message to send.</param>
        /// <returns>An asynchronous <see cref="Task"/> completes once the operation completes.</returns>
        public Task SendAsync(string message)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_send(
                commandHandle,
                Handle,
                message,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Waits to receive a message on the connection.
        /// </summary>
        /// <remarks>
        /// Messages received on a connection result in an <see cref="AgentMessageEvent"/> being raised
        /// asynchronously and these events will be queued.  Calling this method will result in a 
        /// <see cref="Task{AgentMessageEvent}"/> that will resolve to the first event received for the 
        /// current connection when it arrives, or if the event has already been raised, will resolve 
        /// immediately.  If subsequent events have already been queued this method can be called again 
        /// repeatedly until the queue is empty, at which point it will continue to wait until more 
        /// messages arrive on the connection.
        /// </remarks>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an
        /// <see cref="AgentMessageEvent"/> when a message is received.</returns>
        public Task<AgentMessageEvent> WaitForMessageAsync()
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
        /// <remarks>
        /// Once closed a connection instance cannot be re-opened; a new connection instance must be
        /// created.
        /// </remarks>
        /// <returns>An asynchronous <see cref="Task"/> completes once the operation completes.</returns>
        public Task CloseAsync()
        {
            _requiresClose = false;

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_close_connection(
                commandHandle,
                Handle,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            
            GC.SuppressFinalize(this);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Disposes of resources.
        /// </summary>
        public async void Dispose()
        {
            if (_requiresClose)
                await CloseAsync();
        }

        /// <summary>
        /// Finalizes the resource during GC if it hasn't been already.
        /// </summary>
        ~AgentConnection()
        {
            if (_requiresClose)
            {
                IndyNativeMethods.indy_agent_close_connection(
                   -1,
                   Handle,
                   CallbackHelper.NoValueCallback
                );
            }
        }
    }
}
