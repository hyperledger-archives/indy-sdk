package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.junit.Test;

import static org.junit.Assert.assertEquals;


public class SetKeyMetadataTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testSetKeyMetadataWorks() throws Exception {
		Crypto.setKeyMetadata(wallet, VERKEY, METADATA).get();
	}

	@Test
	public void testSetKeyMetadataWorksForReplace() throws Exception {
		Crypto.setKeyMetadata(wallet, VERKEY, METADATA).get();
		String receivedMetadata = Crypto.getKeyMetadata(wallet, VERKEY).get();
		assertEquals(METADATA, receivedMetadata);

		String newMetadata = "updated metadata";
		Crypto.setKeyMetadata(wallet, VERKEY, newMetadata).get();
		String updatedMetadata = Crypto.getKeyMetadata(wallet, VERKEY).get();
		assertEquals(newMetadata, updatedMetadata);
	}

	@Test
	public void testSetKeyMetadataWorksForEmptyString() throws Exception {
		Crypto.setKeyMetadata(wallet, VERKEY, "").get();
	}
}