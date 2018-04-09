package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.blob_storage.BlobStorageWriter;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Test;

import java.util.Date;

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

	@Test
	public void testRevocRegRequestsWorks() throws Exception {
		String myDid = createStoreAndPublishDidFromTrustee();

		// Issuer create credential schema
		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(myDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String schema = createSchemaResult.getSchemaJson();
		String schemaId = createSchemaResult.getSchemaId();

		String schemaRequest = Ledger.buildSchemaRequest(myDid, schema).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();

		String getSchemaRequest = Ledger.buildGetSchemaRequest(myDid, schemaId).get();
		String getSchemaResponse = PoolUtils.ensurePreviousRequestApplied(pool, getSchemaRequest, response -> {
			JSONObject getSchemaResponseObject = new JSONObject(response);
			return !getSchemaResponseObject.getJSONObject("result").isNull("seqNo");
		});

		LedgerResults.ParseResponseResult parseSchemaResult = Ledger.parseGetSchemaResponse(getSchemaResponse).get();

		// Issuer create credential definition
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredDefResult =
				Anoncreds.issuerCreateAndStoreCredentialDef(wallet, myDid, parseSchemaResult.getObjectJson(), TAG, null, REV_CRED_DEF_CONFIG).get();
		String credDefJson = createCredDefResult.getCredDefJson();
		String credDefId = createCredDefResult.getCredDefId();

		String credDefRequest = Ledger.buildCredDefRequest(myDid, credDefJson).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, credDefRequest).get();

		// Issuer create revocation registry
		BlobStorageWriter tailsWriter = BlobStorageWriter.openWriter("default", TAILS_WRITER_CONFIG).get();
		String revRegConfig = "{\"issuance_type\":null,\"max_cred_num\":5}";
		AnoncredsResults.IssuerCreateAndStoreRevocRegResult createRevRegResult = Anoncreds.issuerCreateAndStoreRevocReg(wallet, myDid, null, TAG, credDefId, revRegConfig, tailsWriter).get();
		String revRegId = createRevRegResult.getRevRegId();
		String revRegDef = createRevRegResult.getRevRegDefJson();
		String revRegEntry = createRevRegResult.getRevRegEntryJson();

		String revRegDefRequest = Ledger.buildRevocRegDefRequest(myDid, revRegDef).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, revRegDefRequest).get();

		String revRegEntryRequest = Ledger.buildRevocRegEntryRequest(myDid, revRegId, "CL_ACCUM", revRegEntry).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, revRegEntryRequest).get();

		int timestamp = (int) (new Date().getTime()/1000) + 100;

		String getRevRegRequest = Ledger.buildGetRevocRegRequest(myDid, revRegId, timestamp).get();
		String getRevReResponse = PoolUtils.ensurePreviousRequestApplied(pool, getRevRegRequest, response -> {
			JSONObject responseObject = new JSONObject(response);
			return !responseObject.getJSONObject("result").isNull("seqNo");
		});

		Ledger.parseGetRevocRegResponse(getRevReResponse).get();
	}
}
