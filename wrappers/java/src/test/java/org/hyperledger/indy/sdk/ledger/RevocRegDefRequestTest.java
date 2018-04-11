package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Test;

import static org.junit.Assert.assertTrue;

public class RevocRegDefRequestTest extends LedgerIntegrationTest {

	@Test
	public void testBuildRevocRegDefRequestWorks() throws Exception {
		String expectedResult =
				"\"operation\":{" +
						"\"type\":\"113\"," +
						"\"id\":\"RevocRegID\"," +
						"\"revocDefType\":\"CL_ACCUM\"," +
						"\"tag\":\"TAG1\"," +
						"\"credDefId\":\"CredDefID\"," +
						"\"value\":{" +
						"   \"issuanceType\":\"ISSUANCE_ON_DEMAND\"," +
						"   \"maxCredNum\":5," +
						"   \"publicKeys\":{" +
						"       \"accumKey\":{" +
						"           \"z\":\"1111 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\"" +
						"       }" +
						"   }";

		String data = "{\n" +
				"        \"ver\": \"1.0\",\n" +
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
				"                    \"z\": \"1111 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\"\n" +
				"                }\n" +
				"            }\n" +
				"        }\n" +
				"    }";

		String request = Ledger.buildRevocRegDefRequest(DID, data).get();
		assertTrue(request.replaceAll("\\s+","").contains(expectedResult.replaceAll("\\s+","")));
	}

	@Test
	public void testRevocRegDefRequestsWorks() throws Exception {
		String myDid = createStoreAndPublishDidFromTrustee();

		String getRevRegDefRequest = Ledger.buildGetRevocRegDefRequest(myDid, revRegDefId).get();
		String getRevRegDefResponse = PoolUtils.ensurePreviousRequestApplied(pool, getRevRegDefRequest, response -> {
			JSONObject responseObject = new JSONObject(response);
			return !responseObject.getJSONObject("result").isNull("seqNo");
		});

		Ledger.parseGetRevocRegDefResponse(getRevRegDefResponse).get();
	}
}
