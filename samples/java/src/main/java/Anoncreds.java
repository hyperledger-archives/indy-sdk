import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import utils.PoolUtils;

import static org.hyperledger.indy.sdk.anoncreds.Anoncreds.*;
import static org.junit.Assert.*;


class Anoncreds {

	static void demo() throws Exception {
		System.out.println("Anoncreds sample -> started");

		String issuerWalletName = "issuerWallet";
		String proverWalletName = "trusteeWallet";
		String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
		String proverDid = "VsKV7grR1BUE29mG2Fm2kX";

		//1. Create and Open Pool
		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, "{}").get();

		//2. Issuer Create and Open Wallet
		Wallet.createWallet(poolName, issuerWalletName, "default", null, null).get();
		Wallet issuerWallet = Wallet.openWallet(issuerWalletName, null, null).get();

		//3. Prover Create and Open Wallet
		Wallet.createWallet(poolName, proverWalletName, "default", null, null).get();
		Wallet proverWallet = Wallet.openWallet(proverWalletName, null, null).get();

		//4. Issuer create ClaimDef
		String schemaJson = String.format("{\n" +
				"                    \"seqNo\":1,\n" +
				"                    \"dest\":\"%s\",\n" +
				"                    \"data\": {\n" +
				"                        \"name\":\"gvt\",\n" +
				"                        \"version\":\"1.0\",\n" +
				"                        \"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
				"                    }\n" +
				"                }", issuerDid);

		String schemaKey = String.format("{\n" +
				"                    \"name\":\"gvt\",\n" +
				"                    \"version\":\"1.0\",\n" +
				"                    \"did\":\"%s\"\n" +
				"                }", issuerDid);

		String claimDef = issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, schemaJson, null, false).get();

		//5. Prover create Master Secret
		String masterSecret = "masterSecretName";
		proverCreateMasterSecret(proverWallet, masterSecret).get();

		//6. Prover store Claim Offer
		//		Strng claimOffer = issuerCreateClaimOffer(wallet, schemaJson, issuerDid, proverDid).get();
		String claimOffer = String.format("{\"issuer_did\":\"%s\", \"schema_key\":%s}", issuerDid, schemaKey);
		proverStoreClaimOffer(proverWallet, claimOffer).get();

		//7. Prover get Claim Offers
		String claimOfferFilter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);
		String claimOffersJson = proverGetClaimOffers(proverWallet, claimOfferFilter).get();

		JSONArray claimOffersObject = new JSONArray(claimOffersJson);
		assertEquals(claimOffersObject.length(), 1);

		JSONObject claimOfferObject = claimOffersObject.getJSONObject(0);
		String claimOfferJson = claimOfferObject.toString();

		//8. Prover create ClaimReq
		String claimReq = proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).get();
		assertNotNull(claimReq);

		//9. Issuer create Claim
		String claimAttributesJson = "{\n" +
				"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = issuerCreateClaim(issuerWallet, claimReq, claimAttributesJson, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		//10. Prover store Claim
		proverStoreClaim(proverWallet, claimJson, null).get();

		//11. Prover gets Claims for Proof Request
		String proofRequestJson = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_key\":%s, \"issuer_did\":\"%s\"}]" +
				"                   }," +
				"                   \"attr2_referent\":{" +
				"                       \"name\":\"sex\"," +
				"                       \"restrictions\":[{\"schema_key\":%s, \"issuer_did\":\"%s\"}]" +
				"                   }," +
				"                   \"attr3_referent\":{" +
				"                       \"name\":\"phone\"" +
				"                   }" +
				"               }," +
				"               \"requested_predicates\":{\"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}}\n" +
				"          }", schemaKey, issuerDid, schemaKey, issuerDid);


		String claimsForProofJson = proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray claimsForAttribute2 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray claimsForPredicate = claimsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");

		assertEquals(claimsForAttribute1.length(), 1);
		assertEquals(claimsForAttribute2.length(), 1);
		assertEquals(claimsForPredicate.length(), 1);

		String claimUuid = claimsForAttribute1.getJSONObject(0).getString("referent");

		//12. Prover create Proof
		String selfAttestedValue = "8-800-200";
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"attr3_referent\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":[\"%s\", true],\n" +
				"                                                               \"attr2_referent\":[\"%s\", false]},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":\"%s\"}\n" +
				"                                        }", selfAttestedValue, claimUuid, claimUuid, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, schemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";


		String proofJson = proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();

		JSONObject proof = new JSONObject(proofJson);

		//13. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_referent").getString(1));

		assertNotNull(proof.getJSONObject("requested_proof").getJSONObject("unrevealed_attrs").getString("attr2_referent"));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("attr3_referent"));

		Boolean valid = verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
		assertTrue(valid);

		//14. Close and Delete issuer wallet
		issuerWallet.closeWallet().get();
		Wallet.deleteWallet(issuerWalletName, null).get();

		//15. Close and Delete prover wallet
		proverWallet.closeWallet().get();
		Wallet.deleteWallet(proverWalletName, null).get();

		//16. Close pool
		pool.closePoolLedger().get();

		//17. Delete Pool ledger config
		Pool.deletePoolLedgerConfig(poolName).get();

		System.out.println("Anoncreds sample -> completed");
	}
}
