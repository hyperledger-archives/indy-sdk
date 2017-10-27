package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.junit.Test;

import static org.junit.Assert.assertEquals;


public class SetKeyMetadataTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testSetKeyMetadataWorks() throws Exception {
		Signus.setKeyMetadata(wallet, VERKEY, METADATA).get();
	}

	@Test
	public void testSetKeyMetadataWorksForReplace() throws Exception {
		Signus.setKeyMetadata(wallet, VERKEY, METADATA).get();
		String receivedMetadata = Signus.getKeyMetadata(wallet, VERKEY).get();
		assertEquals(METADATA, receivedMetadata);

		String newMetadata = "updated metadata";
		Signus.setKeyMetadata(wallet, VERKEY, newMetadata).get();
		String updatedMetadata = Signus.getKeyMetadata(wallet, VERKEY).get();
		assertEquals(newMetadata, updatedMetadata);
	}

	@Test
	public void testSetKeyMetadataWorksForEmptyString() throws Exception {
		Signus.setKeyMetadata(wallet, VERKEY, "").get();
	}
}