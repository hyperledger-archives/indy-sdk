package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.blob_storage.BlobStorageWriter;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Test;

import static org.junit.Assert.assertTrue;

public class RevocRegDefRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

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

		String revRegDefRequest = Ledger.buildRevocRegDefRequest(myDid, revRegDef).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, revRegDefRequest).get();

		String getRevRegDefRequest = Ledger.buildGetRevocRegDefRequest(myDid, revRegId).get();
		String getRevRegDefResponse = PoolUtils.ensurePreviousRequestApplied(pool, getRevRegDefRequest, response -> {
			JSONObject responseObject = new JSONObject(response);
			return !responseObject.getJSONObject("result").isNull("seqNo");
		});

		Ledger.parseGetRevocRegDefResponse(getRevRegDefResponse).get();
	}
}
