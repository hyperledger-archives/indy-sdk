package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.*;
import org.junit.rules.Timeout;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class ClaimDefRequestsTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

	private String signatureType = "CL";

	@Test
	public void testBuildClaimDefRequestWorks() throws Exception {
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

		String claimDefRequest = Ledger.buildClaimDefTxn(DID, data).get();

		assertTrue(claimDefRequest.replaceAll("\\s+", "").contains(expectedResult.replaceAll("\\s+", "")));
	}

	@Test
	public void testBuildGetClaimDefRequestWorks() throws Exception {
		int seqNo = 1;
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"108\"," +
				"\"ref\":%d," +
				"\"signature_type\":\"%s\"," +
				"\"origin\":\"%s\"" +
				"}", DID, seqNo, signatureType, DID);

		String getClaimDefRequest = Ledger.buildGetClaimDefTxn(DID, seqNo, signatureType, DID).get();

		assertTrue(getClaimDefRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testBuildClaimDefRequestWorksForInvalidJson() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String data = "{\"primary\":{\"n\":\"1\",\"s\":\"2\",\"rms\":\"3\",\"r\":{\"name\":\"1\"}}}";

		Ledger.buildClaimDefTxn(DID, data).get();
	}

	@Test(timeout = 200_000)
	public void testClaimDefRequestsWorks() throws Exception {
		String myDid = createStoreAndPublishDidFromTrustee();

		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(myDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String schema = createSchemaResult.getSchemaJson();

		String schemaRequest = Ledger.buildSchemaRequest(myDid, schema).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();

		String getSchemaData = String.format("{\"name\":\"%s\",\"version\":\"%s\"}", GVT_SCHEMA_NAME, SCHEMA_VERSION);
		String getSchemaRequest = Ledger.buildGetSchemaRequest(myDid, myDid, getSchemaData).get();
		String getSchemaResponse = PoolUtils.ensurePreviousRequestApplied(pool, getSchemaRequest, response -> {
			JSONObject getSchemaResponseObject = new JSONObject(response);
			return !getSchemaResponseObject.getJSONObject("result").isNull("seqNo");
		});

		LedgerResults.ParseResponseResult parseSchemaResult = Ledger.parseGetSchemaResponse(getSchemaResponse).get();

		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredDefResult =
				Anoncreds.issuerCreateAndStoreCredentialDef(wallet, myDid, parseSchemaResult.getObjectJson(), TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String credDefJson = createCredDefResult.getCredDefJson();

		String claimDefRequest = Ledger.buildClaimDefTxn(myDid, credDefJson).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, claimDefRequest).get();

		String getClaimDefRequest = Ledger.buildGetClaimDefTxn(myDid, Integer.parseInt(parseSchemaResult.getId()), signatureType, myDid).get();
		String getClaimDefResponse = PoolUtils.ensurePreviousRequestApplied(pool, getClaimDefRequest, response -> {
			JSONObject responseObject = new JSONObject(response);
			return !responseObject.getJSONObject("result").isNull("seqNo");
		});

		Ledger.parseGetClaimDefResponse(getClaimDefResponse).get();
	}
}
