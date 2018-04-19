package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.junit.Test;

import static org.junit.Assert.assertTrue;

public class GetRevocRegDefRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildGetRevocRegDefRequestWorks() throws Exception {
		String expectedResult = "\"operation\":{\"type\":\"115\",\"id\":\"RevocRegID\"}";

		String request = Ledger.buildGetRevocRegDefRequest(DID, "RevocRegID").get();

		assertTrue(request.replaceAll("\\s+","").contains(expectedResult.replaceAll("\\s+","")));
	}
}
