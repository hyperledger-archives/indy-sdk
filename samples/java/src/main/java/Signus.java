import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import utils.PoolUtils;

import static org.hyperledger.indy.sdk.signus.Signus.*;
import static org.junit.Assert.assertTrue;


class Signus {

	static void demo() throws Exception {
		System.out.println("Ledger sample -> started");

		String myWalletName = "myWallet";
		String theirWalletName = "theirWallet";

		//1. Create and Open Pool
		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, "{}").get();

		//2. Create and Open My Wallet
		Wallet.createWallet(poolName, myWalletName, "default", null, null).get();
		Wallet myWallet = Wallet.openWallet(myWalletName, null, null).get();

		//3. Create and Open Their Wallet
		Wallet.createWallet(poolName, theirWalletName, "default", null, null).get();
		Wallet theirWallet = Wallet.openWallet(theirWalletName, null, null).get();

		//4. Create My Did
		createAndStoreMyDid(myWallet, "{}").get();

		//5. Create Their Did
		CreateAndStoreMyDidResult createTheirDidResult = createAndStoreMyDid(theirWallet, "{}").get();
		String theirDid = createTheirDidResult.getDid();
		String theirVerkey = createTheirDidResult.getVerkey();

		// 6. Store Their DID
		String identityJson = String.format("{\"did\":\"%s\", \"verkey\":\"%s\"}", theirDid, theirVerkey);
		storeTheirDid(myWallet, identityJson).get();

		// 7. Their sign message
		String msg = "{\n" +
				"        \"reqId\":1495034346617224651,\n" +
				"        \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
				"        \"operation\":{\n" +
				"            \"type\":\"1\",\n" +
				"            \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"\n" +
				"        }\n" +
				"    }";

		byte[] signature = sign(theirWallet, theirDid, msg.getBytes()).get();

		// 8. I verify message
		Boolean valid = verifySignature(myWallet, pool, theirDid, msg.getBytes(), signature).get();
		assertTrue(valid);

		// 9. Close and delete My Wallet
		myWallet.closeWallet().get();
		Wallet.deleteWallet(myWalletName, null).get();

		// 10. Close and delete Their Wallet
		theirWallet.closeWallet().get();
		Wallet.deleteWallet(theirWalletName, null).get();

		// 11. Close Pool
		pool.closePoolLedger().get();

		// 12. Delete Pool ledger config
		Pool.deletePoolLedgerConfig(poolName).get();

		System.out.println("Ledger sample -> completed");
	}
}
