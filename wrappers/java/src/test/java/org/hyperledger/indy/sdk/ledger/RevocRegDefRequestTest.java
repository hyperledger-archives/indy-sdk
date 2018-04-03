package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.junit.Test;

import static org.junit.Assert.assertTrue;

public class RevocRegDefRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildRevocRegDefRequestWorks() throws Exception {
		String expectedResult =
				"\"operation\":{" +
						"\"credDefId\":\"CredDefID\"," +
						"\"id\":\"RevocRegID\"," +
						"\"revocDefType\":\"CL_ACCUM\"," +
						"\"tag\":\"TAG1\"," +
						"\"type\":\"113\"," +
						"\"value\":{" +
						"   \"issuanceType\":\"ISSUANCE_ON_DEMAND\"," +
						"   \"maxCredNum\":5," +
						"   \"publicKeys\":{" +
						"       \"accumKey\":{\"z\":\"\"}" +
						"   }," +
						"   \"tailsHash\":\"s\"," +
						"   \"tailsLocation\":\"http://tails.location.com\"" +
						"}}";

		String data = "{\n" +
				"        \"id\": \"RevocRegID\",\n" +
				"        \"revocDefType\": \"CL_ACCUM\",\n" +
				"        \"tag\": \"TAG1\",\n" +
				"        \"credDefId\": \"CredDefID\",\n" +
				"        \"value\": {\n" +
				"            \"issuanceType\": \"ISSUANCE_ON_DEMAND\",\n" +
				"            \"maxCredNum\": 5,\n" +
				"            \"tailsHash\": \"s\",\n" +
				"            \"tailsLocation\": \"http://tails.location.com\",\n" +
				"            \"publicKeys\": {\n" +
				"                \"accumKey\": {\n" +
				"                    \"z\": \"\"\n" +
				"                }\n" +
				"            }\n" +
				"        }\n" +
				"    }";

		String request = Ledger.buildRevocRegDefRequest(DID, data).get();

		assertTrue(request.replaceAll("\\s+","").contains(expectedResult.replaceAll("\\s+","")));
	}
}
