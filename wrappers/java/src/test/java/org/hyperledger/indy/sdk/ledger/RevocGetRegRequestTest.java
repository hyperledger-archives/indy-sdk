package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Test;

import java.util.Date;

import static org.junit.Assert.assertTrue;

public class RevocGetRegRequestTest extends LedgerIntegrationTest {

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

	@Test
	public void testRevocRegRequestsWorks() throws Exception {
		String myDid = createStoreAndPublishDidFromTrustee();

		int timestamp = (int) (new Date().getTime()/1000) + 100;

		String getRevRegRequest = Ledger.buildGetRevocRegRequest(myDid, revRegDefId, timestamp).get();
		String getRevReResponse = PoolUtils.ensurePreviousRequestApplied(pool, getRevRegRequest, response -> {
			JSONObject responseObject = new JSONObject(response);
			return !responseObject.getJSONObject("result").isNull("seqNo");
		});

		Ledger.parseGetRevocRegResponse(getRevReResponse).get();
	}
}
