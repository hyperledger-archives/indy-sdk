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
		Signus.setDidMetadata(wallet, DID, METADATA).get();
	}

	@Test
	public void testSetDidMetadataWorksForReplace() throws Exception {
		Signus.setDidMetadata(wallet, DID, METADATA).get();
		String receivedMetadata = Signus.getDidMetadata(wallet, DID).get();
		assertEquals(METADATA, receivedMetadata);

		String newMetadata = "updated metadata";
		Signus.setDidMetadata(wallet, DID, newMetadata).get();
		String updatedMetadata = Signus.getDidMetadata(wallet, DID).get();
		assertEquals(newMetadata, updatedMetadata);
	}

	@Test
	public void testSetDidMetadataWorksForEmptyString() throws Exception {
		Signus.setDidMetadata(wallet, DID, "").get();
	}

	@Test
	public void testSetDidMetadataWorksForInvalidDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.setDidMetadata(wallet, INVALID_DID, METADATA).get();
	}
}