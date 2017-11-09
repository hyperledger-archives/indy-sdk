package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class KeyForLocalDidTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testKeyForLocalDidWorksForMyDid() throws Exception {
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, "{}").get();
		String did = result.getDid();
		String key = result.getVerkey();

		String receivedKey = Signus.keyForLocalDid(wallet, did).get();
		assertEquals(key, receivedKey);
	}

	@Test
	public void testKeyForLocalDidWorksForTheirDid() throws Exception {
		Signus.storeTheirDid(wallet, String.format(IDENTITY_JSON_TEMPLATE, DID_MY1, VERKEY_MY1)).get();

		String receivedKey = Signus.keyForLocalDid(wallet, DID_MY1).get();
		assertEquals(VERKEY_MY1, receivedKey);
	}

	@Test
	public void testKeyForDidWorksForNoKey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.keyForLocalDid(wallet, DID_MY2).get();
	}
}