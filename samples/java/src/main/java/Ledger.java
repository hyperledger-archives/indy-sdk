import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidJSONParameters;
import org.hyperledger.indy.sdk.did.DidResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import utils.PoolUtils;

import static org.hyperledger.indy.sdk.ledger.Ledger.buildNymRequest;
import static org.hyperledger.indy.sdk.ledger.Ledger.signAndSubmitRequest;
import static org.junit.Assert.assertEquals;
import static utils.PoolUtils.PROTOCOL_VERSION;


class Ledger {

	static void demo() throws Exception {
		System.out.println("Ledger sample -> started");

		String trusteeSeed = "000000000000000000000000Trustee1";

		// Set protocol version 2 to work with Indy Node 1.4
		Pool.setProtocolVersion(PROTOCOL_VERSION).get();

		// 1. Create ledger config from genesis txn file
		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, "{}").get();

		// 2. Create and Open My Wallet
		String myWalletConfig = new JSONObject().put("id", "myWallet").toString();
		String myWalletCredentials = new JSONObject().put("key", "my_wallet_key").toString();
		Wallet.createWallet(myWalletConfig, myWalletCredentials).get();
		Wallet myWallet = Wallet.openWallet(myWalletConfig, myWalletCredentials).get();

		// 3. Create and Open Trustee Wallet
		String trusteeWalletConfig = new JSONObject().put("id", "theirWallet").toString();
		String trusteeWalletCredentials = new JSONObject().put("key", "trustee_wallet_key").toString();
		Wallet.createWallet(trusteeWalletConfig, trusteeWalletCredentials).get();
		Wallet trusteeWallet = Wallet.openWallet(trusteeWalletConfig, trusteeWalletCredentials).get();

		// 4. Create My Did
		CreateAndStoreMyDidResult createMyDidResult = Did.createAndStoreMyDid(myWallet, "{}").get();
		String myDid = createMyDidResult.getDid();
		String myVerkey = createMyDidResult.getVerkey();

		// 5. Create Did from Trustee1 seed
		DidJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, trusteeSeed, null, null);

		CreateAndStoreMyDidResult createTheirDidResult = Did.createAndStoreMyDid(trusteeWallet, theirDidJson.toJson()).get();
		String trusteeDid = createTheirDidResult.getDid();

		// 6. Build Nym Request
		String nymRequest = buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();

		// 7. Trustee Sign Nym Request
		String nymResponseJson = signAndSubmitRequest(pool, trusteeWallet, trusteeDid, nymRequest).get();

		JSONObject nymResponse = new JSONObject(nymResponseJson);

		assertEquals(myDid, nymResponse.getJSONObject("result").getJSONObject("txn").getJSONObject("data").getString("dest"));
		assertEquals(myVerkey, nymResponse.getJSONObject("result").getJSONObject("txn").getJSONObject("data").getString("verkey"));

		// 8. Close and delete My Wallet
		myWallet.closeWallet().get();
		Wallet.deleteWallet(myWalletConfig, myWalletCredentials).get();

		// 9. Close and delete Their Wallet
		trusteeWallet.closeWallet().get();
		Wallet.deleteWallet(trusteeWalletConfig, trusteeWalletCredentials).get();

		// 10. Close Pool
		pool.closePoolLedger().get();

		// 11. Delete Pool ledger config
		Pool.deletePoolLedgerConfig(poolName).get();

		System.out.println("Ledger sample -> completed");
	}
}
