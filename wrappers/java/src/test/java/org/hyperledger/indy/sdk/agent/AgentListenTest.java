package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.Test;


public class AgentListenTest extends AgentIntegrationTest {

	@Test
	public void testAgentListenWorksForAllDataInWalletPresent() throws Exception {
		String endpoint = "127.0.0.1:9607";

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "indy_agent_connect_works_for_aaa", null, null);

		SignusResults.CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();

		String identityJson = String.format("{\"did\":\"%s\", \"pk\":\"%s\", \"verkey\":\"%s\", \"endpoint\":\"%s\"}",
				myDid.getDid(), myDid.getPk(), myDid.getVerkey(), endpoint);
		Signus.storeTheirDid(wallet, identityJson).get();

		Agent.agentListen(endpoint, incomingConnectionObserver).get();
	}
}