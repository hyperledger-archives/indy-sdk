package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import java.util.concurrent.TimeUnit;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertNotNull;

public class AnoncredsDemoTest extends IndyIntegrationTest {

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

	@Test
	public void testAnoncredsDemo() throws Exception {

		//1. Create and Open Pool
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		Pool pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		//2. Issuer Create and Open Wallet
		Wallet.createWallet(poolName, "issuerWallet", "default", null, null).get();
		Wallet issuerWallet = Wallet.openWallet("issuerWallet", null, null).get();

		//3. Prover Create and Open Wallet
		Wallet.createWallet(poolName, "proverWallet", "default", null, null).get();
		Wallet proverWallet = Wallet.openWallet("proverWallet", null, null).get();

		//4. Issuer create ClaimDef
		String schemaJson = "{\n" +
				"                    \"seqNo\":1,\n" +
				"                    \"data\": {\n" +
				"                        \"name\":\"gvt\",\n" +
				"                        \"version\":\"1.0\",\n" +
				"                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
				"                    }\n" +
				"                }";
		String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";

		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, schemaJson, null, false).get();
		assertNotNull(claimDef);

		//5. Prover create Master Secret
		String masterSecret = "masterSecretName";
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//6. Prover store Claim Offer
		String claimOffer = String.format("{\"issuer_did\":\"%s\", \"schema_seq_no\":%d}", issuerDid, 1);
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOffer).get();

		//7. Prover get Claim Offers
		String claimOfferFilter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);
		String claimOffersJson = Anoncreds.proverGetClaimOffers(proverWallet, claimOfferFilter).get();

		JSONArray claimOffersObject = new JSONArray(claimOffersJson);
		assertEquals(claimOffersObject.length(), 1);

		JSONObject claimOfferObject = claimOffersObject.getJSONObject(0);
		String claimOfferJson = claimOfferObject.toString();

		//8. Prover create ClaimReq
		String proverDid = "BzfFCYk";
		String claimReq = Anoncreds.proverCreateClaimReq(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).get();
		assertNotNull(claimReq);

		//9. Issuer create Claim
		String claimAttributesJson = "{\n" +
				"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, claimAttributesJson, -1, -1).get();
		assertNotNull(createClaimResult);
		String claimJson = createClaimResult.getClaimJson();

		//10. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, claimJson).get();

		//11. Prover gets Claims for Proof Request
		String proofRequestJson = "{\n" +
				"                          \"nonce\":\"123432421212\",\n" +
				"                          \"name\":\"proof_req_1\",\n" +
				"                          \"version\":\"0.1\",\n" +
				"                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"},\n" +
				"                                                \"attr2_uuid\":{\"schema_seq_no\":1,\"name\":\"sex\"}},\n" +
				"                          \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
				"                  }";

		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_uuid");
		JSONArray claimsForAttribute2 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_uuid");
		JSONArray claimsForPredicate = claimsForProof.getJSONObject("predicates").getJSONArray("predicate1_uuid");

		assertEquals(claimsForAttribute1.length(), 1);
		assertEquals(claimsForAttribute2.length(), 1);
		assertEquals(claimsForPredicate.length(), 1);

		String claimUuid = claimsForAttribute1.getJSONObject(0).getString("claim_uuid");

		//12. Prover create Proof
		String selfAttestedValue = "yes";
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"self1\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_uuid\":[\"%s\", true],\n" +
				"                                                               \"attr2_uuid\":[\"%s\", false]},\n" +
				"                                          \"requested_predicates\":{\"predicate1_uuid\":\"%s\"}\n" +
				"                                        }", selfAttestedValue, claimUuid, claimUuid, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, schemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";


		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//13. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_uuid").getString(1));

		assertNotNull(proof.getJSONObject("requested_proof").getJSONObject("unrevealed_attrs").getString("attr2_uuid"));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("self1"));

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
		assertTrue(valid);

		// 14. Close and delete My Wallet
		issuerWallet.closeWallet().get();
		Wallet.deleteWallet("issuerWallet", null).get();

		// 15. Close and delete Their Wallet
		proverWallet.closeWallet().get();
		Wallet.deleteWallet("proverWallet", null).get();

		//16. Close Pool
		pool.closePoolLedger().get();
	}
}
