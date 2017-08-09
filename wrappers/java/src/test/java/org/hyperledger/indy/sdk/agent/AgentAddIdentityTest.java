package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.agent.Agent.Listener;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.Test;


public class AgentAddIdentityTest extends AgentIntegrationTest {

	@Test
	public void testAgentAddIdentityWorks() throws Exception {
		String endpoint = "127.0.0.1:9601";

		SignusResults.CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(wallet, "{}").get();

		Listener activeListener = Agent.agentListen(endpoint, incomingConnectionObserver).get();

		activeListener.agentAddIdentity(pool, wallet, myDid.getDid()).get();
	}

	@Test
	public void testAgentAddIdentityWorksForMultiplyKeys() throws Exception {
		String endpoint = "127.0.0.1:9602";

		SignusResults.CreateAndStoreMyDidResult myDid1 = Signus.createAndStoreMyDid(wallet, "{}").get();
		SignusResults.CreateAndStoreMyDidResult myDid2 = Signus.createAndStoreMyDid(wallet, "{}").get();

		SignusResults.CreateAndStoreMyDidResult[] dids = {myDid1, myDid2};

		Listener activeListener = Agent.agentListen(endpoint, incomingConnectionObserver).get();

		for (SignusResults.CreateAndStoreMyDidResult did : dids) {
			activeListener.agentAddIdentity(pool, wallet, did.getDid()).get();
		}
	}
}