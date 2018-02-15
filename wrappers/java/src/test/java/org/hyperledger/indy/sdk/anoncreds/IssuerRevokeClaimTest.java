package org.hyperledger.indy.sdk.anoncreds;

import static org.junit.Assert.*;

import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateClaimResult;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.*;

import static org.hamcrest.CoreMatchers.isA;

public class IssuerRevokeClaimTest extends AnoncredsIntegrationTest {
	private Wallet issuerWallet;

	private String claimDefJson;
	private IssuerCreateClaimResult claimResult;

	@Before
	public void before() throws Exception {
		StorageUtils.cleanupStorage();

		//1. Create Issuer wallet, get wallet handle
		String walletName = "issuerWallet";
		Wallet.createWallet("default", walletName, "default", null, null).get();
		issuerWallet = Wallet.openWallet(walletName, null, null).get();

		//2. Issuer create claim definition
		claimDefJson = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, gvtSchemaJson, null, true).get();

		//3. Issuer create revocation registry
		Anoncreds.issuerCreateAndStoreRevocReg(issuerWallet, issuerDid, gvtSchemaJson, 5).get();

		//4. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(issuerWallet, masterSecretName).get();

		//5. Issuer create Claim Offer
		String claimOfferJson = Anoncreds.issuerCreateClaimOffer(issuerWallet, gvtSchemaJson, issuerDid, proverDid).get();

		//6. Prover store Claim Offer received from Issuer
		Anoncreds.proverStoreClaimOffer(issuerWallet, claimOfferJson).get();

		//7. Prover create Claim Request
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(issuerWallet, proverDid, claimOfferJson, claimDefJson, masterSecretName).get();

		//8. Issuer create Claim
		int userRevocIndex = 1;
		claimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, gvtClaimValuesJson, userRevocIndex).get();

		//9. Prover store received Claim
		Anoncreds.proverStoreClaim(issuerWallet, claimResult.getClaimJson(), null).get();
	}

	@After
	public void after() throws Exception {
		issuerWallet.closeWallet().get();
		StorageUtils.cleanupStorage();
	}

	@Test
	public void testAnoncredsWorksForClaimRevokedBeforeProofCreated() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(ClaimRevokedException.class));

		//9. Issuer revoke claim
		String revocRegUpdateJson = Anoncreds.issuerRevokeClaim(issuerWallet, issuerDid, gvtSchemaJson, 1).get();

		//10. Prover gets Claims for Proof Request
		String claimsJson = Anoncreds.proverGetClaimsForProofReq(issuerWallet, proofRequest).get();
		JSONObject claims = new JSONObject(claimsJson);
		JSONArray claimsForAttr1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		String claimUuid = claimsForAttr1.getJSONObject(0).getString("referent");

		//11. Prover create Proof
		String requestedClaimsJson = String.format(this.requestedClaimsJsonTemplate, claimUuid, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDefJson);
		String revocRegsJsons = String.format("{\"%s\":%s}", claimUuid, revocRegUpdateJson);

		Anoncreds.proverCreateProof(issuerWallet, proofRequest, requestedClaimsJson, schemasJson, masterSecretName, claimDefsJson, revocRegsJsons).get();
	}

	@Test
	public void testAnoncredsWorksForClaimRevokedAfterProofCreated() throws Exception {
		//9. Prover gets Claims for Proof Request
		String claimsJson = Anoncreds.proverGetClaimsForProofReq(issuerWallet, proofRequest).get();
		JSONObject claims = new JSONObject(claimsJson);
		JSONArray claimsForAttr1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		String claimUuid = claimsForAttr1.getJSONObject(0).getString("referent");

		//10. Prover create Proof
		String requestedClaimsJson = String.format(this.requestedClaimsJsonTemplate, claimUuid, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDefJson);
		String revocRegsJsons = String.format("{\"%s\":%s}", claimUuid, claimResult.getRevocRegUpdateJson());

		String proofJson = Anoncreds.proverCreateProof(issuerWallet, proofRequest, requestedClaimsJson, schemasJson, masterSecretName,
				claimDefsJson, revocRegsJsons).get();

		//11. Issuer revoke prover claim
		String revocRegUpdateJson = Anoncreds.issuerRevokeClaim(issuerWallet, issuerDid, gvtSchemaJson, 1).get();

		//12. Verifier verify proof
		String updatedRevocRegsJsons = String.format("{\"%s\":%s}", claimUuid, revocRegUpdateJson);

		boolean valid = Anoncreds.verifierVerifyProof(proofRequest, proofJson, schemasJson, claimDefsJson, updatedRevocRegsJsons).get();

		assertFalse(valid);
	}
}
