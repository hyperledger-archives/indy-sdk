package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.json.JSONObject;
import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;


public class AgentPrepMsgTest extends IndyIntegrationTestWithSingleWallet {

	private void checkMessage(String senderVk, String recipientDid, byte[] encryptedMsg) throws Exception {
		byte[] decryptedMessage = Signus.decryptSealed(wallet, recipientDid, encryptedMsg).get();
		String decryptedMessageJson = new String(decryptedMessage);
		JSONObject decryptedMsg = new JSONObject(decryptedMessageJson);

		assertTrue(decryptedMsg.getBoolean("auth"));
		assertEquals(senderVk, decryptedMsg.getString("sender"));
		assertNotNull(decryptedMsg.getString("nonce"));
		assertNotNull(decryptedMsg.getString("msg"));
	}

	@Test
	public void testPrepMsgWorksForCreatedDidAsCid() throws Exception {
		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, true).toJson();
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson).get();
		String senderVk = result.getVerkey();

		didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, true).toJson();
		result = Signus.createAndStoreMyDid(this.wallet, didJson).get();
		String recipientDid = result.getDid();
		String recipientVk = result.getVerkey();

		byte[] encryptedMsg = Agent.prepMsg(wallet, senderVk, recipientVk, MESSAGE).get();

		checkMessage(senderVk, recipientDid, encryptedMsg);
	}
}