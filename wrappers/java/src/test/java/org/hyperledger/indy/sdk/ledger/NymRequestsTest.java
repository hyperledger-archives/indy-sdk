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
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class NymRequestsTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String walletName = "ledgerWallet";
	private String identifier = "Th7MpTaRZVRYnPiabds81Y";
	private String dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

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
	public void testBuildNymRequestWorksForOnlyRequiredFields() throws Exception {

		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"dest\":\"%s\",\"type\":\"1\"}", identifier, dest);

		String nymRequest = Ledger.buildNymRequest(identifier, dest, null, null, null).get();
		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildNymRequestWorksForEmptyRole() throws Exception {

		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"dest\":\"%s\",\"role\":null,\"type\":\"1\"}", identifier, dest);

		String nymRequest = Ledger.buildNymRequest(identifier, dest, null, null, "").get();
		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildNymRequestWorksForOnlyOptionalFields() throws Exception {

		String verkey = "Anfh2rjAcxkE249DcdsaQl";
		String role = "STEWARD";
		String alias = "some_alias";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"alias\":\"%s\"," +
				"\"dest\":\"%s\"," +
				"\"role\":\"2\"," +
				"\"type\":\"1\"," +
				"\"verkey\":\"%s\"" +
				"}", identifier, alias, dest, verkey);

		String nymRequest = Ledger.buildNymRequest(identifier, dest, verkey, alias, role).get();

		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildGetNymRequestWorks() throws Exception {

		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"type\":\"105\",\"dest\":\"%s\"}", identifier, dest);

		String nymRequest = Ledger.buildGetNymRequest(identifier, dest).get();

		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testNymRequestWorksWithoutSignature() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, "{}").get();
		String did = result.getDid();

		String nymRequest = Ledger.buildNymRequest(did, did, null, null, null).get();
		Ledger.submitRequest(pool, nymRequest).get();
	}

	@Test
	public void testSendNymRequestsWorksForOnlyRequiredFields() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		String nymResponse = Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		assertNotNull(nymResponse);
	}

	@Test
	public void testSendNymRequestsWorksForOptionalFields() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();
		String role = "STEWARD";
		String alias = "some_alias";

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, alias, role).get();
		String nymResponse = Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		assertNotNull(nymResponse);
	}

	@Test
	public void testGetNymRequestWorks() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		String did = result.getDid();

		String getNymRequest = Ledger.buildGetNymRequest(did, did).get();
		String getNymResponseJson = Ledger.submitRequest(pool, getNymRequest).get();

		JSONObject getNymResponse = new JSONObject(getNymResponseJson);

		assertEquals(did, getNymResponse.getJSONObject("result").getString("dest"));
	}

	@Test
	public void testSendNymRequestsWorksForWrongSignerRole() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, myDidJson.toJson()).get();
		String myDid = myDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson2 =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult2 = Signus.createAndStoreMyDid(wallet, myDidJson2.toJson()).get();
		String myDid2 = myDidResult2.getDid();

		String nymRequest2 = Ledger.buildNymRequest(myDid, myDid2, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, nymRequest2).get();
	}

	@Test
	public void testSendNymRequestsWorksForUnknownSigner() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee9", null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, myDidJson.toJson()).get();
		String myDid = myDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
	}

	@Test
	public void testNymRequestsWorks() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String getNymRequest = Ledger.buildGetNymRequest(myDid, myDid).get();
		String getNymResponseJson = Ledger.submitRequest(pool, getNymRequest).get();

		JSONObject getNymResponse = new JSONObject(getNymResponseJson);

		assertEquals("REPLY", getNymResponse.getString("op"));
		assertEquals("105", getNymResponse.getJSONObject("result").getString("type"));
		assertEquals(myDid, getNymResponse.getJSONObject("result").getString("dest"));
	}

	@Test
	public void testSendNymRequestsWorksForWrongRole() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Ledger.buildNymRequest(identifier, dest, null, null, "WRONG_ROLE").get();
	}
}
