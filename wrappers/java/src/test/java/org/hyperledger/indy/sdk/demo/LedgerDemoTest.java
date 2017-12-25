package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.did.DidJSONParameters;
import org.hyperledger.indy.sdk.did.DidResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import java.util.concurrent.TimeUnit;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;

public class LedgerDemoTest extends IndyIntegrationTest {

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

	@Test
	public void testLedgerDemo() throws Exception {

		// 1. Create ledger config from genesis txn file
		String poolName = PoolUtils.createPoolLedgerConfig();
		
		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		Pool pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		// 2. Create and Open My Wallet
		Wallet.createWallet(poolName, "myWallet", TYPE, null, null).get();
		Wallet myWallet = Wallet.openWallet("myWallet", null, null).get();

		// 3. Create and Open Trustee Wallet
		Wallet.createWallet(poolName, "theirWallet", TYPE, null, null).get();
		Wallet trusteeWallet = Wallet.openWallet("theirWallet", null, null).get();

		// 4. Create My Did
		CreateAndStoreMyDidResult createMyDidResult = Did.createAndStoreMyDid(myWallet, "{}").get();
		assertNotNull(createMyDidResult);
		String myDid = createMyDidResult.getDid();
		String myVerkey = createMyDidResult.getVerkey();

		// 5. Create Did from Trustee1 seed
		DidJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		CreateAndStoreMyDidResult createTheirDidResult = Did.createAndStoreMyDid(trusteeWallet, theirDidJson.toJson()).get();
		assertNotNull(createTheirDidResult);
		String trusteeDid = createTheirDidResult.getDid();

		// 6. Build Nym Request
		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		assertNotNull(nymRequest);

		// 7. Trustee Sign Nym Request
		String nymResponseJson = Ledger.signAndSubmitRequest(pool, trusteeWallet, trusteeDid, nymRequest).get();
		assertNotNull(nymResponseJson);

		JSONObject nymResponse = new JSONObject(nymResponseJson);

		assertEquals(myDid, nymResponse.getJSONObject("result").getString("dest"));
		assertEquals(myVerkey, nymResponse.getJSONObject("result").getString("verkey"));

		// 8. Close and delete My Wallet
		myWallet.closeWallet().get();
		Wallet.deleteWallet("myWallet", null).get();

		// 9. Close and delete Their Wallet
		trusteeWallet.closeWallet().get();
		Wallet.deleteWallet("theirWallet", null).get();

		// 10. Close Pool
		pool.closePoolLedger().get();
	}
}
