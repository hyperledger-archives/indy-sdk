package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.InvalidStructureException;
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

import static org.hamcrest.CoreMatchers.isA;
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
	private String masterSecret = "masterSecretName";
	private String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
	private String issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	private String proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	private String gvtSchemaKey = String.format(SCHEMA_KEY_TEMPLATE, "gvt", issuerDid);
	private String xyzSchemaKey = String.format(SCHEMA_KEY_TEMPLATE, "xyz", issuerDid2);
	private String gvtSchemaJson = String.format(SCHEMA_TEMPLATE, 1, issuerDid, "gvt", "[\"age\",\"sex\",\"height\",\"name\"]");
	private String xyzSchemaJson = String.format(SCHEMA_TEMPLATE, 2, issuerDid2, "xyz", "[\"status\",\"period\"]");
	private String gvtClaimValuesJson = "{\n" +
			"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
			"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
			"               \"height\":[\"175\",\"175\"],\n" +
			"               \"age\":[\"28\",\"28\"]\n" +
			"        }";
	private String xyzClaimValuesJson = "{\n" +
			"               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
			"               \"period\":[\"8\",\"8\"]\n" +
			"        }";

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
		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, gvtSchemaJson, null, false).get();
		assertNotNull(claimDef);

		//5. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//6. Issuer create Claim Offer
		String claimOffer = Anoncreds.issuerCreateClaimOffer(issuerWallet, gvtSchemaJson, issuerDid, proverDid).get();

		//7. Prover store Claim Offer
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOffer).get();

		//8. Prover get Claim Offers
		String claimOfferFilter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);
		String claimOffersJson = Anoncreds.proverGetClaimOffers(proverWallet, claimOfferFilter).get();

		JSONArray claimOffersObject = new JSONArray(claimOffersJson);
		assertEquals(claimOffersObject.length(), 1);

		JSONObject claimOfferObject = claimOffersObject.getJSONObject(0);
		String claimOfferJson = claimOfferObject.toString();

		//9. Prover create ClaimReq
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).get();
		assertNotNull(claimReq);

		//10. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, gvtClaimValuesJson, - 1).get();
		assertNotNull(createClaimResult);
		String claimJson = createClaimResult.getClaimJson();

		//11. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, claimJson, null).get();

		//12. Prover gets Claims for Proof Request
		String proofRequestJson = "{" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\", " +
				"                    \"requested_attrs\": {" +
				"                          \"attr1_referent\":{\"name\":\"name\"}," +
				"                          \"attr2_referent\":{\"name\":\"sex\"}," +
				"                          \"attr3_referent\":{\"name\":\"phone\"}" +
				"                     }," +
				"                    \"requested_predicates\":{" +
				"                         \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}" +
				"                    }" +
				"                  }";

		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray claimsForAttribute2 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray claimsForPredicate = claimsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");

		assertEquals(claimsForAttribute1.length(), 1);
		assertEquals(claimsForAttribute2.length(), 1);
		assertEquals(claimsForPredicate.length(), 1);

		String claimUuid = claimsForAttribute1.getJSONObject(0).getString("referent");

		//13. Prover create Proof
		String selfAttestedValue = "8-800-300";
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"attr3_referent\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":[\"%s\", true],\n" +
				"                                                               \"attr2_referent\":[\"%s\", false]},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":\"%s\"}\n" +
				"                                        }", selfAttestedValue, claimUuid, claimUuid, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//14. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_referent").getString(1));

		assertNotNull(proof.getJSONObject("requested_proof").getJSONObject("unrevealed_attrs").getString("attr2_referent"));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("attr3_referent"));

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
		assertTrue(valid);
	}

	@Test
	public void testAnoncredsWorksForMultiplyIssuerSingleProver() throws Exception {

		Wallet issuerGvtWallet = issuerWallet;

		//1. Issuer2 Create and Open Wallet
		Wallet.createWallet(poolName, "issuer2Wallet", "default", null, null).get();
		Wallet issuerXyzWallet = Wallet.openWallet("issuer2Wallet", null, null).get();

		//2. Issuer create ClaimDef
		String gvtClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerGvtWallet, issuerDid, gvtSchemaJson, null, false).get();

		//3. Issuer create ClaimDef
		String xyzClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerXyzWallet, issuerDid2, xyzSchemaJson, null, false).get();

		//4. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//5. Issuer1 create Claim Offer
		String gvtClaimOffer = Anoncreds.issuerCreateClaimOffer(issuerGvtWallet, gvtSchemaJson, issuerDid, proverDid).get();

		//6. Prover store Claim Offer received from Issuer1
		Anoncreds.proverStoreClaimOffer(proverWallet, gvtClaimOffer).get();

		//7. Issuer2 create Claim Offer
		String xyzClaimOffer = Anoncreds.issuerCreateClaimOffer(issuerXyzWallet, xyzSchemaJson, issuerDid2, proverDid).get();

		//8. Prover store Claim Offer received from Issuer2
		Anoncreds.proverStoreClaimOffer(proverWallet, xyzClaimOffer).get();

		//9. Prover create ClaimReq for GVT Claim Offer
		String gvtClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret).get();

		//10. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult gvtCreateClaimResult = Anoncreds.issuerCreateClaim(issuerGvtWallet, gvtClaimReq, gvtClaimValuesJson, - 1).get();
		String gvtClaimJson = gvtCreateClaimResult.getClaimJson();

		//11. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, gvtClaimJson, null).get();

		//12. Prover create ClaimReq for GVT Claim Offer
		String xyzClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret).get();

		//13. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult xyzCreateClaimResult = Anoncreds.issuerCreateClaim(issuerXyzWallet, xyzClaimReq, xyzClaimValuesJson, - 1).get();
		String xyzClaimJson = xyzCreateClaimResult.getClaimJson();

		//14. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, xyzClaimJson, null).get();

		//15. Prover gets Claims for Proof Request
		String proofRequestJson = "{" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\", " +
				"                    \"requested_attrs\": {" +
				"                          \"attr1_referent\":{ \"name\":\"name\"}," +
				"                          \"attr2_referent\":{ \"name\":\"status\"}" +
				"                     }," +
				"                    \"requested_predicates\":{" +
				"                         \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}," +
				"                          \"predicate2_referent\":{\"attr_name\":\"period\",\"p_type\":\">=\",\"value\":5}" +
				"                    }" +
				"                  }";

		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray claimsForAttribute2 = claimsForProof.getJSONObject("attrs").getJSONArray("attr2_referent");
		JSONArray claimsForPredicate1 = claimsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");
		JSONArray claimsForPredicate2 = claimsForProof.getJSONObject("predicates").getJSONArray("predicate2_referent");

		assertEquals(claimsForAttribute1.length(), 1);
		assertEquals(claimsForAttribute2.length(), 1);
		assertEquals(claimsForPredicate1.length(), 1);
		assertEquals(claimsForPredicate2.length(), 1);

		String claimUuidForAttr1 = claimsForAttribute1.getJSONObject(0).getString("referent");
		String claimUuidForAttr2 = claimsForAttribute2.getJSONObject(0).getString("referent");
		String claimUuidForPredicate1 = claimsForPredicate1.getJSONObject(0).getString("referent");
		String claimUuidForPredicate2 = claimsForPredicate2.getJSONObject(0).getString("referent");

		//16. Prover create Proof
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":[\"%s\", true],\n" +
				"                                                               \"attr2_referent\":[\"%s\", true]},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":\"%s\"," +
				"                                                                    \"predicate2_referent\":\"%s\"}\n" +
				"                                        }", claimUuidForAttr1, claimUuidForAttr2, claimUuidForPredicate1, claimUuidForPredicate2);

		String schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtSchemaJson, claimUuidForAttr2, xyzSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtClaimDef, claimUuidForAttr2, xyzClaimDef);
		String revocRegsJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//17. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_referent").getString(1));

		assertEquals("partial",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr2_referent").getString(1));

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
		assertTrue(valid);

		//18. Close and delete Issuer2 Wallet
		issuerXyzWallet.closeWallet().get();
		Wallet.deleteWallet("issuer2Wallet", null).get();
	}

	@Test
	public void testAnoncredsWorksForSingleIssuerSingleProverMultiplyClaims() throws Exception {

		//1. Issuer create ClaimDef
		String gvtClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, gvtSchemaJson, null, false).get();

		//2. Issuer create ClaimDef
		String xyzSchemaJson = String.format(SCHEMA_TEMPLATE, 2, issuerDid, "xyz", "[\"status\",\"period\"]");
		String xyzClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, xyzSchemaJson, null, false).get();

		//3. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//4. Issuer create GVT Claim Offer
		String gvtClaimOffer = Anoncreds.issuerCreateClaimOffer(issuerWallet, gvtSchemaJson, issuerDid, proverDid).get();

		//5. Prover store Claim Offer received from Issuer
		Anoncreds.proverStoreClaimOffer(proverWallet, gvtClaimOffer).get();

		//6. Issuer create GVT Claim Offer
		String xyzClaimOffer = Anoncreds.issuerCreateClaimOffer(issuerWallet, xyzSchemaJson, issuerDid, proverDid).get();

		//7. Prover store Claim Offer received from Issuer
		Anoncreds.proverStoreClaimOffer(proverWallet, xyzClaimOffer).get();

		//8. Prover create ClaimReq for GVT Claim Offer
		String gvtClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret).get();

		//9. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult gvtCreateClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, gvtClaimReq, gvtClaimValuesJson, - 1).get();
		String gvtClaimJson = gvtCreateClaimResult.getClaimJson();

		//10. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, gvtClaimJson, null).get();

		//11. Prover create ClaimReq for GVT Claim Offer
		String xyzClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret).get();

		//12. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult xyzCreateClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, xyzClaimReq, xyzClaimValuesJson, - 1).get();
		String xyzClaimJson = xyzCreateClaimResult.getClaimJson();

		//13. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, xyzClaimJson, null).get();

		//14. Prover gets Claims for Proof Request
		String proofRequestJson = String.format("{" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\", " +
				"                    \"requested_attrs\": {" +
				"                          \"attr1_referent\":{ \"name\":\"name\", \"restrictions\":[{\"schema_key\":%s}]}" +
				"                     }," +
				"                    \"requested_predicates\":{" +
				"                         \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}," +
				"                          \"predicate2_referent\":{\"attr_name\":\"period\",\"p_type\":\">=\",\"value\":5}" +
				"                    }" +
				"                  }", gvtSchemaKey);

		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray claimsForPredicate1 = claimsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");
		JSONArray claimsForPredicate2 = claimsForProof.getJSONObject("predicates").getJSONArray("predicate2_referent");

		assertEquals(claimsForAttribute1.length(), 1);
		assertEquals(claimsForPredicate1.length(), 1);
		assertEquals(claimsForPredicate2.length(), 1);

		String claimUuidForAttr1 = claimsForAttribute1.getJSONObject(0).getString("referent");
		String claimUuidForPredicate1 = claimsForPredicate1.getJSONObject(0).getString("referent");
		String claimUuidForPredicate2 = claimsForPredicate2.getJSONObject(0).getString("referent");

		//15. Prover create Proof
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":[\"%s\", true]},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":\"%s\"," +
				"                                                                    \"predicate2_referent\":\"%s\"}\n" +
				"                                        }", claimUuidForAttr1, claimUuidForPredicate1, claimUuidForPredicate2);

		String schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtSchemaJson, claimUuidForPredicate2, xyzSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtClaimDef, claimUuidForPredicate2, xyzClaimDef);
		String revocRegsJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//16. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_referent").getString(1));

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyProofWorksForProofDoesNotCorrespondToProofRequest() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		//1. Issuer create ClaimDef
		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, gvtSchemaJson, null, false).get();
		assertNotNull(claimDef);

		//2. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//3. Issuer create Claim Offer
		String claimOfferJson = Anoncreds.issuerCreateClaimOffer(issuerWallet, gvtSchemaJson, issuerDid, proverDid).get();

		//4. Prover store Claim Offer
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOfferJson).get();

		//5. Prover create ClaimReq
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).get();
		assertNotNull(claimReq);

		//6. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, gvtClaimValuesJson, - 1).get();
		assertNotNull(createClaimResult);
		String claimJson = createClaimResult.getClaimJson();

		//7. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, claimJson, null).get();

		//8. Prover gets Claims for Proof Request
		String proofRequestJson = String.format("{" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\", " +
				"                    \"requested_attrs\": {" +
				"                          \"attr1_referent\":{ \"name\":\"name\", \"restrictions\":[{\"schema_key\":%s}]}," +
				"                          \"attr2_referent\":{ \"name\":\"phone\"}" +
				"                     }," +
				"                    \"requested_predicates\":{}" +
				"                  }", gvtSchemaKey);

		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");

		assertEquals(claimsForAttribute1.length(), 1);

		String claimUuid = claimsForAttribute1.getJSONObject(0).getString("referent");

		//9. Prover create Proof
		String selfAttestedValue = "8-800-300";
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"attr2_referent\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":[\"%s\", true]},\n" +
				"                                          \"requested_predicates\":{}\n" +
				"                                        }", selfAttestedValue, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//10. Verifier verify Proof
		assertEquals("Alex",
				proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONArray("attr1_referent").getString(1));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("attr2_referent"));


		proofRequestJson = String.format("{" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\", " +
				"                    \"requested_attrs\": {" +
				"                          \"attr1_referent\":{ \"name\":\"name\", \"restrictions\":[{\"schema_key\":%s}]}" +
				"                     }," +
				"                    \"requested_predicates\":{" +
				"                          \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}" +
				"                    }" +
				"                  }", gvtSchemaKey);

		Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).get();
	}
}
