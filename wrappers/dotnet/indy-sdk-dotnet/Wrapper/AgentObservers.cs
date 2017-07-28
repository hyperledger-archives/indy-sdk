﻿namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Observer classes for use with the Agent API.
    /// </summary>
    public sealed class AgentObservers
    {
        /// <summary>
        /// Prevents class being instantiated.
        /// </summary>
        private AgentObservers()
        {
        }

        /// <summary>
        /// Interface for handling the event raised on establishment of a connection.
        /// </summary>
        public interface ConnectionObserver
        {
            /// <summary>
            /// Called when a connection is established by a remote party.
            /// </summary>
            /// <param name="listener">The listener the connection was established on.</param>
            /// <param name="connection">The connection that was created.</param>
            /// <param name="senderDid">The DID of the sender.</param>
            /// <param name="receiverDid">The DID of the receiver.</param>
            /// <returns>A MessageObserver that will receive notifications when messages are received on the connection.</returns>
            MessageObserver OnConnection(Agent.Listener listener, Agent.Connection connection, string senderDid, string receiverDid);
        }

        /// <summary>
        /// Interface for handling the event raised when a message is received.
        /// </summary>
        public interface MessageObserver
        {
            /// <summary>
            /// Called when a message is received.
            /// </summary>
            /// <param name="connection">The connection the message was received on.</param>
            /// <param name="message">The content of the received message.</param>
            void OnMessage(Agent.Connection connection, string message);
        }
    }
}
