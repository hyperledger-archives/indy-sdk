package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.json.JSONObject;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class SchemaRequestsTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildSchemaRequestWorks() throws Exception {
		String data = "{\"name\":\"name\",\"version\":\"1.0\",\"attr_names\":[\"name\",\"male\"]}";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"101\"," +
				"\"data\":%s" +
				"}", DID, data);

		String schemaRequest = Ledger.buildSchemaRequest(DID, data).get();

		assertTrue(schemaRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testBuildGetSchemaRequestWorks() throws Exception {
		String data = "{\"name\":\"name\",\"version\":\"1.0\"}";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"107\"," +
				"\"dest\":\"%s\"," +
				"\"data\":%s" +
				"}", DID, DID, data);

		String getSchemaRequest = Ledger.buildGetSchemaRequest(DID, DID, data).get();

		assertTrue(getSchemaRequest.contains(expectedResult));
	}

	@Test
	public void testSchemaRequestWorksWithoutSignature() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidLedgerTransactionException.class));

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = didResult.getDid();

		String schemaRequest = Ledger.buildSchemaRequest(did, SCHEMA_DATA).get();
		String schemaResponse = Ledger.submitRequest(pool, schemaRequest).get();
		assertNotNull(schemaResponse);
	}

	@Test
	public void testSchemaRequestsWorks() throws Exception {
		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = didResult.getDid();

		String schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"attr_names\": [\"name\", \"male\"]}";

		String schemaRequest = Ledger.buildSchemaRequest(did, schemaData).get();
		Ledger.signAndSubmitRequest(pool, wallet, did, schemaRequest).get();

		String getSchemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
		String getSchemaRequest = Ledger.buildGetSchemaRequest(did, did, getSchemaData).get();
		String getSchemaResponse = Ledger.submitRequest(pool, getSchemaRequest).get();

		JSONObject getSchemaResponseObject = new JSONObject(getSchemaResponse);

		assertEquals("gvt2", getSchemaResponseObject.getJSONObject("result").getJSONObject("data").getString("name"));
		assertEquals("2.0", getSchemaResponseObject.getJSONObject("result").getJSONObject("data").getString("version"));
	}

	@Test
	public void testGetSchemaRequestsWorksForUnknownSchema() throws Exception {
		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = didResult.getDid();

		String getSchemaData = "{\"name\":\"schema_name\",\"version\":\"2.0\"}";
		String getSchemaRequest = Ledger.buildGetSchemaRequest(did, did, getSchemaData).get();
		String getSchemaResponse = Ledger.submitRequest(pool, getSchemaRequest).get();

		JSONObject getSchemaResponseObject = new JSONObject(getSchemaResponse);

		// TODO FIXME restore after INDY-699 will be fixed
		// assertNull(getSchemaResponseObject.getJSONObject("result").optJSONObject("data"));
		assertEquals(getSchemaResponseObject.getJSONObject("result").optJSONObject("data").toString(), getSchemaData);
	}

	@Test
	public void testBuildSchemaRequestWorksForMissedFields() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String data = "{\"name\":\"name\",\"version\":\"1.0\"}";

		Ledger.buildSchemaRequest(DID, data).get();
	}
}
