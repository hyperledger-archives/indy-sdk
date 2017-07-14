package org.hyperledger.indy.sdk.agent;

/**
 * agent.rs observers
 */
public final class AgentObservers {

	private AgentObservers() {

	}

	public interface ConnectionObserver {

		public MessageObserver onConnection(Agent.Listener listener, Agent.Connection connection, String senderDid, String receiverDid);
	}

	public interface MessageObserver {

		public void onMessage(Agent.Connection connection, String message);
	}
}
