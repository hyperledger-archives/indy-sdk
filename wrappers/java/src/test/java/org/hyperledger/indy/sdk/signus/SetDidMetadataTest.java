package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class SetDidMetadataTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testSetDidMetadataWorks() throws Exception {
		Signus.setDidMetadata(wallet, DID1, METADATA).get();
	}

	@Test
	public void testSetDidMetadataWorksForReplace() throws Exception {
		Signus.setDidMetadata(wallet, DID1, METADATA).get();
		String receivedMetadata = Signus.getDidMetadata(wallet, DID1).get();
		assertEquals(METADATA, receivedMetadata);

		String newMetadata = "updated metadata";
		Signus.setDidMetadata(wallet, DID1, newMetadata).get();
		String updatedMetadata = Signus.getDidMetadata(wallet, DID1).get();
		assertEquals(newMetadata, updatedMetadata);
	}

	@Test
	public void testSetDidMetadataWorksForEmptyString() throws Exception {
		Signus.setDidMetadata(wallet, DID1, "").get();
	}

	@Test
	public void testSetDidMetadataWorksForInvalidDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.setDidMetadata(wallet, "invalid_base58string", METADATA).get();
	}
}