package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.agent.Agent.Connection;
import org.hyperledger.indy.sdk.agent.Agent.Listener;


class AgentIntegrationTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	protected static final String AGENT_IDENTITY_JSON_TEMPLATE = "{\"did\":\"%s\", \"pk\":\"%s\", \"verkey\":\"%s\", \"endpoint\":\"%s\"}";

	static final AgentObservers.MessageObserver messageObserver = new AgentObservers.MessageObserver() {

		public void onMessage(Connection connection, String message) {

			System.out.println("Received message '" + message + "' on connection " + connection);
		}
	};

	static final AgentObservers.MessageObserver messageObserverForIncoming = new AgentObservers.MessageObserver() {

		public void onMessage(Connection connection, String message) {

			System.out.println("Received message '" + message + "' on incoming connection " + connection);
		}
	};

	static final AgentObservers.ConnectionObserver incomingConnectionObserver = new AgentObservers.ConnectionObserver() {

		public AgentObservers.MessageObserver onConnection(Listener listener, Connection connection, String senderDid, String receiverDid) {

			System.out.println("New connection " + connection);

			return messageObserverForIncoming;
		}
	};
}