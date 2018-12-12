package org.hyperledger.indy.sdk.did;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.wallet.WalletItemNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class KeyForDidTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testKeyForDidWorksForMyDid() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, "{}").get();
		String did = result.getDid();
		String key = result.getVerkey();

		String receivedKey = Did.keyForDid(pool, wallet, did).get();
		assertEquals(key, receivedKey);
	}

	@Test
	public void testKeyForDidWorksForTheirDid() throws Exception {
		Did.storeTheirDid(wallet, String.format(IDENTITY_JSON_TEMPLATE, DID_MY1, VERKEY_MY1)).get();

		String receivedKey = Did.keyForDid(pool, wallet, DID_MY1).get();
		assertEquals(VERKEY_MY1, receivedKey);
	}

	@Test
	public void testKeyForDidWorksForGetKeyFromLedger() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(this.wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = result.getDid();

		Did.storeTheirDid(wallet, String.format(IDENTITY_JSON_TEMPLATE,  DID_MY1, VERKEY_MY1)).get();
		String nymRequest = Ledger.buildNymRequest(trusteeDid,  DID_MY1, VERKEY_MY1, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String receivedKey = Did.keyForDid(pool, wallet, DID_MY1).get();
		assertEquals(VERKEY_MY1, receivedKey);
	}

	@Test
	public void testKeyForDidWorksForNoKey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Did.keyForDid(pool, wallet, DID_MY2).get();
	}
}