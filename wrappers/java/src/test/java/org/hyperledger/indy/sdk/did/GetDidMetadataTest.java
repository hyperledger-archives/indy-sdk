package org.hyperledger.indy.sdk.did;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class GetDidMetadataTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testGetDidMetadataWorks() throws Exception {
		Did.setDidMetadata(wallet, DID, METADATA).get();
		String receivedMetadata = Did.getDidMetadata(wallet, DID).get();
		assertEquals(METADATA, receivedMetadata);
	}

	@Test
	public void testGetDidMetadataWorksForEmptyString() throws Exception {
		Did.setDidMetadata(wallet, DID, "").get();
		String receivedMetadata = Did.getDidMetadata(wallet, DID).get();
		assertEquals("", receivedMetadata);
	}

	@Test
	public void testGetDidMetadataWorksForNoMetadata() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Did.getDidMetadata(wallet, DID).get();
	}
}