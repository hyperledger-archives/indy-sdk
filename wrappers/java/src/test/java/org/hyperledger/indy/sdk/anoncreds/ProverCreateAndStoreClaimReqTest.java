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

		String claimOffer = String.format(claimOfferTemplate, issuerDid, 1);

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOffer, claimDef, masterSecretName).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentIssuer() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimOffer = String.format(claimOfferTemplate, "acWziYqKpYi6ov5FcYDi1e3", 1);

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOffer, claimDef, masterSecretName).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentSchema() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimOffer = String.format(claimOfferTemplate, issuerDid, 2);

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOffer, claimDef, masterSecretName).get();
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

		String claimOffer = String.format(claimOfferTemplate, issuerDid, 1);

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOffer, claimDef, "other_master_secret").get();
	}
}
