package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class GetKeyMetadataTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testGetKeyMetadataWorks() throws Exception {
		Signus.setKeyMetadata(wallet, VERKEY, METADATA).get();
		String receivedMetadata = Signus.getKeyMetadata(wallet, VERKEY).get();
		assertEquals(METADATA, receivedMetadata);
	}

	@Test
	public void testGetKeyMetadataWorksForEmptyString() throws Exception {
		Signus.setKeyMetadata(wallet, VERKEY, "").get();
		String receivedMetadata = Signus.getKeyMetadata(wallet, VERKEY).get();
		assertEquals("", receivedMetadata);
	}

	@Test
	public void testGetKeyMetadataWorksForNoMetadata() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.getKeyMetadata(wallet, VERKEY).get();
	}
}