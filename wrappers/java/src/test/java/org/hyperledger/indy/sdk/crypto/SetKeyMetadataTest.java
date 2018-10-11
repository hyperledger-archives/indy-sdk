package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.junit.Before;
import org.junit.Test;

import static org.junit.Assert.assertEquals;


public class SetKeyMetadataTest extends IndyIntegrationTestWithSingleWallet {

	private String key;

	@Before
	public void createKey() throws Exception {
		key = Crypto.createKey(wallet, "{}").get();
	}

	@Test
	public void testSetKeyMetadataWorks() throws Exception {
		Crypto.setKeyMetadata(wallet, key, METADATA).get();
	}

	@Test
	public void testSetKeyMetadataWorksForReplace() throws Exception {
		Crypto.setKeyMetadata(wallet, key, METADATA).get();
		String receivedMetadata = Crypto.getKeyMetadata(wallet, key).get();
		assertEquals(METADATA, receivedMetadata);

		String newMetadata = "updated metadata";
		Crypto.setKeyMetadata(wallet, key, newMetadata).get();
		String updatedMetadata = Crypto.getKeyMetadata(wallet, key).get();
		assertEquals(newMetadata, updatedMetadata);
	}

	@Test
	public void testSetKeyMetadataWorksForEmptyString() throws Exception {
		Crypto.setKeyMetadata(wallet, key, "").get();
	}

	@Test
	public void testSetKeyMetadataWorksForNoKey() throws Exception {
		Crypto.setKeyMetadata(wallet, VERKEY, METADATA).get();
	}
}