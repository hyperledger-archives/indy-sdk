package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.junit.Test;

import static org.junit.Assert.assertTrue;

public class RevocGetRegDeltaRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildGetRevocRegDeltaRequestWorks() throws Exception {
		String expectedResult =
				"\"operation\": {\n" +
						"            \"type\": \"117\",\n" +
						"            \"revocRegDefId\": \"RevocRegID\",\n" +
						"            \"to\": 100\n" +
						"        }";

		String request = Ledger.buildGetRevocRegDeltaRequest(DID, "RevocRegID", - 1, 100).get();

		assertTrue(request.replaceAll("\\s+", "").contains(expectedResult.replaceAll("\\s+", "")));
	}
}
