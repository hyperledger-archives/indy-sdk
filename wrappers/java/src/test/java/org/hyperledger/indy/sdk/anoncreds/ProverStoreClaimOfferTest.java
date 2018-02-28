package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverStoreClaimOfferTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverStoreClaimOfferWorks() throws Exception {

		initCommonWallet();

		Anoncreds.proverStoreClaimOffer(wallet, issuer1GvtClaimOffer).get();
	}

	@Test
	public void testProverStoreClaimOfferWorksForInvalidJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimOffer = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		Anoncreds.proverStoreClaimOffer(wallet, claimOffer).get();
	}
}
