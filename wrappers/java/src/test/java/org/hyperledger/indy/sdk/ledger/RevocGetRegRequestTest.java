package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.junit.Test;

import static org.junit.Assert.assertTrue;

public class RevocGetRegRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildGetRevocRegRequestWorks() throws Exception {
		String expectedResult =
				"\"operation\": {\n" +
						"            \"type\": \"116\",\n" +
						"            \"revocRegDefId\": \"RevocRegID\",\n" +
						"            \"timestamp\": 100\n" +
						"        }";

		String request = Ledger.buildGetRevocRegRequest(DID, "RevocRegID", 100).get();

		assertTrue(request.replaceAll("\\s+", "").contains(expectedResult.replaceAll("\\s+", "")));
	}
}
