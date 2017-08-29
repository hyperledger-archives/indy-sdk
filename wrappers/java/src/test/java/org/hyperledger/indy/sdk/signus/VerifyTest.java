package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.io.UnsupportedEncodingException;

import static junit.framework.TestCase.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class VerifyTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String trusteeDid;
	private String trusteeVerkey;
	private String identityJson;
	private String myDid;
	private String myVerkey;
	private String walletName = "signusWallet";
	private byte[] msg = "{\"reqId\":1496822211362017764}".getBytes();
	private byte[] signature = {-87, -41, 8, -31, 7, 107, 110, 9, -63, -94, -54, -42, -94, 66, -18, -45, 63, -47, 12, -60, 8, -45, 55, 27, 120, 94, -52, -109, 53, 104,
			103, 61, 60, -7, -19, 127, 103, 46, -36, -33, 10, 95, 75, 53, -11, -46, -15, -105, -65, 41, 48, 30, 9, 16, 78, -4, -99, -50, -46, -111, 125, -123, 109, 11};


	@Before
	public void createWalletWithDid() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(poolName, walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, false);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		assertNotNull(result);

		trusteeDid = result.getDid();
		trusteeVerkey = result.getVerkey();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson2 =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "00000000000000000000000000000My1", null, null);

		CreateAndStoreMyDidResult result2 = Signus.createAndStoreMyDid(wallet, didJson2.toJson()).get();
		myDid = result2.getDid();
		myVerkey = result2.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
		pool.closePoolLedger().get();
	}

	@Test
	public void testVerifyWorksForVerkeyCachedInWallet() throws Exception {
		identityJson = String.format("{\"did\":\"%s\",\"verkey\":\"%s\"}", myDid, myVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		Boolean valid = Signus.verifySignature(wallet, pool, myDid, msg, signature).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetVerkeyFromLedger() throws Exception {
		Signus.storeTheirDid(wallet, String.format("{\"did\":\"%s\"}", myDid)).get();


		Boolean valid = Signus.verifySignature(wallet, pool, myDid, msg, signature).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetNymFromLedger() throws Exception {
		Boolean valid = Signus.verifySignature(wallet, pool, myDid, msg, signature).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForOtherSigner() throws Exception {
		identityJson = String.format("{\"did\":\"%s\", \"verkey\":\"%s\"}", trusteeDid, trusteeVerkey);

		Signus.storeTheirDid(wallet, identityJson).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Steward1", null, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		String stewardDid = result.getDid();
		String stewardVerkey = result.getVerkey();

		identityJson = String.format("{\"did\":\"%s\", \"verkey\":\"%s\"}", stewardDid, stewardVerkey);

		Signus.storeTheirDid(wallet, identityJson).get();

		String msg = "{\"reqId\":1496822211362017764}";

		byte[] signature = Signus.sign(wallet, trusteeDid, msg.getBytes()).get();

		Boolean valid = Signus.verifySignature(wallet, pool, stewardDid, msg.getBytes(), signature).get();

		assertFalse(valid);
	}
}
