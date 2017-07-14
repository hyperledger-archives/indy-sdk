package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
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

import java.math.BigInteger;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

import static org.junit.Assert.assertNotNull;

public class RequestsTest extends IndyIntegrationTest {

	@Rule
	public Timeout globalTimeout = new Timeout(5, TimeUnit.SECONDS);

	@Test
	public void testSubmitRequestWorks() throws Exception {

		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, null).get();

		Wallet.createWallet("otherPoolName", "ledgerWallet", "default", null, null).get();

		String request = "{\n" +
				"                        \"reqId\":1491566332010860,\n" +
				"                         \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\n" +
				"                         \"operation\":{\n" +
				"                            \"type\":\"105\",\n" +
				"                            \"dest\":\"Th7MpTaRZVRYnPiabds81Y\"\n" +
				"                         },\n" +
				"                         \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"}";

		String response = Ledger.submitRequest(pool, request).get();

		JSONObject responseObject = new JSONObject(response);

		Assert.assertEquals("REPLY", responseObject.getString("op"));
		Assert.assertEquals("105", responseObject.getJSONObject("result").getString("type"));
		Assert.assertEquals(1491566332010860L, responseObject.getJSONObject("result").getLong("reqId"));
		Assert.assertEquals("{\"dest\":\"Th7MpTaRZVRYnPiabds81Y\",\"identifier\":\"V4SGRU86Z58d6TV7PBUe6f\",\"role\":\"2\",\"verkey\":\"~7TYfekw4GUagBnBVCqPjiC\"}", responseObject.getJSONObject("result").getString("data"));
		Assert.assertEquals("Th7MpTaRZVRYnPiabds81Y", responseObject.getJSONObject("result").getString("identifier"));
		Assert.assertEquals("Th7MpTaRZVRYnPiabds81Y", responseObject.getJSONObject("result").getString("dest"));

		pool.closePoolLedger().get();
	}

	@Test
	public void testSignAndSubmitRequestWorks() throws Exception {

		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, null).get();

		Wallet.createWallet(poolName, "ledgerWallet", "default", null, null).get();
		Wallet wallet = Wallet.openWallet("ledgerWallet", null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "00000000000000000000000000000My1", null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, myDidJson.toJson()).get();
		String myDid = myDidResult.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		pool.closePoolLedger().get();
		wallet.closeWallet().get();
		Wallet.deleteWallet("ledgerWallet", null).get();
	}

	@Test
	public void testSignAndSubmitRequestWorksForIncompatibleWalletAndPool() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletIncompatiblePoolError));

		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, null).get();

		Wallet.createWallet("otherPoolName", "ledgerWallet", "default", null, null).get();
		Wallet wallet = Wallet.openWallet("ledgerWallet", null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "00000000000000000000000000000My1", null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, myDidJson.toJson()).get();
		String myDid = myDidResult.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		pool.closePoolLedger().get();
		wallet.closeWallet().get();
		Wallet.deleteWallet("ledgerWallet", null).get();
	}
}
