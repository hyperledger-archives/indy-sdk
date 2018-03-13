package org.hyperledger.indy.sdk.anoncreds;

import static org.hyperledger.indy.sdk.utils.EnvironmentUtils.getIndyHomePath;
import static org.junit.Assert.*;


import blob_storage.BlobStorage;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.*;


public class IssuerRevokeClaimTest extends AnoncredsIntegrationTest {

	private String tailsWriterConfig = String.format("{\"base_dir\":\"%s\", \"uri_pattern\":\"\"}", getIndyHomePath("tails"));

	@Test
	public void testIssuerRevokeProofWorks() throws Exception {
		//1. Create wallet, get wallet handle
		String walletName = "revocationWallet";
		Wallet.createWallet("default", walletName, "default", null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		//2. Issuer create Schema
		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, gvtSchemaName, schemaVersion, gvtSchemaAttributes).get();
		String schemaJson = createSchemaResult.getSchemaJson();

		//3. Issuer create claim definition
		String revocationCredentialDefConfig = "{\"support_revocation\":true}";
		AnoncredsResults.IssuerCreateAndStoreClaimDefResult createClaimDefResult = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schemaJson, tag, null, revocationCredentialDefConfig).get();
		String claimDefId = createClaimDefResult.getClaimDefId();
		String claimDefJson = createClaimDefResult.getClaimDefJson();

		//4. Issuer create revocation registry
		String revRegConfig = "{\"issuance_type\":null,\"max_cred_num\":5}";
		AnoncredsResults.IssuerCreateAndStoreRevocRegResult createRevRegResult = Anoncreds.issuerCreateAndStoreRevocReg(wallet, issuerDid, null, tag, claimDefId, revRegConfig, "default", tailsWriterConfig).get();
		String revRegId = createRevRegResult.getRevRegId();
		String revRegDef = createRevRegResult.getRevRegDefJson();

		//5. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		//6. Issuer create Claim Offer
		String claimOfferJson = Anoncreds.issuerCreateClaimOffer(wallet, claimDefId, issuerDid, proverDid).get();

		//7. Prover store Claim Offer received from Issuer
		Anoncreds.proverStoreClaimOffer(wallet, claimOfferJson).get();

		//8. Prover create Claim Request
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOfferJson, claimDefJson, masterSecretName).get();

		//9. Issuer open TailsReader
		JSONObject revRegDeg = new JSONObject(revRegDef);
		BlobStorage tailsReader = BlobStorage.openReader("default",
				tailsWriterConfig,
				revRegDeg.getJSONObject("value").getString("tails_location"),
				revRegDeg.getJSONObject("value").getString("tails_hash")).get();

		//10. Issuer create Claim
		int userRevocIndex = 1;
		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimReq, gvtClaimValuesJson, revRegId, tailsReader.getTailsReaderHandle(), userRevocIndex).get();
		String claim = createClaimResult.getClaimJson();
		String revRegDelta = createClaimResult.getRevocRegDeltaJson();

		//11. Prover create RevocationInfo
		int timestamp = 100;
		String revInfo = Anoncreds.createRevocationInfo(tailsReader.getTailsReaderHandle(), revRegDef, revRegDelta, timestamp, userRevocIndex).get();

		//12. Prover store RevocationInfo
		Anoncreds.storeRevocationInfo(wallet, claimId1, revInfo).get();

		//13. Prover store received Claim
		Anoncreds.proverStoreClaim(wallet, claimId1, claim, revRegDef).get();

		//14. Prover gets Claims for Proof Request
		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();
		JSONObject claims = new JSONObject(claimsJson);
		JSONArray claimsForAttr1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");

		String claimUuid = claimsForAttr1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//15. Prover create Proof
		String requestedClaimsJson = String.format("{" +
				"\"self_attested_attributes\":{}," +
				"\"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true, \"timestamp\":%d }}," +
				"\"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\", \"timestamp\":%d}}" +
				"}", claimUuid, timestamp, claimUuid, timestamp);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, schemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDefJson);
		String revInfos = String.format("{\"%s\": { \"%s\":%s }}", claimUuid, timestamp, revInfo);

		String proofJson = Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson, masterSecretName,
				claimDefsJson, revInfos).get();
		JSONObject proof = new JSONObject(proofJson);

		//16. Issuer revoke Claim
		revRegDelta = Anoncreds.issuerRevokeClaim(wallet, tailsReader.getTailsReaderHandle(), revRegId, userRevocIndex).get();

		//17. Verifier verify proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		String id = revealedAttr1.getString("referent");

		schemasJson = String.format("{\"%s\":%s}", id, schemaJson);
		claimDefsJson = String.format("{\"%s\":%s}", id, claimDefJson);
		String revRegDefsJson = String.format("{\"%s\":%s}", id, revRegDef);
		String revRegs = String.format("{\"%s\": { \"%s\":%s }}", id, timestamp, revRegDelta);

		boolean valid = Anoncreds.verifierVerifyProof(proofRequest, proofJson, schemasJson, claimDefsJson, revRegDefsJson, revRegs).get();
		assertFalse(valid);

		// 17. Close and Delete Wallet
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}
}
