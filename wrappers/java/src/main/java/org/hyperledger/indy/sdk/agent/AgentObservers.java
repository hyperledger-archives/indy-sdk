package org.hyperledger.indy.sdk.agent;

/**
 * agent.rs observers
 */
public final class AgentObservers {

	private AgentObservers() {

	}

	public interface AgentConnectObserver {

		public void onMessage(Agent.Connection connection, String message);
	}

	public interface AgentListenObserver extends AgentConnectObserver {

		public void onConnection(Agent.Listener listener, Agent.Connection connection, String senderDid, String receiverDid);
		public void onMessage(Agent.Connection connection, String message);
	}
}
