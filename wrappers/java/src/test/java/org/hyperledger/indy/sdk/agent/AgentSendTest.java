package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.agent.Agent.Connection;
import org.hyperledger.indy.sdk.agent.Agent.Listener;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.Test;

import java.util.concurrent.CompletableFuture;

import static org.junit.Assert.assertEquals;

public class AgentSendTest extends AgentIntegrationTest {

	private static CompletableFuture<Connection> serverToClientConnectionFuture = new CompletableFuture<Connection>();
	private static CompletableFuture<String> serverToClientMsgFuture = new CompletableFuture<String>();
	private static CompletableFuture<String> clientToServerMsgFuture = new CompletableFuture<String>();

	private static final AgentObservers.MessageObserver messageObserver = new AgentObservers.MessageObserver() {

		public void onMessage(Connection connection, String message) {

			System.out.println("Received message '" + message + "' on connection " + connection);

			serverToClientMsgFuture.complete(message);
		}
	};

	private static final AgentObservers.MessageObserver messageObserverForIncoming = new AgentObservers.MessageObserver() {

		public void onMessage(Connection connection, String message) {

			System.out.println("Received message '" + message + "' on incoming connection " + connection);

			clientToServerMsgFuture.complete(message);
		}
	};

	private static final AgentObservers.ConnectionObserver incomingConnectionObserver = new AgentObservers.ConnectionObserver() {

		public AgentObservers.MessageObserver onConnection(Listener listener, Connection connection, String senderDid, String receiverDid) {

			System.out.println("New connection " + connection);

			serverToClientConnectionFuture.complete(connection);

			return messageObserverForIncoming;
		}
	};

	@Test
	public void testAgentSendWorksForAllDataInWalletPresent() throws Exception {
		String endpoint = "127.0.0.1:9609";

		SignusResults.CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(wallet, "{}").get();

		String identityJson = String.format(AGENT_IDENTITY_JSON_TEMPLATE, myDid.getDid(), myDid.getPk(), myDid.getVerkey(), endpoint);
		Signus.storeTheirDid(wallet, identityJson).get();

		Listener activeListener = Agent.agentListen(endpoint, incomingConnectionObserver).get();

		activeListener.agentAddIdentity(pool, wallet, myDid.getDid()).get();

		Connection clientToServerConnection = Agent.agentConnect(pool, wallet, myDid.getDid(), myDid.getDid(), messageObserver).get();

		String clientToServerMessage = "msg_from_client";
		String serverToClientMessage = "msg_from_server";

		clientToServerConnection.agentSend(clientToServerMessage).get();

		assertEquals(clientToServerMessage, clientToServerMsgFuture.get());

		Connection serverToClientConnection = serverToClientConnectionFuture.get();
		serverToClientConnection.agentSend(serverToClientMessage).get();

		assertEquals(serverToClientMessage, serverToClientMsgFuture.get());
	}
}