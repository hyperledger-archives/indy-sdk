package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import java.util.concurrent.TimeUnit;

public class SignusDemoTest extends IndyIntegrationTest {

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

	@Test
	public void testSignusDemo() throws Exception {

		//1. Create and Open Pool
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		Pool pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		//2. Create and Open My Wallet
		Wallet.createWallet(poolName, "myWallet", "default", null, null).get();
		Wallet myWallet = Wallet.openWallet("myWallet", null, null).get();

		//3. Create and Open Their Wallet
		Wallet.createWallet(poolName, "theirWallet", "default", null, null).get();
		Wallet theirWallet = Wallet.openWallet("theirWallet", null, null).get();

		//4. Create My Did
		CreateAndStoreMyDidResult createMyDidResult = Signus.createAndStoreMyDid(myWallet, "{}").get();
		assertNotNull(createMyDidResult);

		//5. Create Their Did
		CreateAndStoreMyDidResult createTheirDidResult = Signus.createAndStoreMyDid(theirWallet, "{}").get();
		assertNotNull(createTheirDidResult);
		String theirDid = createTheirDidResult.getDid();
		String theirVerkey = createTheirDidResult.getVerkey();

		// 6. Store Their DID
		String identityJson = String.format("{\"did\":\"%s\", \"verkey\":\"%s\"}", theirDid, theirVerkey);
		Signus.storeTheirDid(myWallet, identityJson).get();

		// 7. Their sign message
		String msg = "{\n" +
				"        \"reqId\":1495034346617224651,\n" +
				"        \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
				"        \"operation\":{\n" +
				"            \"type\":\"1\",\n" +
				"            \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"\n" +
				"        }\n" +
				"    }";

		byte[] signature = Signus.sign(theirWallet, theirDid, msg.getBytes()).get();

		// 8. I verify message
		Boolean valid = Signus.verifySignature(myWallet, pool, theirDid, msg.getBytes(), signature).get();
		assertTrue(valid);

		// 9. Close and delete My Wallet
		myWallet.closeWallet().get();
		Wallet.deleteWallet("myWallet", null).get();

		// 10. Close and delete Their Wallet
		theirWallet.closeWallet().get();
		Wallet.deleteWallet("theirWallet", null).get();

		//11. Close Pool
		pool.closePoolLedger().get();
	}
}
