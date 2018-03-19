package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverStoreCredentialOfferTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverStoreCredentialOfferWorks() throws Exception {

		Anoncreds.proverStoreCredentialOffer(wallet, issuer1GvtCredOffer).get();
	}

	@Test
	public void testProverStoreCredentialOfferWorksForInvalidJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String credentialOffer = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		Anoncreds.proverStoreCredentialOffer(wallet, credentialOffer).get();
	}
}
