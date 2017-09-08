namespace Hyperledger.Indy.AgentApi
{
    /// <summary>
    /// Event raised when a connection is established on an <see cref="AgentListener"/>.
    /// </summary>
    /// <remarks>
    /// The AgentConnectionEvent is raised asynchronously when an agent establishes a connection on 
    /// an <see cref="AgentListener"/> that is listening for incoming connections.  These
    /// events are queued and events for a specific listener can be obtained by calling the listener's 
    /// <see cref="AgentListener.WaitForConnection"/> method.   
    /// </remarks>
    public sealed class AgentConnectionEvent : AgentEvent
    {
        /// <summary>
        /// Initializes a new AgentConnectionEvent.
        /// </summary>
        /// <param name="listener">The listener the connection was established on.</param>
        /// <param name="result">The result of the opening the connection.</param>
        /// <param name="connection">The connection.</param>
        /// <param name="senderDid">The DID of the sender.</param>
        /// <param name="receiverDid">The DID of the receiver.</param>
        internal AgentConnectionEvent(AgentListener listener, ErrorCode result, AgentConnection connection, string senderDid, string receiverDid) :
            base(listener.Handle, result)
        {
            Connection = connection;
            SenderDid = senderDid;
            ReceiverDid = receiverDid;
            Listener = listener;
        }

        /// <summary>
        /// Gets the listener the connection was received on.
        /// </summary>
        public AgentListener Listener { get; }

        /// <summary>
        /// Gets the connection that was established.
        /// </summary>
        public AgentConnection Connection { get; }

        /// <summary>
        /// Gets the DID of the sender that initiated the connection.
        /// </summary>
        public string SenderDid { get; }

        /// <summary>
        /// Gets the DID of the receiver of the connection.
        /// </summary>
        public string ReceiverDid { get; }
    }
}
