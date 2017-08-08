package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.agent.Agent.Listener;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.Test;


public class AgentRemoveIdentityTest extends AgentIntegrationTest {

	@Test
	public void testAgentRemoveIdentityWorks() throws Exception {
		String endpoint = "127.0.0.1:9608";

		SignusResults.CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(wallet, "{}").get();

		Listener activeListener = Agent.agentListen(endpoint, incomingConnectionObserver).get();

		activeListener.agentAddIdentity(pool, wallet, myDid.getDid()).get();

		activeListener.agentRemoveIdentity(wallet, myDid.getDid()).get();
	}
}