package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.agent.Agent;
import org.hyperledger.indy.sdk.agent.AgentResults;
import org.hyperledger.indy.sdk.crypto.Crypto;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.Assert;
import org.junit.Test;

public class AgentDemoTest extends IndyIntegrationTest {

	@Test
	public void testAgentDemo() throws Exception {
		// 1. Create and open wallets for Alice and Bob
		Wallet.createWallet("no pool", "alice_wallet", null, null, null).get();
		Wallet aliceWallet = Wallet.openWallet("alice_wallet", null, null).get();
		Wallet.createWallet("no pool", "bob_wallet", null, null, null).get();
		Wallet bobWallet = Wallet.openWallet("bob_wallet", null, null).get();

		// 2. Create keys for Alice and Bob
		String aliceKey = Crypto.createKey(aliceWallet, "{}").get();
		String bobKey = Crypto.createKey(bobWallet, "{}").get();

		// 3. Prepare authenticated message from Alice to Bob
		byte[] encryptedAuthMsg = Agent.prepMsg(aliceWallet, aliceKey, bobKey, MESSAGE).get();

		// 4. Parse authenticated message on Bob's side
		{
			AgentResults.ParseMsgResult decryptedAuth = Agent.parseMsg(bobWallet, bobKey, encryptedAuthMsg).get();
			Assert.assertEquals(aliceKey, decryptedAuth.getSenderKey());
			Assert.assertArrayEquals(MESSAGE, decryptedAuth.getMessage());
		}

		// 5. Prepare anonymous message from Bob to Alice
		byte[] encryptedAnonMsg = Agent.prepAnonymousMsg(aliceKey, MESSAGE).get();

		// 6. Parse anonymous message on Alice's side
		{
			AgentResults.ParseMsgResult decryptedAnon = Agent.parseMsg(aliceWallet, aliceKey, encryptedAnonMsg).get();
			Assert.assertNull(decryptedAnon.getSenderKey());
			Assert.assertArrayEquals(MESSAGE, decryptedAnon.getMessage());
		}
	}
}
