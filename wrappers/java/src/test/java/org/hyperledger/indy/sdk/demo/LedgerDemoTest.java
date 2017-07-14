package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;

public class LedgerDemoTest extends IndyIntegrationTest {

	@Test
	public void testLedgerDemo() throws Exception {

		// 1. Create ledger config from genesis txn file
		String poolName = PoolUtils.createPoolLedgerConfig();
		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);

		// 2. Open pool ledger
		Pool pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		// 3. Create and Open My Wallet
		Wallet.createWallet(poolName, "myWallet", "default", null, null).get();
		Wallet myWallet = Wallet.openWallet("myWallet", null, null).get();

		// 4. Create and Open Trustee Wallet
		Wallet.createWallet(poolName, "theirWallet", "default", null, null).get();
		Wallet trusteeWallet = Wallet.openWallet("theirWallet", null, null).get();

		// 5. Create My Did
		CreateAndStoreMyDidResult createMyDidResult = Signus.createAndStoreMyDid(myWallet, "{}").get();
		assertNotNull(createMyDidResult);
		String myDid = createMyDidResult.getDid();
		String myVerkey = createMyDidResult.getVerkey();

		// 6. Create Did from Trustee1 seed
		SignusJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, null);

		CreateAndStoreMyDidResult createTheirDidResult = Signus.createAndStoreMyDid(trusteeWallet, theirDidJson.toJson()).get();
		assertNotNull(createTheirDidResult);
		String trusteeDid = createTheirDidResult.getDid();

		// 7. Build Nym Request
		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		assertNotNull(nymRequest);

		// 8. Trustee Sign Nym Request
		String nymResponseJson = Ledger.signAndSubmitRequest(pool, trusteeWallet, trusteeDid, nymRequest).get();
		assertNotNull(nymResponseJson);

		JSONObject nymResponse = new JSONObject(nymResponseJson);

		assertEquals(myDid, nymResponse.getJSONObject("result").getString("dest"));
		assertEquals(myVerkey, nymResponse.getJSONObject("result").getString("verkey"));

		// 9. Close and delete My Wallet
		myWallet.closeWallet().get();
		Wallet.deleteWallet("myWallet", null).get();

		// 10. Close and delete Their Wallet
		trusteeWallet.closeWallet().get();
		Wallet.deleteWallet("theirWallet", null).get();

		// 11. Close Pool
		pool.closePoolLedger().get();
	}
}
