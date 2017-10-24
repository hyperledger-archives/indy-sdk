package org.hyperledger.indy.sdk.signus;

import org.bitcoinj.core.Base58;
import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.*;


public class CreateKeyTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testCreateKeyWorksForSeed() throws Exception {
		String paramJson = new SignusJSONParameters.CreateKeyJSONParameter(MY1_SEED, null).toJson();
		String senderVk = Signus.createKey(wallet, paramJson).get();
		assertEquals(32, Base58.decode(senderVk).length);
	}

	@Test
	public void testCreateKeyWorksWithoutSeed() throws Exception {
		String senderVk = Signus.createKey(wallet, "{}").get();
		assertEquals(32, Base58.decode(senderVk).length);
	}

	@Test
	public void testCreateKeyWorksForInvalidSeed() throws Exception {
		String paramJson = new SignusJSONParameters.CreateKeyJSONParameter("invalid_base58_string11111111111", null).toJson();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.createKey(wallet, paramJson).get();
	}
}