package org.hyperledger.indy.sdk.pairwise;

import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;


public class SetPairwiseMetadataTest extends PairwiseIntegrationTest {

	@Test
	public void testSetPairwiseMetadataWorks() throws Exception {
		Pairwise.createPairwise(wallet, theirDid, myDid, null).get();
		String pairwiseWithoutMetadata = Pairwise.getPairwise(wallet, theirDid).get();

		Pairwise.setPairwiseMetadata(wallet, theirDid, metadata).get();
		String pairwiseWithMetadata = Pairwise.getPairwise(wallet, theirDid).get();

		assertNotEquals(pairwiseWithoutMetadata, pairwiseWithMetadata);
		assertEquals(String.format(PAIRWISE_TEMPLATE, myDid, metadata), pairwiseWithMetadata);

	}

	@Test
	public void testSetPairwiseMetadataWorksForNotCreatedPairwise() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Pairwise.setPairwiseMetadata(wallet, theirDid, metadata).get();
	}
}
