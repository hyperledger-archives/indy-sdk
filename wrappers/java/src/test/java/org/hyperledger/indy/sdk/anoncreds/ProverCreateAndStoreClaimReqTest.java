package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverCreateAndStoreClaimReqTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverCreateAndStoreClaimReqWorks() throws Exception {
		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, issuer1gvtClaimDef, masterSecretName).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentIssuer() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer2GvtClaimOffer, issuer1gvtClaimDef, masterSecretName).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForInvalidClaimOffer() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimOffer = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOffer, issuer1gvtClaimDef, masterSecretName).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForInvalidMasterSecret() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, issuer1gvtClaimDef, masterSecretName + "a").get();
	}
}
