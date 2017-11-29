package org.hyperledger.indy.sdk.ledger;

import static org.junit.Assert.assertTrue;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.json.JSONObject;
import org.junit.Test;

public class GetTxnRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildGetTxnRequestWorks() throws Exception {
		int data = 1;
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"3\"," +
				"\"data\":%s" +
				"}", DID, data);

		String getTxnRequest = Ledger.buildGetTxnRequest(DID, data).get();
		assertTrue(getTxnRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testGetTxnRequestWorks() throws Exception {
		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = didResult.getDid();

		String schemaRequest = Ledger.buildSchemaRequest(did, SCHEMA_DATA).get();
		String schemaResponse = Ledger.signAndSubmitRequest(pool, wallet, did, schemaRequest).get();

		JSONObject schemaResponseObj = new JSONObject(schemaResponse);
		int seqNo = schemaResponseObj.getJSONObject("result").getInt("seqNo");

		String getTxnRequest = Ledger.buildGetTxnRequest(did, seqNo).get();
		String getTxnResponse = Ledger.submitRequest(pool, getTxnRequest).get();

		JSONObject getTxnResponseObj = new JSONObject(getTxnResponse);
		JSONObject schemaTransactionObj = getTxnResponseObj.getJSONObject("result").getJSONObject("data");

		assertTrue(new JSONObject(SCHEMA_DATA).similar(schemaTransactionObj.getJSONObject("data")));
	}

	@Test
	public void testGetTxnRequestWorksForInvalidSeqNo() throws Exception {
		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = didResult.getDid();

		String schemaRequest = Ledger.buildSchemaRequest(did, SCHEMA_DATA).get();
		String schemaResponse = Ledger.signAndSubmitRequest(pool, wallet, did, schemaRequest).get();

		JSONObject schemaResponseObj = new JSONObject(schemaResponse);
		int seqNo = schemaResponseObj.getJSONObject("result").getInt("seqNo") + 1;

		String getTxnRequest = Ledger.buildGetTxnRequest(did, seqNo).get();
		String getTxnResponse = Ledger.submitRequest(pool, getTxnRequest).get();

		JSONObject getTxnResponseObj = new JSONObject(getTxnResponse);
		assertTrue(getTxnResponseObj.getJSONObject("result").isNull("data"));
	}
}
