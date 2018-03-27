package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverCreateAndStoreClaimReqTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverCreateAndStoreClaimReqWorks() throws Exception {

		initCommonWallet();
		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, claimDef, masterSecretName).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentIssuer() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer2GvtClaimOffer, claimDef, masterSecretName).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForInvalidClaimOffer() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimOffer = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOffer, claimDef, masterSecretName).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForInvalidMasterSecret() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, claimDef, masterSecretName + "a").get();
	}
}
