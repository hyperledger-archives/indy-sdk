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

		String claimRequest = Anoncreds.proverCreateAndStoreCredentialReq(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretName).get();

		AnoncredsResults.IssuerCreateCredentialResult createClaimResult = Anoncreds.issuerCreateCredentail(wallet, claimRequest, gvtCredentialValuesJson, null, - 1, - 1).get();
		String claimJson = createClaimResult.getCredentialJson();

		Anoncreds.proverStoreCredential(wallet, credentialId1, claimJson, null).get();
	}

	@Test
	public void testProverStoreClaimWorksWithoutClaimReq() throws Exception {

		JSONObject claimObj = new JSONObject(credential);
		claimObj.put("cred_def_id", "other_cred_def_id");

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverStoreCredential(wallet, credentialId1, claimObj.toString(), null).get();
	}

	@Test
	public void testProverStoreClaimWorksForInvalidClaimJson() throws Exception {

		Anoncreds.proverCreateAndStoreCredentialReq(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretName).get();

		String claimJson = "{\"credential\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
				"            \"issuer_did\":1}";

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.proverStoreCredential(wallet, credentialId1, claimJson, null).get();
	}
}
