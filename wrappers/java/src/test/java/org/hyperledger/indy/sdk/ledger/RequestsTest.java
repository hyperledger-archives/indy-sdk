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

import static org.junit.Assert.assertNotNull;

public class RequestsTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String walletName = "ledgerWallet";

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
	public void testSubmitRequestWorks() throws Exception {

		String request = "{\"reqId\":1491566332010860,\n" +
				"          \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\n" +
				"          \"operation\":{\n" +
				"             \"type\":\"105\",\n" +
				"             \"dest\":\"Th7MpTaRZVRYnPiabds81Y\"\n" +
				"          },\n" +
				"          \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"}";

		String response = Ledger.submitRequest(pool, request).get();

		JSONObject responseObject = new JSONObject(response);

		Assert.assertEquals("REPLY", responseObject.getString("op"));
		Assert.assertEquals("105", responseObject.getJSONObject("result").getString("type"));
		Assert.assertEquals(1491566332010860L, responseObject.getJSONObject("result").getLong("reqId"));
		Assert.assertEquals("{\"dest\":\"Th7MpTaRZVRYnPiabds81Y\",\"identifier\":\"V4SGRU86Z58d6TV7PBUe6f\",\"role\":\"2\",\"seqNo\":2,\"txnTime\":null,\"verkey\":\"~7TYfekw4GUagBnBVCqPjiC\"}", responseObject.getJSONObject("result").getString("data"));
		Assert.assertEquals("Th7MpTaRZVRYnPiabds81Y", responseObject.getJSONObject("result").getString("identifier"));
		Assert.assertEquals("Th7MpTaRZVRYnPiabds81Y", responseObject.getJSONObject("result").getString("dest"));
	}

	@Test
	public void testSignAndSubmitRequestWorks() throws Exception {

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
	public void testSignAndSubmitRequestWorksForNotFoundSigner() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "00000000000000000000UnknowSigner", null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String signerDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(signerDid, myDid, null, null, null).get();
		String nymResponse = Ledger.signAndSubmitRequest(pool, wallet, signerDid, nymRequest).get();
		assertNotNull(nymResponse);
	}

	@Test
	public void testSignAndSubmitRequestWorksForIncompatibleWalletAndPool() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletIncompatiblePoolError));

		String walletName = "incompatibleWallet";

		Wallet.createWallet("otherPoolName", walletName, "default", null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}
}
