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

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class AttribRequestsTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String walletName = "ledgerWallet";
	private String identifier = "Th7MpTaRZVRYnPiabds81Y";
	private String dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
	private String endpoint = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";

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
	public void testBuildAttribRequestWorksForRawData() throws Exception {

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"100\"," +
				"\"dest\":\"%s\"," +
				"\"raw\":\"%s\"" +
				"}", identifier, dest, endpoint);

		String attribRequest = Ledger.buildAttribRequest(identifier, dest, null, endpoint, null).get();

		assertTrue(attribRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testBuildAttribRequestWorksForMissedAttribute() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Ledger.buildAttribRequest(identifier, dest, null, null, null).get();
	}

	@Test
	public void testBuildGetAttribRequestWorks() throws Exception {

		String raw = "endpoint";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"104\"," +
				"\"dest\":\"%s\"," +
				"\"raw\":\"%s\"" +
				"}", identifier, dest, raw);

		String getAttribRequest = Ledger.buildGetAttribRequest(identifier, dest, raw).get();

		assertTrue(getAttribRequest.contains(expectedResult));
	}

	@Test
	public void testSendAttribRequestWorksWithoutSignature() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		String attribRequest = Ledger.buildAttribRequest(trusteeDid, trusteeDid, null, endpoint, null).get();
		Ledger.submitRequest(pool, attribRequest).get();
	}

	@Test
	public void testAttribRequestsWorks() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String attribRequest = Ledger.buildAttribRequest(myDid, myDid, null, endpoint, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, attribRequest).get();

		String getAttribRequest = Ledger.buildGetAttribRequest(myDid, myDid, "endpoint").get();
		String getAttribResponse = Ledger.submitRequest(pool, getAttribRequest).get();

		JSONObject getAttribResponseObject = new JSONObject(getAttribResponse);

		assertEquals(endpoint, getAttribResponseObject.getJSONObject("result").getString("data"));
	}

	@Test
	public void testBuildAttribRequestWorksForInvalidIdentifier() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Ledger.buildAttribRequest("invalid_base58_identifier", dest, null, endpoint, null).get();
	}
}
