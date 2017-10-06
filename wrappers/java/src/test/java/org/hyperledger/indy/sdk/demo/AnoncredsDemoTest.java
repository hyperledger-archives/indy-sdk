package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.After;
import org.junit.Before;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertNotNull;

public class AnoncredsDemoTest extends IndyIntegrationTest {

	@Rule
	public Timeout globalTimeout = new Timeout(2, TimeUnit.MINUTES);

	private Pool pool;
	private Wallet issuerWallet;
	private Wallet proverWallet;
	private String poolName;

	@Before
	public void createWallet() throws Exception {
		//1. Create and Open Pool
		poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		//2. Issuer Create and Open Wallet
		Wallet.createWallet(poolName, "issuerWallet", TYPE, null, null).get();
		issuerWallet = Wallet.openWallet("issuerWallet", null, null).get();

		//3. Prover Create and Open Wallet
		Wallet.createWallet(poolName, "proverWallet", TYPE, null, null).get();
		proverWallet = Wallet.openWallet("proverWallet", null, null).get();
	}

	@After
	public void deleteWallet() throws Exception {
		issuerWallet.closeWallet().get();
		Wallet.deleteWallet("issuerWallet", null).get();

		proverWallet.closeWallet().get();
		Wallet.deleteWallet("proverWallet", null).get();

		pool.closePoolLedger().get();
	}

	@Test
	public void testAnoncredsDemo() throws Exception {

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
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).get();
		assertNotNull(claimReq);

