package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class GetDidMetadataTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testGetDidMetadataWorks() throws Exception {
		Signus.setDidMetadata(wallet, DID, METADATA).get();
		String receivedMetadata = Signus.getDidMetadata(wallet, DID).get();
		assertEquals(METADATA, receivedMetadata);
	}

	@Test
	public void testGetDidMetadataWorksForEmptyString() throws Exception {
		Signus.setDidMetadata(wallet, DID, "").get();
		String receivedMetadata = Signus.getDidMetadata(wallet, DID).get();
		assertEquals("", receivedMetadata);
	}

	@Test
	public void testGetDidMetadataWorksForNoMetadata() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.getDidMetadata(wallet, DID).get();
	}
}