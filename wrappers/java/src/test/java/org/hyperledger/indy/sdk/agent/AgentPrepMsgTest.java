package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.crypto.Crypto;
import org.hyperledger.indy.sdk.crypto.CryptoJSONParameters;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.json.JSONObject;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;


public class AgentPrepMsgTest extends IndyIntegrationTestWithSingleWallet {

	private void checkMessage(String senderVk, byte[] encryptedMsg) throws Exception {
		Wallet.createWallet(POOL, "walletForCheck", TYPE, null, null).get();
		Wallet localWallet = Wallet.openWallet("walletForCheck", null, null).get();

		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY2_SEED, null, true).toJson();
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(localWallet, didJson).get();
		String recipientDid = result.getDid();

		byte[] decryptedMessage = Signus.decryptSealed(localWallet, recipientDid, encryptedMsg).get();
		String decryptedMessageJson = new String(decryptedMessage);
		JSONObject decryptedMsg = new JSONObject(decryptedMessageJson);

		assertTrue(decryptedMsg.getBoolean("auth"));
		assertEquals(senderVk, decryptedMsg.getString("sender"));
		assertNotNull(decryptedMsg.getString("nonce"));
		assertNotNull(decryptedMsg.getString("msg"));

		localWallet.closeWallet().get();
		Wallet.deleteWallet("walletForCheck", null).get();
	}

	@Test
	public void testPrepMsgWorksForCreatedKey() throws Exception {
		String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter(MY1_SEED, null).toJson();
		String senderVk = Crypto.createKey(wallet, paramJson).get();

		byte[] encryptedMsg = Agent.prepMsg(wallet, senderVk, VERKEY_MY2, MESSAGE).get();

		checkMessage(senderVk, encryptedMsg);
	}

	@Test
	public void testPrepMsgWorksForCreatedDid() throws Exception {
		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, false).toJson();
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson).get();
		String senderVk = result.getVerkey();

		byte[] encryptedMsg = Agent.prepMsg(wallet, senderVk, VERKEY_MY2, MESSAGE).get();

		checkMessage(senderVk, encryptedMsg);
	}

	@Test
	public void testPrepMsgWorksForCreatedDidAsCid() throws Exception {
		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, true).toJson();
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson).get();
		String senderVk = result.getVerkey();

		byte[] encryptedMsg = Agent.prepMsg(wallet, senderVk, VERKEY_MY2, MESSAGE).get();

		checkMessage(senderVk, encryptedMsg);
	}

	@Test
	public void testPrepMsgWorksForUnknownSenderVerkey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Agent.prepMsg(wallet, VERKEY, VERKEY_MY2, MESSAGE).get();
	}

	@Test
	public void testPrepMsgWorksForInvalidRecipientVk() throws Exception {
		String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter(MY1_SEED, null).toJson();
		String senderVk = Crypto.createKey(wallet, paramJson).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Agent.prepMsg(wallet, senderVk, INVALID_VERKEY, MESSAGE).get();
	}
}