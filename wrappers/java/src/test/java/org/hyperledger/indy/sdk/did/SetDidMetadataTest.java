package org.hyperledger.indy.sdk.did;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class SetDidMetadataTest extends IndyIntegrationTestWithSingleWallet {

	private String did;

	@Before
	public void createDid() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, "{}").get();
		did = result.getDid();
	}

	@Test
	public void testSetDidMetadataWorks() throws Exception {
		Did.setDidMetadata(wallet, did, METADATA).get();
	}

	@Test
	public void testSetDidMetadataWorksForReplace() throws Exception {
		Did.setDidMetadata(wallet, did, METADATA).get();
		String receivedMetadata = Did.getDidMetadata(wallet, did).get();
		assertEquals(METADATA, receivedMetadata);

		String newMetadata = "updated metadata";
		Did.setDidMetadata(wallet, did, newMetadata).get();
		String updatedMetadata = Did.getDidMetadata(wallet, did).get();
		assertEquals(newMetadata, updatedMetadata);
	}

	@Test
	public void testSetDidMetadataWorksForEmptyString() throws Exception {
		Did.setDidMetadata(wallet, did, "").get();
	}

	@Test
	public void testSetDidMetadataWorksForUnknownDid() throws Exception {
		Did.setDidMetadata(wallet, DID, METADATA).get();
	}

	@Test
	public void testSetDidMetadataWorksForInvalidDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Did.setDidMetadata(wallet, INVALID_DID, METADATA).get();
	}
}