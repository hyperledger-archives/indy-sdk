package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.junit.Test;

import java.util.Arrays;
import java.util.Base64;
import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.*;

public class AgentPrepAnonymousMsgTest extends IndyIntegrationTestWithSingleWallet {

	private void checkMessage(byte[] encryptedMsg) throws Exception {
		Wallet.createWallet(POOL, "walletForCheck", TYPE, null, null).get();
		Wallet localWallet = Wallet.openWallet("walletForCheck", null, null).get();

		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, false).toJson();
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(localWallet, didJson).get();
		String recipientDid = result.getDid();

		byte[] decryptedMessage = Signus.decryptSealed(localWallet, recipientDid, encryptedMsg).get();
		String decryptedMessageJson = new String(decryptedMessage);
		JSONObject decryptedMsg = new JSONObject(decryptedMessageJson);

		assertFalse(decryptedMsg.getBoolean("auth"));
		assertTrue(Arrays.equals(MESSAGE, Base64.getDecoder().decode(decryptedMsg.getString("msg"))));

		localWallet.closeWallet().get();
		Wallet.deleteWallet("walletForCheck", null).get();
	}

	@Test
	public void testPrepAnonymousMsgWorks() throws Exception {
		byte[] encryptedMsg = Agent.prepAnonymousMsg(VERKEY_MY1, MESSAGE).get();
		checkMessage(encryptedMsg);
	}

	@Test
	public void testPrepAnonymousMsgWorksForInvalidRecipientVk() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Agent.prepAnonymousMsg(INVALID_VERKEY, MESSAGE).get();
	}
}