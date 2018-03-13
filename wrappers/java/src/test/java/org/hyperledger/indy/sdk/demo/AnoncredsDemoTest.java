package org.hyperledger.indy.sdk.demo;

import blob_storage.BlobStorage;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateSchemaResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreClaimDefResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateClaimResult;
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
import static org.hyperledger.indy.sdk.utils.EnvironmentUtils.getIndyHomePath;
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
	private String claimId1 = "id1";
	private String claimId2 = "id2";
	private String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
	private String proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	private String gvtClaimValues = "{\n" +
			"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
			"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
			"               \"height\":[\"175\",\"175\"],\n" +
			"               \"age\":[\"28\",\"28\"]\n" +
			"        }";
	private String xyzClaimValues = "{\n" +
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

		//4. Issuer create Schema
		IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String gvtSchemaJson = createSchemaResult.getSchemaJson();

		//5. Issuer create ClaimDef
		IssuerCreateAndStoreClaimDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String claimDefId = createCredDefResult.getClaimDefId();
		String claimDef = createCredDefResult.getClaimDefJson();

		//6. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//7. Issuer create Claim Offer
		String claimOffer = Anoncreds.issuerCreateClaimOffer(issuerWallet, claimDefId, issuerDid, proverDid).get();

		//8. Prover store Claim Offer
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOffer).get();

		//9. Prover get Claim Offers
		String claimOfferFilter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);
		String claimOffersJson = Anoncreds.proverGetClaimOffers(proverWallet, claimOfferFilter).get();

		JSONArray claimOffersObject = new JSONArray(claimOffersJson);
		assertEquals(claimOffersObject.length(), 1);

		JSONObject claimOfferObject = claimOffersObject.getJSONObject(0);
		String claimOfferJson = claimOfferObject.toString();

		//10. Prover create ClaimReq
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).get();

		//11. Issuer create Claim
		IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, gvtClaimValues, null, - 1, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		//12. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, claimId1, claimJson, null).get();

		//13. Prover gets Claims for Proof Request
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

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray claimsForAttribute2 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray claimsForPredicate = claimsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");

		assertEquals(claimsForAttribute1.length(), 1);
		assertEquals(claimsForAttribute2.length(), 1);
		assertEquals(claimsForPredicate.length(), 1);

		String claimUuid = claimsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//13. Prover create Proof
		String selfAttestedValue = "8-800-300";
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"attr3_referent\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true},\n" +
				"                                                               \"attr2_referent\":{\"cred_id\":\"%s\", \"revealed\":false}},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}}\n" +
				"                                        }", selfAttestedValue, claimUuid, claimUuid, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocInfosJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocInfosJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//14. Verifier verify Proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		assertNotNull(proof.getJSONObject("requested_proof").getJSONObject("unrevealed_attrs").getString("attr2_referent"));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("attr3_referent"));

		String id = revealedAttr1.getString("referent");
		schemasJson = String.format("{\"%s\":%s}", id, gvtSchemaJson);
		claimDefsJson = String.format("{\"%s\":%s}", id, claimDef);
		String revocRegDefsJson = "{}";
		String revocRegsJson = "{}";

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegDefsJson, revocRegsJson).get();
		assertTrue(valid);
	}

	@Test
	public void testAnoncredsWorksForMultiplyIssuerSingleProver() throws Exception {

		Wallet issuerGvtWallet = issuerWallet;

		//1. Issuer2 Create and Open Wallet
		Wallet.createWallet(poolName, "issuer2Wallet", "default", null, null).get();
		Wallet issuerXyzWallet = Wallet.openWallet("issuer2Wallet", null, null).get();

		//2. Issuer1 create GVT Schema
		IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String gvtSchemaJson = createSchemaResult.getSchemaJson();

		//3. Issuer create ClaimDef
		IssuerCreateAndStoreClaimDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreClaimDef(issuerGvtWallet, issuerDid, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String gvtClaimDefId = createCredDefResult.getClaimDefId();
		String gvtClaimDef = createCredDefResult.getClaimDefJson();

		//4. Issuer2 create XYZ Schema
		String issuerDid2 = "VsKV7grR1BUE29mG2Fm2kX";
		createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid2, XYZ_SCHEMA_NAME, SCHEMA_VERSION, XYZ_SCHEMA_ATTRIBUTES).get();
		String xyzSchemaJson = createSchemaResult.getSchemaJson();

		//5. Issuer create ClaimDef
		createCredDefResult = Anoncreds.issuerCreateAndStoreClaimDef(issuerXyzWallet, issuerDid2, xyzSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String xyzClaimDefId = createCredDefResult.getClaimDefId();
		String xyzClaimDef = createCredDefResult.getClaimDefJson();

		//6. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//7. Issuer1 create Claim Offer
		String gvtClaimOffer = Anoncreds.issuerCreateClaimOffer(issuerGvtWallet, gvtClaimDefId, issuerDid, proverDid).get();

		//8. Prover store Claim Offer received from Issuer1
		Anoncreds.proverStoreClaimOffer(proverWallet, gvtClaimOffer).get();

		//9. Issuer2 create Claim Offer
		String xyzClaimOffer = Anoncreds.issuerCreateClaimOffer(issuerXyzWallet, xyzClaimDefId, issuerDid2, proverDid).get();

		//10. Prover store Claim Offer received from Issuer2
		Anoncreds.proverStoreClaimOffer(proverWallet, xyzClaimOffer).get();

		//11. Prover create ClaimReq for GVT Claim Offer
		String gvtClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret).get();

		//12. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult gvtCreateClaimResult = Anoncreds.issuerCreateClaim(issuerGvtWallet, gvtClaimReq, gvtClaimValues, null, - 1, - 1).get();
		String gvtClaimJson = gvtCreateClaimResult.getClaimJson();

		//13. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, claimId1, gvtClaimJson, null).get();

		//14. Prover create ClaimReq for GVT Claim Offer
		String xyzClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret).get();

		//15. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult xyzCreateClaimResult = Anoncreds.issuerCreateClaim(issuerXyzWallet, xyzClaimReq, xyzClaimValues, null, - 1, - 1).get();
		String xyzClaimJson = xyzCreateClaimResult.getClaimJson();

		//16. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, claimId2, xyzClaimJson, null).get();

		//17. Prover gets Claims for Proof Request
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

		String claimUuidForAttr1 = claimsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String claimUuidForAttr2 = claimsForAttribute2.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String claimUuidForPredicate1 = claimsForPredicate1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String claimUuidForPredicate2 = claimsForPredicate2.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//18. Prover create Proof
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true},\n" +
				"                                                               \"attr2_referent\":{\"cred_id\":\"%s\", \"revealed\":true}},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}," +
				"                                                                    \"predicate2_referent\":{\"cred_id\":\"%s\"}}\n" +
				"                                        }", claimUuidForAttr1, claimUuidForAttr2, claimUuidForPredicate1, claimUuidForPredicate2);

		String schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtSchemaJson, claimUuidForAttr2, xyzSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtClaimDef, claimUuidForAttr2, xyzClaimDef);
		String revocInfosJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocInfosJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//19. Verifier verify Proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		JSONObject revealedAttr2 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr2_referent");
		assertEquals("partial", revealedAttr2.getString("raw"));

		String subProofId1 = revealedAttr1.getString("referent");
		String subProofId2 = revealedAttr2.getString("referent");
		schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", subProofId1, gvtSchemaJson, subProofId2, xyzSchemaJson);
		claimDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", subProofId1, gvtClaimDef, subProofId2, xyzClaimDef);
		String revocRegDefsJson = "{}";
		String revocRegsJson = "{}";

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegDefsJson, revocRegsJson).get();
		assertTrue(valid);

		//20. Close and delete Issuer2 Wallet
		issuerXyzWallet.closeWallet().get();
		Wallet.deleteWallet("issuer2Wallet", null).get();
	}

	@Test
	public void testAnoncredsWorksForSingleIssuerSingleProverMultiplyClaims() throws Exception {
		//1. Issuer create GVT Schema
		IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String gvtSchemaJson = createSchemaResult.getSchemaJson();

		//2. Issuer create ClaimDef
		IssuerCreateAndStoreClaimDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String gvtClaimDefId = createCredDefResult.getClaimDefId();
		String gvtClaimDef = createCredDefResult.getClaimDefJson();

		//3. Issuer create XYZ Schema
		createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, XYZ_SCHEMA_NAME, SCHEMA_VERSION, XYZ_SCHEMA_ATTRIBUTES).get();
		String xyzSchemaJson = createSchemaResult.getSchemaJson();

		//4. Issuer create ClaimDef
		createCredDefResult = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, xyzSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String xyzClaimDefId = createCredDefResult.getClaimDefId();
		String xyzClaimDef = createCredDefResult.getClaimDefJson();

		//4. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//5. Issuer create GVT Claim Offer
		String gvtClaimOffer = Anoncreds.issuerCreateClaimOffer(issuerWallet, gvtClaimDefId, issuerDid, proverDid).get();

		//6. Prover store GVT Claim Offer
		Anoncreds.proverStoreClaimOffer(proverWallet, gvtClaimOffer).get();

		//7. Issuer create XYZ Claim Offer
		String xyzClaimOffer = Anoncreds.issuerCreateClaimOffer(issuerWallet, xyzClaimDefId, issuerDid, proverDid).get();

		//8. Prover store XYZ Claim Offer
		Anoncreds.proverStoreClaimOffer(proverWallet, xyzClaimOffer).get();

		//9. Prover create ClaimReq for GVT Claim Offer
		String gvtClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret).get();

		//10. Issuer create GVT Claim
		AnoncredsResults.IssuerCreateClaimResult gvtCreateClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, gvtClaimReq, gvtClaimValues, null, - 1, - 1).get();
		String gvtClaimJson = gvtCreateClaimResult.getClaimJson();

		//11. Prover store GVT Claim
		Anoncreds.proverStoreClaim(proverWallet, claimId1, gvtClaimJson, null).get();

		//12. Prover create ClaimReq for XYZ Claim Offer
		String xyzClaimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret).get();

		//13. Issuer create XYZ Claim
		AnoncredsResults.IssuerCreateClaimResult xyzCreateClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, xyzClaimReq, xyzClaimValues, null, - 1, - 1).get();
		String xyzClaimJson = xyzCreateClaimResult.getClaimJson();

		//14. Prover store XYZ Claim
		Anoncreds.proverStoreClaim(proverWallet, claimId2, xyzClaimJson, null).get();

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

		String claimUuidForAttr1 = claimsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String claimUuidForAttr2 = claimsForAttribute2.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String claimUuidForPredicate1 = claimsForPredicate1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String claimUuidForPredicate2 = claimsForPredicate2.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//16. Prover create Proof
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true},\n" +
				"                                                               \"attr2_referent\":{\"cred_id\":\"%s\", \"revealed\":true}},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}," +
				"                                                                    \"predicate2_referent\":{\"cred_id\":\"%s\"}}\n" +
				"                                        }", claimUuidForAttr1, claimUuidForAttr2, claimUuidForPredicate1, claimUuidForPredicate2);

		String schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtSchemaJson, claimUuidForAttr2, xyzSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", claimUuidForAttr1, gvtClaimDef, claimUuidForAttr2, xyzClaimDef);
		String revocInfosJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocInfosJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//17. Verifier verify Proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		JSONObject revealedAttr2 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr2_referent");
		assertEquals("partial", revealedAttr2.getString("raw"));

		String subProofId1 = revealedAttr1.getString("referent");
		String subProofId2 = revealedAttr2.getString("referent");
		schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", subProofId1, gvtSchemaJson, subProofId2, xyzSchemaJson);
		claimDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", subProofId1, gvtClaimDef, subProofId2, xyzClaimDef);
		String revocRegDefsJson = "{}";
		String revocRegsJson = "{}";

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegDefsJson, revocRegsJson).get();
		assertTrue(valid);
	}

	@Test
	public void testAnoncredsWorksForNonRevocedProof() throws Exception {

		//1. Issuer create Schema
		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String schemaJson = createSchemaResult.getSchemaJson();

		//2. Issuer create claim definition
		String revocationCredentialDefConfig = "{\"support_revocation\":true}";
		AnoncredsResults.IssuerCreateAndStoreClaimDefResult createClaimDefResult = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, schemaJson, TAG, null, revocationCredentialDefConfig).get();
		String claimDefId = createClaimDefResult.getClaimDefId();
		String claimDefJson = createClaimDefResult.getClaimDefJson();

		//3. Issuer create revocation registry
		String revRegConfig = "{\"issuance_type\":null,\"max_cred_num\":5}";
		String tailsWriterConfig = String.format("{\"base_dir\":\"%s\", \"uri_pattern\":\"\"}", getIndyHomePath("tails"));

		AnoncredsResults.IssuerCreateAndStoreRevocRegResult createRevRegResult = Anoncreds.issuerCreateAndStoreRevocReg(issuerWallet, issuerDid, null, TAG, claimDefId, revRegConfig, "default", tailsWriterConfig).get();
		String revRegId = createRevRegResult.getRevRegId();
		String revRegDef = createRevRegResult.getRevRegDefJson();

		//4. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//5. Issuer create Claim Offer
		String claimOfferJson = Anoncreds.issuerCreateClaimOffer(issuerWallet, claimDefId, issuerDid, proverDid).get();

		//6. Prover store Claim Offer received from Issuer
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOfferJson).get();

		//7. Prover create Claim Request
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOfferJson, claimDefJson, masterSecret).get();

		//8. Issuer open TailsReader
		JSONObject revRegDeg = new JSONObject(revRegDef);
		BlobStorage tailsReader = BlobStorage.openReader("default",
				tailsWriterConfig,
				revRegDeg.getJSONObject("value").getString("tails_location"),
				revRegDeg.getJSONObject("value").getString("tails_hash")).get();

		//9. Issuer create Claim
		int userRevocIndex = 1;
		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, gvtClaimValues, revRegId, tailsReader.getTailsReaderHandle(), userRevocIndex).get();
		String claim = createClaimResult.getClaimJson();
		String revRegDelta = createClaimResult.getRevocRegDeltaJson();

		//10. Prover create RevocationInfo
		int timestamp = 100;
		String revInfo = Anoncreds.createRevocationInfo(tailsReader.getTailsReaderHandle(), revRegDef, revRegDelta, timestamp, userRevocIndex).get();

		//11. Prover store RevocationInfo
		Anoncreds.storeRevocationInfo(proverWallet, claimId1, revInfo).get();

		//12. Prover store received Claim
		Anoncreds.proverStoreClaim(proverWallet, claimId1, claim, revRegDef).get();

		//13. Prover gets Claims for Proof Request
		String proofRequest = "{\n" +
				"                   \"nonce\":\"123432421212\",\n" +
				"                   \"name\":\"proof_req_1\",\n" +
				"                   \"version\":\"0.1\", " +
				"                   \"requested_attrs\":{" +
				"                          \"attr1_referent\":{\"name\":\"name\"}" +
				"                    },\n" +
				"                    \"requested_predicates\":{" +
				"                          \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}" +
				"                    }" +
				"               }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequest).get();
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

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequest, requestedClaimsJson, schemasJson, masterSecret,
				claimDefsJson, revInfos).get();
		JSONObject proof = new JSONObject(proofJson);

		//15. Verifier verify proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		String id = revealedAttr1.getString("referent");

		schemasJson = String.format("{\"%s\":%s}", id, schemaJson);
		claimDefsJson = String.format("{\"%s\":%s}", id, claimDefJson);
		String revRegDefsJson = String.format("{\"%s\":%s}", id, revRegDef);
		String revRegs = String.format("{\"%s\": { \"%s\":%s }}", id, timestamp, revRegDelta);

		boolean valid = Anoncreds.verifierVerifyProof(proofRequest, proofJson, schemasJson, claimDefsJson, revRegDefsJson, revRegs).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyProofWorksForProofDoesNotCorrespondToProofRequest() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		//1. Issuer create Schema
		IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String gvtSchemaId = createSchemaResult.getSchemaId();
		String gvtSchemaJson = createSchemaResult.getSchemaJson();

		//2. Issuer create ClaimDef
		IssuerCreateAndStoreClaimDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String claimDefId = createCredDefResult.getClaimDefId();
		String claimDef = createCredDefResult.getClaimDefJson();

		//3. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//4. Issuer create Claim Offer
		String claimOfferJson = Anoncreds.issuerCreateClaimOffer(issuerWallet, claimDefId, issuerDid, proverDid).get();

		//5. Prover store Claim Offer
		Anoncreds.proverStoreClaimOffer(proverWallet, claimOfferJson).get();

		//6. Prover create ClaimReq
		String claimReq = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).get();

		//7. Issuer create Claim
		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, gvtClaimValues, null, - 1, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		//8. Prover store Claim
		Anoncreds.proverStoreClaim(proverWallet, claimId1, claimJson, null).get();

		//9. Prover gets Claims for Proof Request
		String proofRequestJson = String.format("{" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\", " +
				"                    \"requested_attrs\": {" +
				"                          \"attr1_referent\":{ \"name\":\"name\", \"restrictions\":[{\"schema_id\":%s}]}," +
				"                          \"attr2_referent\":{ \"name\":\"phone\"}" +
				"                     }," +
				"                    \"requested_predicates\":{}" +
				"                  }", gvtSchemaId);

		String claimsForProofJson = Anoncreds.proverGetClaimsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(claimsForProofJson);

		JSONObject claimsForProof = new JSONObject(claimsForProofJson);
		JSONArray claimsForAttribute1 = claimsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");

		assertEquals(claimsForAttribute1.length(), 1);

		String claimUuid = claimsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//9. Prover create Proof
		String selfAttestedValue = "8-800-300";
		String requestedClaimsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"attr2_referent\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":[\"%s\", true]},\n" +
				"                                          \"requested_predicates\":{}\n" +
				"                                        }", selfAttestedValue, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocInfosJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
				masterSecret, claimDefsJson, revocInfosJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//10. Verifier verify Proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("attr3_referent"));

		String id = revealedAttr1.getString("referent");
		schemasJson = String.format("{\"%s\":%s}", id, gvtSchemaJson);
		claimDefsJson = String.format("{\"%s\":%s}", id, claimDef);
		String revocRegDefsJson = "{}";
		String revocRegsJson = "{}";

		proofRequestJson = String.format("{" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\", " +
				"                    \"requested_attrs\": {" +
				"                          \"attr1_referent\":{ \"name\":\"name\", \"restrictions\":[{\"schema_id\":%s}]}" +
				"                     }," +
				"                    \"requested_predicates\":{" +
				"                          \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}" +
				"                    }" +
				"                  }", gvtSchemaId);



		Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegDefsJson, revocRegsJson).get();
	}
}
