package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.json.JSONObject;
import org.junit.*;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverStoreClaimTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverStoreClaimWorks() throws Exception {

		String claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, issuer1gvtClaimDef, masterSecretName).get();

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, gvtClaimValuesJson, null, - 1, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimId1, claimJson, null).get();
	}

	@Test
	public void testProverStoreClaimWorksWithoutClaimReq() throws Exception {

		JSONObject claimObj = new JSONObject(claim);
		claimObj.put("cred_def_id", "other_cred_def_id");

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverStoreClaim(wallet, claimId1, claimObj.toString(), null).get();
	}

	@Test
	public void testProverStoreClaimWorksForInvalidClaimJson() throws Exception {

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, issuer1gvtClaimDef, masterSecretName).get();

		String claimJson = "{\"claim\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
				"            \"issuer_did\":1}";

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.proverStoreClaim(wallet, claimId1, claimJson, null).get();
	}
}
