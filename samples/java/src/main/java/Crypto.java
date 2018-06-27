import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.did.DidResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.crypto.CryptoResults.AuthDecryptResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.Assert;
import utils.PoolUtils;

import java.util.Arrays;

import static org.hyperledger.indy.sdk.did.Did.*;
import static org.hyperledger.indy.sdk.crypto.Crypto.*;
import static utils.PoolUtils.PROTOCOL_VERSION;


class Crypto {

	static void demo() throws Exception {
		System.out.println("Crypto sample -> started");

		String myWalletName = "myWallet";
		String theirWalletName = "theirWallet";

		// Set protocol version 2 to work with Indy Node 1.4
		Pool.setProtocolVersion(PROTOCOL_VERSION).get();

		// 1. Create and Open Pool
		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, "{}").get();

		// 2. Create and Open My Wallet
		String myWalletCredentials = "{\"key\":\"my_wallet_key\"}";
		Wallet.createWallet(poolName, myWalletName, "default", null, myWalletCredentials).get();
		Wallet myWallet = Wallet.openWallet(myWalletName, null, myWalletCredentials).get();

		// 3. Create and Open Their Wallet
		String theirWalletCredentials = "{\"key\":\"their_wallet_key\"}";
		Wallet.createWallet(poolName, theirWalletName, "default", null, theirWalletCredentials).get();
		Wallet theirWallet = Wallet.openWallet(theirWalletName, null, theirWalletCredentials).get();

		// 4. Create My Did
		CreateAndStoreMyDidResult myDid = createAndStoreMyDid(myWallet, "{}").get();
		String myVerkey = myDid.getVerkey();

		// 5. Create Their Did
		CreateAndStoreMyDidResult createTheirDidResult = createAndStoreMyDid(theirWallet, "{}").get();
		String theirVerkey = createTheirDidResult.getVerkey();

		// 6. Their auth encrypt message
		String msg = "{\n" +
				"        \"reqId\":1495034346617224651,\n" +
				"        \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
				"        \"operation\":{\n" +
				"            \"type\":\"1\",\n" +
				"            \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"\n" +
				"        }\n" +
				"    }";

		byte[] encryptedMessage = authCrypt(theirWallet, theirVerkey, myVerkey, msg.getBytes()).get();

		// 7. I decrypt message
		AuthDecryptResult authDecryptResult = authDecrypt(myWallet, myVerkey, encryptedMessage).get();

		Assert.assertTrue(Arrays.equals(msg.getBytes(), authDecryptResult.getDecryptedMessage()));
		Assert.assertEquals(theirVerkey, authDecryptResult.getVerkey());

		// 8. Close and delete My Wallet
		myWallet.closeWallet().get();
		Wallet.deleteWallet(myWalletName, myWalletCredentials).get();

		// 9. Close and delete Their Wallet
		theirWallet.closeWallet().get();
		Wallet.deleteWallet(theirWalletName, theirWalletCredentials).get();

		// 10. Close Pool
		pool.closePoolLedger().get();

		// 11. Delete Pool ledger config
		Pool.deletePoolLedgerConfig(poolName).get();

		System.out.println("Crypto sample -> completed");
	}
}
