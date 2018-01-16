import org.hyperledger.indy.sdk.agent.AgentResults;
import org.hyperledger.indy.sdk.crypto.Crypto;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.Assert;

import static org.hyperledger.indy.sdk.agent.Agent.*;

class Agent {
	private static final byte[] MESSAGE = "{\"reqId\":1496822211362017764}".getBytes();
	private static final String ALICE_WALLET = "alice_wallet";
	private static final String BOB_WALLET = "bob_wallet";
	private static final String POOL_NAME = "no pool";

	static void demo() throws Exception {
		System.out.println("Agent sample -> started");

		// 1. Create and open wallets for Alice and Bob
		Wallet.createWallet(POOL_NAME, ALICE_WALLET, null, null, null).get();
		Wallet aliceWallet = Wallet.openWallet(ALICE_WALLET, null, null).get();
		Wallet.createWallet(POOL_NAME, BOB_WALLET, null, null, null).get();
		Wallet bobWallet = Wallet.openWallet(BOB_WALLET, null, null).get();

		// 2. Create keys for Alice and Bob
		String aliceKey = Crypto.createKey(aliceWallet, "{}").get();
		String bobKey = Crypto.createKey(bobWallet, "{}").get();

		// 3. Prepare authenticated message from Alice to Bob
		byte[] encryptedAuthMsg = prepMsg(aliceWallet, aliceKey, bobKey, MESSAGE).get();

		// 4. Parse authenticated message on Bob's side
		{
			AgentResults.ParseMsgResult decryptedAuth = parseMsg(bobWallet, bobKey, encryptedAuthMsg).get();
			Assert.assertEquals(aliceKey, decryptedAuth.getSenderKey());
			Assert.assertArrayEquals(MESSAGE, decryptedAuth.getMessage());
		}

		// 5. Prepare anonymous message from Bob to Alice
		byte[] encryptedAnonMsg = prepAnonymousMsg(aliceKey, MESSAGE).get();

		// 6. Parse anonymous message on Alice's side
		{
			AgentResults.ParseMsgResult decryptedAnon = parseMsg(aliceWallet, aliceKey, encryptedAnonMsg).get();
			Assert.assertNull(decryptedAnon.getSenderKey());
			Assert.assertArrayEquals(MESSAGE, decryptedAnon.getMessage());
		}

		aliceWallet.closeWallet();
		bobWallet.closeWallet();
		Wallet.deleteWallet(ALICE_WALLET, null);
		Wallet.deleteWallet(BOB_WALLET, null);

		System.out.println("Agent sample -> completed");
	}
}