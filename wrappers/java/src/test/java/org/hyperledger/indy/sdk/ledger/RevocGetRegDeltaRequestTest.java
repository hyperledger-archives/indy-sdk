package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Test;

import java.util.Date;

import static org.junit.Assert.assertTrue;

public class RevocGetRegDeltaRequestTest extends LedgerIntegrationTest {

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

	@Test
	public void testRevocRegRequestsDeltaWorks() throws Exception {
		postEntities();

		int to = (int) (new Date().getTime()/1000) + 100;

		String getRevRegRequest = Ledger.buildGetRevocRegDeltaRequest(DID, revRegDefId, -1, to).get();
		String getRevReResponse = PoolUtils.ensurePreviousRequestApplied(pool, getRevRegRequest, response -> {
			JSONObject responseObject = new JSONObject(response);
			return !responseObject.getJSONObject("result").isNull("seqNo");
		});

		Ledger.parseGetRevocRegDeltaResponse(getRevReResponse).get();
	}
}
