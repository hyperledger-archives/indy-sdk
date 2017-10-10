package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.agent.Agent.Connection;
import org.hyperledger.indy.sdk.agent.Agent.Listener;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.Test;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;


public class AgentCloseConnectionTest extends AgentIntegrationTest {

	private static CompletableFuture<Connection> serverToClientConnectionFuture = new CompletableFuture<Connection>();

	private static final AgentObservers.ConnectionObserver incomingConnectionObserver = new AgentObservers.ConnectionObserver() {

		public AgentObservers.MessageObserver onConnection(Listener listener, Connection connection, String senderDid, String receiverDid) {

			System.out.println("New connection " + connection);

			serverToClientConnectionFuture.complete(connection);

			return messageObserverForIncoming;
		}
	};

	@Test
	public void testAgentCloseConnectionWorksForOutgoing() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String endpoint = "127.0.0.1:9603";

		SignusResults.CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(wallet, "{}").get();

		String identityJson = String.format(AGENT_IDENTITY_JSON_TEMPLATE, myDid.getDid(), myDid.getPk(), myDid.getVerkey(), endpoint);
		Signus.storeTheirDid(wallet, identityJson).get();

		Listener activeListener = Agent.agentListen(endpoint, incomingConnectionObserver).get();

		activeListener.agentAddIdentity(pool, wallet, myDid.getDid()).get();

		Connection connection = Agent.agentConnect(pool, wallet, myDid.getDid(), myDid.getDid(), messageObserver).get();

		connection.agentCloseConnection().get();

		connection.agentSend("msg").get();
	}

	@Test
	public void testAgentCloseConnectionWorksForIncoming() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String endpoint = "127.0.0.1:9613";

		SignusResults.CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(wallet, "{}").get();

		String identityJson = String.format(AGENT_IDENTITY_JSON_TEMPLATE, myDid.getDid(), myDid.getPk(), myDid.getVerkey(), endpoint);
		Signus.storeTheirDid(wallet, identityJson).get();

		Listener activeListener = Agent.agentListen(endpoint, incomingConnectionObserver).get();

		activeListener.agentAddIdentity(pool, wallet, myDid.getDid()).get();

		Agent.agentConnect(pool, wallet, myDid.getDid(), myDid.getDid(), messageObserver).get();

		Connection serverToClientConnection = serverToClientConnectionFuture.get();

		serverToClientConnection.agentCloseConnection().get();

		serverToClientConnection.agentSend("msg").get();
	}
}