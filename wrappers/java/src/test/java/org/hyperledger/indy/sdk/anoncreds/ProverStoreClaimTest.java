package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.json.JSONObject;
import org.junit.*;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverStoreClaimTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverStoreClaimWorks() throws Exception {
		initCommonWallet();

		String claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, claimDef, masterSecretName).get();

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, gvtClaimValuesJson, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimJson, null).get();
	}

	@Test
	public void testProverStoreClaimWorksWithoutClaimReq() throws Exception {
		initCommonWallet();

		JSONObject claimObj = new JSONObject(claim);
		claimObj.put("issuer_did", proverDid);

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverStoreClaim(wallet, claimObj.toString(), null).get();
	}

	@Test
	public void testProverStoreClaimWorksForInvalidClaimJson() throws Exception {

		initCommonWallet();

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, claimDef, masterSecretName).get();

		String claimJson = "{\"claim\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
				"            \"issuer_did\":1,\"\n" +
				"            \"revoc_reg_seq_no\":null,\n" +
				"            \"schema_key\":1}";

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.proverStoreClaim(wallet, claimJson, null).get();
	}
}
