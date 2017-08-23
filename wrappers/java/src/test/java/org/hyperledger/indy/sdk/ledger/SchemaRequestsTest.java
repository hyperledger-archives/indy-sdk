package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static junit.framework.TestCase.assertNull;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class SchemaRequestsTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String walletName = "ledgerWallet";
	private String identifier = "Th7MpTaRZVRYnPiabds81Y";

	@Before
	public void openPool() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();
		pool = Pool.openPoolLedger(poolName, null).get();

		Wallet.createWallet(poolName, walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void closePool() throws Exception {
		pool.closePoolLedger().get();
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testBuildSchemaRequestWorks() throws Exception {

		String data = "{\"name\":\"name\",\"version\":\"1.0\",\"attr_names\":[\"name\",\"male\"]}";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"101\"," +
				"\"data\":%s" +
				"}", identifier, data);

		String schemaRequest = Ledger.buildSchemaRequest(identifier, data).get();

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
				"}", identifier, identifier, data);

		String getSchemaRequest = Ledger.buildGetSchemaRequest(identifier, identifier, data).get();

		assertTrue(getSchemaRequest.contains(expectedResult));
	}

	@Test
	public void testSchemaRequestWorksWithoutSignature() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String did = didResult.getDid();

		String schemaData = "{\"name\":\"gvt2\",\n" +
				"             \"version\":\"2.0\",\n" +
				"             \"attr_names\": [\"name\", \"male\"]}";

		String schemaRequest = Ledger.buildSchemaRequest(did, schemaData).get();
		String schemaResponse = Ledger.submitRequest(pool, schemaRequest).get();

		assertNotNull(schemaResponse);
	}

	@Test
	public void testSchemaRequestsWorks() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
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
		assertEquals(did, getSchemaResponseObject.getJSONObject("result").getJSONObject("data").getString("origin"));
	}

	@Test
	public void testGetSchemaRequestsWorksForUnknownSchema() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String did = didResult.getDid();

		String getSchemaData = "{\"name\":\"schema_name\",\"version\":\"2.0\"}";
		String getSchemaRequest = Ledger.buildGetSchemaRequest(did, did, getSchemaData).get();
		String getSchemaResponse = Ledger.submitRequest(pool, getSchemaRequest).get();

		JSONObject getSchemaResponseObject = new JSONObject(getSchemaResponse);

		assertNull(getSchemaResponseObject.getJSONObject("result").optJSONObject("data"));
	}

	@Test
	public void testBuildSchemaRequestWorksForMissedFields() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String data = "{\"name\":\"name\",\"version\":\"1.0\"}";

		Ledger.buildSchemaRequest(identifier, data).get();
	}
}
