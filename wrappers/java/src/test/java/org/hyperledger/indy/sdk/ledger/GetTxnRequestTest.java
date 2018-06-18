package org.hyperledger.indy.sdk.ledger;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Test;

public class GetTxnRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildGetTxnRequestWorks() throws Exception {
		int seq_no = 1;
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"3\"," +
				"\"data\":%s," +
				"\"ledgerId\":1" +
				"}", DID, seq_no);

		String getTxnRequest = Ledger.buildGetTxnRequest(DID, null, seq_no).get();
		assertTrue(getTxnRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testBuildGetTxnRequestWorksForLedgerType() throws Exception {
		int seq_no = 1;
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"3\"," +
				"\"data\":%s," +
				"\"ledgerId\":0" +
				"}", DID, seq_no);

		String getTxnRequest = Ledger.buildGetTxnRequest(DID, "POOL", seq_no).get();
		assertTrue(getTxnRequest.replace("\\", "").contains(expectedResult));
	}

	@Test(timeout = PoolUtils.TEST_TIMEOUT_FOR_REQUEST_ENSURE)
	public void testGetTxnRequestWorks() throws Exception {
		String did = createStoreAndPublishDidFromTrustee();

		String schemaRequest = Ledger.buildSchemaRequest(did, SCHEMA_DATA).get();
		String schemaResponse = Ledger.signAndSubmitRequest(pool, wallet, did, schemaRequest).get();

		JSONObject schemaResponseObj = new JSONObject(schemaResponse);
		int seqNo = schemaResponseObj.getJSONObject("result").getJSONObject("txnMetadata").getInt("seqNo");

		String getTxnRequest = Ledger.buildGetTxnRequest(did, null, seqNo).get();
		String expectedData = "{\"name\":\"gvt\",\"version\":\"1.0\",\"attr_names\": [\"name\"]}";

		String getTxnResponse = PoolUtils.ensurePreviousRequestApplied(pool, getTxnRequest, response -> {
			JSONObject getTxnResponseObj = new JSONObject(response);
			JSONObject schemaTransactionObj =
					getTxnResponseObj.getJSONObject("result").getJSONObject("data").getJSONObject("txn").getJSONObject("data").getJSONObject("data");

			return new JSONObject(expectedData).similar(schemaTransactionObj);
		});
		assertNotNull(getTxnResponse);
	}

	@Test
	public void testGetTxnRequestWorksForInvalidSeqNo() throws Exception {
		String did = createStoreAndPublishDidFromTrustee();

		String schemaRequest = Ledger.buildSchemaRequest(did, SCHEMA_DATA).get();
		String schemaResponse = Ledger.signAndSubmitRequest(pool, wallet, did, schemaRequest).get();

		JSONObject schemaResponseObj = new JSONObject(schemaResponse);
		int seqNo = schemaResponseObj.getJSONObject("result").getJSONObject("txnMetadata").getInt("seqNo") + 1;

		String getTxnRequest = Ledger.buildGetTxnRequest(did, null, seqNo).get();
		String getTxnResponse = Ledger.submitRequest(pool, getTxnRequest).get();

		JSONObject getTxnResponseObj = new JSONObject(getTxnResponse);
		assertTrue(getTxnResponseObj.getJSONObject("result").isNull("data"));
	}
}
