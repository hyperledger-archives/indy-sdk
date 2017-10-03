namespace Hyperledger.Indy.AgentApi
{
    /// <summary>
    /// Event raised when a message is received on an <see cref="AgentConnection"/>.
    /// </summary>
    /// <remarks>
    /// <para>The AgentMessageEvent is raised asynchronously when a message is received on 
    /// an <see cref="AgentConnection"/>.  These events are queued and events for a specific 
    /// connection can be obtained by calling the connection's <see cref="AgentConnection.WaitForMessageAsync"/>
    /// method.
    /// </para>
    /// <note type="note">Messages received on a connection arrived encrypted, however the <see cref="Message"/> 
    /// property of the event contains the decrypted message content.
    /// </note>
    /// </remarks>
    public sealed class AgentMessageEvent : AgentEvent
    {
        /// <summary>
        /// Initializes a new AgentMessageEvent.
        /// </summary>
        /// <param name="connection">The connection the message was received on.</param>
        /// <param name="result">The result of receiving the message.</param>
        /// <param name="message">The message.</param>
        internal AgentMessageEvent(AgentConnection connection, ErrorCode result, string message) :
            base(connection.Handle, result)
        {
            Connection = connection;
            Message = message;
        }

        /// <summary>
        /// Gets the connection the message was received on.
        /// </summary>
        public AgentConnection Connection { get; }

        /// <summary>
        /// Gets the decrypted content of the received message.
        /// </summary>
        public string Message { get; }
    }
}
