using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;
using static Hyperledger.Indy.IndyNativeMethods;

namespace Hyperledger.Indy.AgentApi
{
    /// <summary>
    /// Represents a listener that can receive incoming connections from an agent and exposes 
    /// static methods creating listener instances.
    /// </summary>
    /// <remarks>
    /// <para>AgentListener instances are created using the <see cref="ListenAsync(string)"/> static 
    /// method of this class, however the new listener cannot receive connections until at least one 
    /// identity has been added to the listener using its the <see cref="AddIdentityAsync(Pool, Wallet, string)"/>
    /// method.
    /// </para>
    /// <para>When an open listener receives an incoming connection an <see cref="AgentConnectionEvent"/> 
    /// is raised asynchronously and these events can be obtained by calling the <see cref="WaitForConnectionAsync"/> 
    /// method on the listener instance, which will return a <see cref="Task{AgentConnectionEvent}"/> that will resolve to
    /// the first received event for that listener.  
    /// </para>
    /// <para>When a listener is no longer required it must be closed using its <see cref="CloseAsync"/> 
    /// method.  AgentListener instances will automatically be closed when the listener is disposed 
    /// so ideally the listener will be opened in a <c>using</c> block.
    /// </para>
    /// </remarks>
    /// <seealso cref="AgentConnection"/>
    /// <seealso cref="AgentConnectionEvent"/>
    public sealed class AgentListener : IDisposable
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
            var taskCompletionSource = PendingCommands.Remove<AgentListener>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var listener = (AgentListener)taskCompletionSource.Task.AsyncState;
            listener.Handle = listener_handle;

            taskCompletionSource.SetResult(listener);
        };

        /// <summary>
        /// Creates a new AgentListener that listens for incoming connections on the specified endpoint.
        /// </summary>
        /// <remarks>
        /// The endpoint specified must be in the format <c>address:port</c> where <c>address</c> is
        /// an IP address or host address and <c>port</c> is a numeric port number.
        /// </remarks>
        /// <param name="endpoint">The endpoint on which the incoming connections will listened for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an AgentListener instance 
        /// once the listener has been created.</returns>
        public static Task<AgentListener> ListenAsync(string endpoint)
        {
            var listener = new AgentListener();

            var taskCompletionSource = new TaskCompletionSource<AgentListener>(listener);
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_listen(
                commandHandle,
                endpoint,
                _listenerCreatedCallback,
                listener.ConnectionEstablishedCallback,
                listener.MessageReceivedCallback);

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
        /// Connections opened on the listener.
        /// </summary>
        private IDictionary<IntPtr, AgentConnection> _connections = new ConcurrentDictionary<IntPtr, AgentConnection>();

        /// <summary>
        /// Whether or not the close function has been called.
        /// </summary>
        private bool _closeRequested = false;

        /// <summary>
        /// Gets the handle for the listener.
        /// </summary>
        internal IntPtr Handle { get; private set; }

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
        /// <remarks>
        /// <para>Although an AgentListner instance can listen for incoming connections on a specified
        /// endpoint, any incoming connection to an identity not associated with the listener will be
        /// automatically rejected.  This method adds an identity to the listener that will be authorized 
        /// to accept connections.
        /// </para>
        /// <para>This method will perform a <see cref="Wallet"/> lookup to find the identity information 
        /// for the DID to add and consequently the DID must have already been saved in the wallet using 
        /// the <see cref="Hyperledger.Indy.SignusApi.CreateAndStoreMyDidResult"/> method prior to attempting to
        /// add it to the listener.
        /// </para>
        /// <para>Authorization to accept incoming connections to a DID on a listener can be removed using
        /// the <see cref="RemoveIdentityAsync(Wallet, string)"/> method.
        /// </para>
        /// </remarks>
        /// <seealso cref="Signus"/>
        /// <param name="pool">The node pool that will be used to verify the identity.</param>
        /// <param name="wallet">The Wallet that contains the identity.</param>
        /// <param name="did">The DID of the identity to authorize connections to.</param>
        /// <returns>An asynchronous <see cref="Task"/> completes once the operation completes.</returns>
        public Task AddIdentityAsync(Pool pool, Wallet wallet, string did)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_add_identity(
                commandHandle,
                Handle,
                pool.Handle,
                wallet.Handle,
                did,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Removes an identity from the listener.
        /// </summary>
        /// <remarks>
        /// <para>Once an identity has been added to an AgentListner using the <see cref="AddIdentityAsync(Pool, Wallet, string)"/>
        /// it can be removed using this method. A <see cref="Wallet"/> lookup will be performed to find 
        /// the identity information for the DID so the wallet containing the DID must be provided.
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet that contains the identity for the DID.</param>
        /// <param name="did">The DID of the identity to remove. </param>
        /// <returns>An asynchronous <see cref="Task"/> completes once the operation completes.</returns>
        public Task RemoveIdentityAsync(Wallet wallet, string did)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_remove_identity(
                commandHandle,
                Handle,
                wallet.Handle,
                did,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Waits to receive a connection on the listener.
        /// </summary>
        /// <remarks>Connections received on a listener result in an <see cref="AgentConnectionEvent"/> 
        /// being raised asynchronously and these events will be queued.  Calling this method will result 
        /// in a <see cref="Task{AgentConnectionEvent}"/> that will resolve to the first event received for the listener
        /// when it arrives, or if the event has already been raised, will resolve immediately.  If 
        /// subsequent events have already been queued this method can be called again repeatedly until 
        /// the queue is empty, at which point it will continue to wait until more connections are received
        /// by the listener.
        /// </remarks>
        /// <seealso cref="AgentConnection"/>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an 
        /// <see cref="AgentConnectionEvent"/> when a connection is established.</returns>
        public Task<AgentConnectionEvent> WaitForConnectionAsync()
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
        /// <remarks>
        /// <para>When a listener is closed it stops listening for new incoming connections and
        /// all incoming <see cref="AgentConnection"/> instances accepted by the listener are also 
        /// closed.
        /// </para>
        /// <para>Once closed a listener cannot be re-opened; a new listener instance must instead be 
        /// created using the <see cref="ListenAsync(string)"/> static method.
        /// </para>
        /// </remarks>
        /// <returns>An asynchronous <see cref="Task"/> completes once the operation completes.</returns>
        public Task CloseAsync()
        {
            if (_closeRequested)
                return Task.FromResult(true);

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = IndyNativeMethods.indy_agent_close_listener(
                commandHandle,
                Handle,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            _closeRequested = true;
            GC.SuppressFinalize(this);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Disposes of resources.
        /// </summary>
        public async void Dispose()
        {
            if (!_closeRequested)
                await CloseAsync();
        }

        /// <summary>
        /// Finalizes the resource during GC if it hasn't been already.
        /// </summary>
        ~AgentListener()
        {
            if (!_closeRequested)
            {
                IndyNativeMethods.indy_agent_close_listener(
                    -1,
                    Handle,
                    CallbackHelper.NoValueCallback
                );
            }
        }
    }
}
