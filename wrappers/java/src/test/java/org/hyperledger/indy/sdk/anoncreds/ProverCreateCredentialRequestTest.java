package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverCreateCredentialRequestTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverCreateAndStoreCredentialReqWorks() throws Exception {
		Anoncreds.proverCreateCredentialReq(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretId).get();
	}

	@Test
	public void testProverCreateAndStoreCredentialReqWorksForCredentialDefDoesNotCorrespondToCredentialOfferDifferentIssuer() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.proverCreateCredentialReq(wallet, proverDid, issuer2GvtCredOffer, issuer1gvtCredDef, masterSecretId).get();
	}

	@Test
	public void testProverCreateAndStoreCredentialReqWorksForInvalidCredentialOffer() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String credentialOffer = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		Anoncreds.proverCreateCredentialReq(wallet, proverDid, credentialOffer, issuer1gvtCredDef, masterSecretId).get();
	}

	@Test
	public void testProverCreateAndStoreCredentialReqWorksForInvalidMasterSecret() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverCreateCredentialReq(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretId + "a").get();
	}
}
