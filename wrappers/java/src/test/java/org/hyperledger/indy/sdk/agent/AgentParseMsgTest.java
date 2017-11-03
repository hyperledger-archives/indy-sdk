package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.crypto.Crypto;
import org.hyperledger.indy.sdk.crypto.CryptoJSONParameters;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.Arrays;
import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.*;


public class AgentParseMsgTest extends IndyIntegrationTestWithPoolAndSingleWallet {
	@Test
	public void testParseMsgWorksForAuthenticatedMessage() throws Exception {
		String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter(MY1_SEED, null).toJson();
		String senderVk = Crypto.createKey(wallet, paramJson).get();

		paramJson = new CryptoJSONParameters.CreateKeyJSONParameter(MY2_SEED, null).toJson();
		String recipientVk = Crypto.createKey(wallet, paramJson).get();

		byte[] encryptedMsg = Agent.prepMsg(wallet, senderVk, recipientVk, MESSAGE).get();
		AgentResults.ParseMsgResult parseResult = Agent.parseMsg(wallet, recipientVk, encryptedMsg).get();
		assertEquals(senderVk, parseResult.getSenderKey());
		assertTrue(Arrays.equals(MESSAGE, parseResult.getMessage()));
	}

	@Test
	public void testParseMsgWorksForAnonymousMessage() throws Exception {
		String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter(MY2_SEED, null).toJson();
		String recipientVk = Crypto.createKey(wallet, paramJson).get();

		byte[] encryptedMsg = Agent.prepAnonymousMsg(recipientVk, MESSAGE).get();
		AgentResults.ParseMsgResult parseResult = Agent.parseMsg(wallet, recipientVk, encryptedMsg).get();
		assertNull(parseResult.getSenderKey());
		assertTrue(Arrays.equals(MESSAGE, parseResult.getMessage()));
	}

	@Test
	public void testParseMsgWorksForInvalidAuthenticatedMessage() throws Exception {
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, "{}").get();
		String recipientDid = result.getDid();
		String recipientVk = result.getVerkey();

		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, recipientDid, recipientVk);
		Signus.storeTheirDid(wallet, identityJson).get();

		String msg = String.format("{\"auth\":true,\"nonсe\":\"Th7MpTaRZVRYnPiabds81Y12\",\"sender\":\"%s\",\"msg\":\"unencrypted message\"}", VERKEY);
		byte[] encryptedMsg = Signus.encryptSealed(wallet, pool, recipientDid, msg.getBytes()).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Agent.parseMsg(wallet, recipientVk, encryptedMsg).get();
	}

	@Test
	public void testParseMsgWorksForInvalidAnonymousMessage() throws Exception {
		String recipientVk = Crypto.createKey(wallet, "{}").get();

		String msg = "unencrypted message";

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Agent.parseMsg(wallet, recipientVk, msg.getBytes()).get();
	}

	@Test
	public void testParseMsgWorksForUnknownRecipientVk() throws Exception {
		byte[] encryptedMsg = Agent.prepAnonymousMsg(VERKEY, MESSAGE).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Agent.parseMsg(wallet, VERKEY, encryptedMsg).get();
	}
}