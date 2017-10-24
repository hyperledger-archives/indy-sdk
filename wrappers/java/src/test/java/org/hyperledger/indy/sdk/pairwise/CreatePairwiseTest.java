package org.hyperledger.indy.sdk.pairwise;

import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class CreatePairwiseTest extends PairwiseIntegrationTest {

	@Test
	public void testCreatePairwiseWorks() throws Exception {
		Pairwise.createPairwise(wallet, theirDid, myDid, metadata).get();
	}

	@Test
	public void testCreatePairwiseWorksForEmptyMetadata() throws Exception {
		Pairwise.createPairwise(wallet, theirDid, myDid, null).get();
	}

	@Test
	public void testCreatePairwiseWorksForNotFoundMyDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Pairwise.createPairwise(wallet, theirDid, DID1, null).get();
	}

	@Test
	public void testCreatePairwiseWorksForNotFoundTheirDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Pairwise.createPairwise(wallet, DID1, myDid, null).get();
	}
}
