package org.hyperledger.indy.sample.Tests;

import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.hyperledger.indy.sample.utils.PoolUtils;
import org.hyperledger.indy.sample.utils.StorageUtils;

import static org.hyperledger.indy.sdk.ledger.Ledger.buildNymRequest;
import static org.hyperledger.indy.sdk.ledger.Ledger.signAndSubmitRequest;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;


public class Ledger {

	public static void run() throws Exception {
		StorageUtils.cleanupStorage();

		String trusteeSeed = "000000000000000000000000Trustee1";

		// 1. Create ledger config from genesis txn file
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		Pool pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		// 2. Create and Open My Wallet
		Wallet.createWallet(poolName, "myWallet", "default", null, null).get();
		Wallet myWallet = Wallet.openWallet("myWallet", null, null).get();

		// 3. Create and Open Trustee Wallet
		Wallet.createWallet(poolName, "theirWallet", "default", null, null).get();
		Wallet trusteeWallet = Wallet.openWallet("theirWallet", null, null).get();

		// 4. Create My Did
		CreateAndStoreMyDidResult createMyDidResult = Signus.createAndStoreMyDid(myWallet, "{}").get();
		assertNotNull(createMyDidResult);
		String myDid = createMyDidResult.getDid();
		String myVerkey = createMyDidResult.getVerkey();

		// 5. Create Did from Trustee1 seed
		SignusJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, trusteeSeed, null, null);

		CreateAndStoreMyDidResult createTheirDidResult = Signus.createAndStoreMyDid(trusteeWallet, theirDidJson.toJson()).get();
		assertNotNull(createTheirDidResult);
		String trusteeDid = createTheirDidResult.getDid();

		// 6. Build Nym Request
		String nymRequest = buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		assertNotNull(nymRequest);

		// 7. Trustee Sign Nym Request
		String nymResponseJson = signAndSubmitRequest(pool, trusteeWallet, trusteeDid, nymRequest).get();
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

		StorageUtils.cleanupStorage();
	}
}