		//9. Issuer create Claim
		String claimAttributesJson = "{\n" +
				"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, claimAttributesJson, - 1).get();
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
	}

	@Test
	public void testAnoncredsWorksForMultiplyIssuerSingleProver() throws Exception {

		String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
		String issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";

		Wallet issuerGvtWallet = issuerWallet;

		//1. Issuer2 Create and Open Wallet
		Wallet.createWallet(poolName, "issuer2Wallet", "default", null, null).get();
		Wallet issuerXyzWallet = Wallet.openWallet("issuer2Wallet", null, null).get();

		//2. Issuer create ClaimDef
		String gvtSchemaJson = "{\n" +
				"                    \"seqNo\":1,\n" +
				"                    \"data\": {\n" +
				"                        \"name\":\"gvt\",\n" +
				"                        \"version\":\"1.0\",\n" +
				"                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
				"                    }\n" +
				"                }";

		String gvtClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerGvtWallet, issuerDid, gvtSchemaJson, null, false).get();

		//3. Issuer create ClaimDef
		String xyzSchemaJson = "{\n" +
				"                    \"seqNo\":2,\n" +
				"                    \"data\": {\n" +
				"                        \"name\":\"xyz\",\n" +
				"                        \"version\":\"1.0\",\n" +
				"                        \"keys\":[\"status\",\"period\"]\n" +
				"                    }\n" +
				"                }";

		String xyzClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerXyzWallet, issuerDid2, xyzSchemaJson, null, false).get();

		//4. Prover create Master Secret
		String masterSecret = "masterSecretName";
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//5. Prover store Claim Offer received from Issuer1
		String claimOffer = String.format("{\"issuer_did\":\"%s\", \"schema_seq_no\":%d}", issuerDid, 1);
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOffer).get();

		//6. Prover store Claim Offer received from Issuer2
		String claimOffer2 = String.format("{\"issuer_did\":\"%s\", \"schema_seq_no\":%d}", issuerDid2, 2);
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOffer2).get();

		//7. Prover get Claim Offers
		String claimOffersJson = Anoncreds.proverGetClaimOffers(proverWallet, "{}").get();

		JSONArray claimOffersObject = new JSONArray(claimOffersJson);
		assertEquals(2, claimOffersObject.length());

		JSONObject claimOfferObj1 = claimOffersObject.getJSONObject(0);
		JSONObject claimOfferObj2 = claimOffersObject.getJSONObject(1);

		String gvtClaimOffer = claimOfferObj1.getString("issuer_did").equals(issuerDid) ? claimOfferObj1.toString() : claimOfferObj2.toString();
		String xyzClaimOffer = claimOfferObj1.getString("issuer_did").equals(issuerDid2) ? claimOfferObj1.toString() : claimOfferObj2.toString();


		//8. Prover create ClaimReq for GVT Claim Offer
		String proverDid = "BzfFCYk";
		String gvtClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret).get();

		//9. Issuer create Claim
		String gvtClaimAttributesJson = "{\n" +
				"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult gvtCreateClaimResult = Anoncreds.issuerCreateClaim(issuerGvtWallet, gvtClaimReq, gvtClaimAttributesJson, - 1).get();
		String gvtClaimJson = gvtCreateClaimResult.getClaimJson();

		//10. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, gvtClaimJson).get();

		//11. Prover create ClaimReq for GVT Claim Offer
		String xyzClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret).get();

		//12. Issuer create Claim
		String xyzClaimAttributesJson = "{\n" +
				"               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
				"               \"period\":[\"8\",\"8\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult xyzCreateClaimResult = Anoncreds.issuerCreateClaim(issuerXyzWallet, xyzClaimReq, xyzClaimAttributesJson, - 1).get();
		String xyzClaimJson = xyzCreateClaimResult.getClaimJson();

		//13. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, xyzClaimJson).get();

		//14. Prover gets Claims for Proof Request
		String proofRequestJson = "{\n" +
				"                          \"nonce\":\"123432421212\",\n" +
				"                          \"name\":\"proof_req_1\",\n" +
				"                          \"version\":\"0.1\",\n" +
				"                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"},\n" +
				"                                               \"attr2_uuid\":{\"schema_seq_no\":2,\"name\":\"status\"}},\n" +
				"                          \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}," +
				"                                                    \"predicate2_uuid\":{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}\n" +
				"                  }";


		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_uuid");
		JSONArray claimsForAttribute2 = claimsForProof.getJSONObject("attrs").getJSONArray("attr2_uuid");
		JSONArray claimsForPredicate1 = claimsForProof.getJSONObject("predicates").getJSONArray("predicate1_uuid");
		JSONArray claimsForPredicate2 = claimsForProof.getJSONObject("predicates").getJSONArray("predicate2_uuid");

		assertEquals(claimsForAttribute1.length(), 1);
		assertEquals(claimsForAttribute2.length(), 1);
		assertEquals(claimsForPredicate1.length(), 1);
		assertEquals(claimsForPredicate2.length(), 1);

		String claimUuidForAttr1 = claimsForAttribute1.getJSONObject(0).getString("claim_uuid");
		String claimUuidForAttr2 = claimsForAttribute2.getJSONObject(0).getString("claim_uuid");
		String claimUuidForPredicate1 = claimsForPredicate1.getJSONObject(0).getString("claim_uuid");
		String claimUuidForPredicate2 = claimsForPredicate2.getJSONObject(0).getString("claim_uuid");

		//15. Prover create Proof
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{},\n" +
				"                                          \"requested_attrs\":{\"attr1_uuid\":[\"%s\", true],\n" +
				"                                                               \"attr2_uuid\":[\"%s\", true]},\n" +
				"                                          \"requested_predicates\":{\"predicate1_uuid\":\"%s\"," +
				"                                                                    \"predicate2_uuid\":\"%s\"}\n" +
				"                                        }", claimUuidForAttr1, claimUuidForAttr2, claimUuidForPredicate1, claimUuidForPredicate2);

		String schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtSchemaJson, claimUuidForAttr2, xyzSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtClaimDef, claimUuidForAttr2, xyzClaimDef);

		String revocRegsJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//16. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_uuid").getString(1));

		assertEquals("partial",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr2_uuid").getString(1));

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
		assertTrue(valid);

		//18. Close and delete Issuer2 Wallet
		issuerXyzWallet.closeWallet().get();
		Wallet.deleteWallet("issuer2Wallet", null).get();
	}

	@Test
	public void testAnoncredsWorksForSingleIssuerSingleProverMultiplyClaims() throws Exception {

		String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";

		//1. Issuer create ClaimDef
		String gvtSchemaJson = "{\n" +
				"                    \"seqNo\":1,\n" +
				"                    \"data\": {\n" +
				"                        \"name\":\"gvt\",\n" +
				"                        \"version\":\"1.0\",\n" +
				"                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
				"                    }\n" +
				"                }";

		String gvtClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, gvtSchemaJson, null, false).get();

		//2. Issuer create ClaimDef
		String xyzSchemaJson = "{\n" +
				"                    \"seqNo\":2,\n" +
				"                    \"data\": {\n" +
				"                        \"name\":\"xyz\",\n" +
				"                        \"version\":\"1.0\",\n" +
				"                        \"keys\":[\"status\",\"period\"]\n" +
				"                    }\n" +
				"                }";

		String xyzClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, xyzSchemaJson, null, false).get();

		//3. Prover create Master Secret
		String masterSecret = "masterSecretName";
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//4. Prover store Claim Offer received from Issuer
		String claimOffer = String.format("{\"issuer_did\":\"%s\", \"schema_seq_no\":%d}", issuerDid, 1);
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOffer).get();

		//5. Prover store Claim Offer received from Issuer
		String claimOffer2 = String.format("{\"issuer_did\":\"%s\", \"schema_seq_no\":%d}", issuerDid, 2);
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOffer2).get();

		//6. Prover get Claim Offers
		String claimOffersJson = Anoncreds.proverGetClaimOffers(proverWallet, "{}").get();

		JSONArray claimOffersObject = new JSONArray(claimOffersJson);
		assertEquals(2, claimOffersObject.length());

		JSONObject claimOfferObj1 = claimOffersObject.getJSONObject(0);
		JSONObject claimOfferObj2 = claimOffersObject.getJSONObject(1);

		String gvtClaimOffer = claimOfferObj1.getInt("schema_seq_no") == 1 ? claimOfferObj1.toString() : claimOfferObj2.toString();
		String xyzClaimOffer = claimOfferObj1.getInt("schema_seq_no") == 2 ? claimOfferObj1.toString() : claimOfferObj2.toString();


		//7. Prover create ClaimReq for GVT Claim Offer
		String proverDid = "BzfFCYk";
		String gvtClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret).get();

		//8. Issuer create Claim
		String gvtClaimAttributesJson = "{\n" +
				"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult gvtCreateClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, gvtClaimReq, gvtClaimAttributesJson, - 1).get();
		String gvtClaimJson = gvtCreateClaimResult.getClaimJson();

		//9. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, gvtClaimJson).get();

		//10. Prover create ClaimReq for GVT Claim Offer
		String xyzClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret).get();

		//11. Issuer create Claim
		String xyzClaimAttributesJson = "{\n" +
				"               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
				"               \"period\":[\"8\",\"8\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult xyzCreateClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, xyzClaimReq, xyzClaimAttributesJson, - 1).get();
		String xyzClaimJson = xyzCreateClaimResult.getClaimJson();

		//12. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, xyzClaimJson).get();

		//13. Prover gets Claims for Proof Request
		String proofRequestJson = "{\n" +
				"                          \"nonce\":\"123432421212\",\n" +
				"                          \"name\":\"proof_req_1\",\n" +
				"                          \"version\":\"0.1\",\n" +
				"                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"}},\n" +
				"                          \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}," +
				"                                                    \"predicate2_uuid\":{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}\n" +
				"                  }";


		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_uuid");
		JSONArray claimsForPredicate1 = claimsForProof.getJSONObject("predicates").getJSONArray("predicate1_uuid");
		JSONArray claimsForPredicate2 = claimsForProof.getJSONObject("predicates").getJSONArray("predicate2_uuid");

		assertEquals(claimsForAttribute1.length(), 1);
		assertEquals(claimsForPredicate1.length(), 1);
		assertEquals(claimsForPredicate2.length(), 1);

		String claimUuidForAttr1 = claimsForAttribute1.getJSONObject(0).getString("claim_uuid");
		String claimUuidForPredicate1 = claimsForPredicate1.getJSONObject(0).getString("claim_uuid");
		String claimUuidForPredicate2 = claimsForPredicate2.getJSONObject(0).getString("claim_uuid");

		//14. Prover create Proof
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{},\n" +
				"                                          \"requested_attrs\":{\"attr1_uuid\":[\"%s\", true]},\n" +
				"                                          \"requested_predicates\":{\"predicate1_uuid\":\"%s\"," +
				"                                                                    \"predicate2_uuid\":\"%s\"}\n" +
				"                                        }", claimUuidForAttr1, claimUuidForPredicate1, claimUuidForPredicate2);

		String schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtSchemaJson, claimUuidForPredicate2, xyzSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtClaimDef, claimUuidForPredicate2, xyzClaimDef);

		String revocRegsJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//15. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_uuid").getString(1));

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyProofWorksForProofDoesNotCorrespondToProofRequest() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		//1. Issuer create ClaimDef
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

		//2. Prover create Master Secret
		String masterSecret = "masterSecretName";
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//3. Prover store Claim Offer
		String claimOffer = String.format("{\"issuer_did\":\"%s\", \"schema_seq_no\":%d}", issuerDid, 1);
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOffer).get();

		//4. Prover get Claim Offers
		String claimOfferFilter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);
		String claimOffersJson = Anoncreds.proverGetClaimOffers(proverWallet, claimOfferFilter).get();

		JSONArray claimOffersObject = new JSONArray(claimOffersJson);
		assertEquals(claimOffersObject.length(), 1);

		JSONObject claimOfferObject = claimOffersObject.getJSONObject(0);
		String claimOfferJson = claimOfferObject.toString();

		//5. Prover create ClaimReq
		String proverDid = "BzfFCYk";
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).get();
		assertNotNull(claimReq);

		//6. Issuer create Claim
		String claimAttributesJson = "{\n" +
				"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, claimAttributesJson, - 1).get();
		assertNotNull(createClaimResult);
		String claimJson = createClaimResult.getClaimJson();

		//7. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, claimJson).get();

		//8. Prover gets Claims for Proof Request
		String proofRequestJson = "{\n" +
				"                          \"nonce\":\"123432421212\",\n" +
				"                          \"name\":\"proof_req_1\",\n" +
				"                          \"version\":\"0.1\",\n" +
				"                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"}},\n" +
				"                          \"requested_predicates\":{}\n" +
				"                  }";

		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_uuid");

		assertEquals(claimsForAttribute1.length(), 1);

		String claimUuid = claimsForAttribute1.getJSONObject(0).getString("claim_uuid");

		//9. Prover create Proof
		String selfAttestedValue = "yes";
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"self1\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_uuid\":[\"%s\", true]},\n" +
				"                                          \"requested_predicates\":{}\n" +
				"                                        }", selfAttestedValue, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, schemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";


		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//10. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_uuid").getString(1));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("self1"));

		proofRequestJson = "{\n" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\",\n" +
				"                    \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"}},\n" +
				"                    \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
				"           }";

		Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
	}
}
