package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.json.JSONObject;
import org.junit.Test;

import java.util.Arrays;
import java.util.Base64;

import static org.junit.Assert.*;

public class AgentPrepAnonymousMsgTest extends IndyIntegrationTestWithSingleWallet {

	private void checkMessage(String recipientDid, byte[] encryptedMsg) throws Exception {
		byte[] decryptedMessage = Signus.decryptSealed(wallet, recipientDid, encryptedMsg).get();
		String decryptedMessageJson = new String(decryptedMessage);
		JSONObject decryptedMsg = new JSONObject(decryptedMessageJson);

		assertFalse(decryptedMsg.getBoolean("auth"));
		assertTrue(Arrays.equals(MESSAGE, Base64.getDecoder().decode(decryptedMsg.getString("msg"))));
	}

	@Test
	public void testPrepAnonymousMsgWorks() throws Exception {

		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, true).toJson();
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson).get();
		String recipientDid = result.getDid();
		String recipientVk = result.getVerkey();

		byte[] encryptedMsg = Agent.prepAnonymousMsg(recipientVk, MESSAGE).get();

		checkMessage(recipientDid, encryptedMsg);
	}
}