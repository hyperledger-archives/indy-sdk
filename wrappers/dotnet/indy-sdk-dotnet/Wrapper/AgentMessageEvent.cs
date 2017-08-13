namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Event raised when a message is received.
    /// </summary>
    public sealed class AgentMessageEvent : AgentEvent
    {
        /// <summary>
        /// Initializes a new MessageEvent.
        /// </summary>
        /// <param name="connection">The connection the message was recevied on.</param>
        /// <param name="result">The result of receiving the message.</param>
        /// <param name="message">The message.</param>
        public AgentMessageEvent(AgentConnection connection, ErrorCode result, string message) :
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
        /// Gets the received message.
        /// </summary>
        public string Message { get; }
    }
}
