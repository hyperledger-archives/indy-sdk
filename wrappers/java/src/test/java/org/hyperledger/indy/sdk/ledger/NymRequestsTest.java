package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.junit.Assert;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

public class NymRequestsTest extends IndyIntegrationTest {

	@Rule
	public Timeout globalTimeout = new Timeout(5, TimeUnit.SECONDS);

	@Test
	public void testBuildNymRequestWorksForOnlyRequiredFields() throws Exception {

		String identifier = "Th7MpTaRZVRYnPiabds81Y";
		String dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"type\":\"1\",\"dest\":\"%s\"}", identifier, dest);

		String nymRequest = Ledger.buildNymRequest(identifier, dest, null, null, null).get();

		Assert.assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildNymRequestWorksForOnlyOptionalFields() throws Exception {

		String identifier = "Th7MpTaRZVRYnPiabds81Y";
		String dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
		String verkey = "Anfh2rjAcxkE249DcdsaQl";
		String role = "STEWARD";
		String alias = "some_alias";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"1\"," +
				"\"dest\":\"%s\"," +
				"\"verkey\":\"%s\"," +
				"\"alias\":\"%s\"," +
				"\"role\":\"2\"" +
				"}", identifier, dest, verkey, alias, role);

		String nymRequest = Ledger.buildNymRequest(identifier, dest, verkey, alias, role).get();

		Assert.assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildGetNymRequestWorks() throws Exception {

		String identifier = "Th7MpTaRZVRYnPiabds81Y";
		String dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"type\":\"105\",\"dest\":\"%s\"}", identifier, dest);

		String nymRequest = Ledger.buildGetNymRequest(identifier, dest).get();

		Assert.assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testNymRequestWorksWithoutSignature() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		Pool pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(poolName, "ledgerWallet", "default", null, null).get();
		Wallet wallet = Wallet.openWallet("ledgerWallet", null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "00000000000000000000000000000My1", null, null);

		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		String did = result.getDid();

		String nymRequest = Ledger.buildNymRequest(did, did, null, null, null).get();
		Ledger.submitRequest(pool, nymRequest).get();
	}

	@Test
	public void testGetNymRequestWorks() throws Exception {

		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		Pool pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(poolName, "ledgerWallet", "default", null, null).get();
		Wallet wallet = Wallet.openWallet("ledgerWallet", null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, null);

		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		String did = result.getDid();

		String getNymRequest = Ledger.buildGetNymRequest(did, did).get();
		String getNymResponseJson = Ledger.submitRequest(pool, getNymRequest).get();

		JSONObject getNymResponse = new JSONObject(getNymResponseJson);

		Assert.assertEquals("REPLY", getNymResponse.getString("op"));
		Assert.assertEquals("105", getNymResponse.getJSONObject("result").getString("type"));
		Assert.assertEquals(did, getNymResponse.getJSONObject("result").getString("dest"));
	}

	@Test
	public void testNymRequestsWorks() throws Exception {

		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		Pool pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(poolName, "ledgerWallet", "default", null, null).get();
		Wallet wallet = Wallet.openWallet("ledgerWallet", null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, myDidJson.toJson()).get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String getNymRequest = Ledger.buildGetNymRequest(myDid, myDid).get();
		String getNymResponseJson = Ledger.submitRequest(pool, getNymRequest).get();

		JSONObject getNymResponse = new JSONObject(getNymResponseJson);

		Assert.assertEquals("REPLY", getNymResponse.getString("op"));
		Assert.assertEquals("105", getNymResponse.getJSONObject("result").getString("type"));
		Assert.assertEquals(myDid, getNymResponse.getJSONObject("result").getString("dest"));
	}
}
