package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.Test;


public class AgentListenTest extends AgentIntegrationTest {

	@Test
	public void testAgentListenWorksForAllDataInWalletPresent() throws Exception {
		String endpoint = "127.0.0.1:9607";

		SignusResults.CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(wallet, "{}").get();

		String identityJson = String.format(AGENT_IDENTITY_JSON_TEMPLATE, myDid.getDid(), myDid.getPk(), myDid.getVerkey(), endpoint);
		Signus.storeTheirDid(wallet, identityJson).get();

		Agent.agentListen(endpoint, incomingConnectionObserver).get();
	}
}