package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.*;
import org.junit.rules.Timeout;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

public class CredDefRequestsTest extends LedgerIntegrationTest {

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

	@Test
	public void testBuildCredDefRequestWorks() throws Exception {
		String data = "{\n" +
				"        \"ver\": \"1.0\",\n" +
				"        \"id\": \"cred_def_id\",\n" +
				"        \"schemaId\": \"1\",\n" +
				"        \"type\": \"CL\",\n" +
				"        \"tag\": \"TAG_1\",\n" +
				"        \"value\": {\n" +
				"            \"primary\": {\n" +
				"                \"n\": \"1\",\n" +
				"                \"s\": \"2\",\n" +
				"                \"rms\": \"3\",\n" +
				"                \"r\": {\"name\": \"1\"},\n" +
				"                \"rctxt\": \"1\",\n" +
				"                \"z\": \"1\"\n" +
				"            }\n" +
				"        }\n" +
				"    }";

		String expectedResult = "\"operation\": {\n" +
				"            \"ref\": 1,\n" +
				"            \"data\": {\n" +
				"                \"primary\": {\"n\": \"1\", \"s\": \"2\", \"rms\": \"3\", \"r\": {\"name\": \"1\"}, \"rctxt\": \"1\", \"z\": \"1\"}\n" +
				"            },\n" +
				"            \"type\": \"102\",\n" +
				"            \"signature_type\": \"CL\"\n" +
				"        }";

		String credDefRequest = Ledger.buildCredDefRequest(DID, data).get();

		assertTrue(credDefRequest.replaceAll("\\s+", "").contains(expectedResult.replaceAll("\\s+", "")));
	}

	@Test
	public void testBuildGetCredDefRequestWorks() throws Exception {
		int seqNo = 1;
		String signatureType = "CL";
		String id = DID + ":3:" + signatureType + ":" + seqNo;
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"108\"," +
				"\"ref\":%d," +
				"\"signature_type\":\"%s\"," +
				"\"origin\":\"%s\"" +
				"}", DID, seqNo, signatureType, DID);

		String getCredDefRequest = Ledger.buildGetCredDefRequest(DID, id).get();

		assertTrue(getCredDefRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testBuildCredDefRequestWorksForInvalidJson() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String data = "{\"primary\":{\"n\":\"1\",\"s\":\"2\",\"rms\":\"3\",\"r\":{\"name\":\"1\"}}}";

		Ledger.buildCredDefRequest(DID, data).get();
	}

	@Test(timeout = PoolUtils.TEST_TIMEOUT_FOR_REQUEST_ENSURE)
	public void testCredDefRequestsWorks() throws Exception {
		String myDid = createStoreAndPublishDidFromTrustee();

		String getCredDefRequest = Ledger.buildGetCredDefRequest(myDid, credDefId).get();
		String getCredDefResponse = PoolUtils.ensurePreviousRequestApplied(pool, getCredDefRequest, response -> {
			JSONObject responseObject = new JSONObject(response);
			return !responseObject.getJSONObject("result").isNull("seqNo");
		});

		Ledger.parseGetCredDefResponse(getCredDefResponse).get();
	}
}
