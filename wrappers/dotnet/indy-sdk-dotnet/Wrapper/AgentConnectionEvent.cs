namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Event used when a connection is established on a listener.
    /// </summary>
    public sealed class AgentConnectionEvent : AgentEvent
    {
        /// <summary>
        /// Initializes a new ConnectionEvent.
        /// </summary>
        /// <param name="listener">The listener the connection was established on.</param>
        /// <param name="result">The result of the opening the connection.</param>
        /// <param name="connection">The connection.</param>
        /// <param name="senderDid">The DID of the sender.</param>
        /// <param name="receiverDid">The DID of the receiver.</param>
        public AgentConnectionEvent(AgentListener listener, ErrorCode result, AgentConnection connection, string senderDid, string receiverDid) :
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
        public AgentListener Listener { get; }

        /// <summary>
        /// Gets the connection.
        /// </summary>
        public AgentConnection Connection { get; }

        /// <summary>
        /// Gets the DID of the sender.
        /// </summary>
        public string SenderDid { get; }

        /// <summary>
        /// Gets the DID of the receiver.
        /// </summary>
        public string ReceiverDid { get; }
    }
}
