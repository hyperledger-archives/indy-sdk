package org.hyperledger.indy.sdk.agent;

/**
 * agent.rs observers
 */
/**
 * Observer interfaces for agents.
 */
public final class AgentObservers {

	private AgentObservers() {

	}

	/**
	 * An observer to receive notification of the establishment of a connection.
	 */
	public interface ConnectionObserver {

		/**
		 * Called when a connection is established.
		 * 
		 * @param listener The listener the connection is associated with.  Can be null for outgoing connections.
		 * @param connection The connection that has been established.
		 * @param senderDid The DID of the identity that initated the connection.
		 * @param receiverDid The DID of the identity that received the connection.
		 * @return A MessageObserver that will receive notifications when the connection receives a message.
		 */
		public MessageObserver onConnection(Agent.Listener listener, Agent.Connection connection, String senderDid, String receiverDid);
	}

	/**
	 * An observer to receive notification of the receipt of a message.
	 */
	public interface MessageObserver {

		/**
		 * Called when a message is received.
		 * 
		 * @param connection The connection the message was received on.
		 * @param message The received message.
		 */
		public void onMessage(Agent.Connection connection, String message);
	}
}
