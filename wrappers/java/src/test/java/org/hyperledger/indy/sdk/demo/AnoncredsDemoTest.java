package org.hyperledger.indy.sdk.demo;

import blob_storage.BlobStorage;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateSchemaResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreCredentialDefResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateCredentialResult;
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
	private String credentialId1 = "id1";
	private String credentialId2 = "id2";
	private String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
	private String proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	private String gvtCredentialValues = "{\n" +
			"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
			"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
			"               \"height\":[\"175\",\"175\"],\n" +
			"               \"age\":[\"28\",\"28\"]\n" +
			"        }";
	private String xyzCredentialValues = "{\n" +
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

		//5. Issuer create CredentialDef
		IssuerCreateAndStoreCredentialDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(issuerWallet, issuerDid, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String credentialDefId = createCredDefResult.getCredDefId();
		String credentialDef = createCredDefResult.getCredDefJson();

		//6. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//7. Issuer create Credential Offer
		String credentialOffer = Anoncreds.issuerCreateCredentialOffer(issuerWallet, credentialDefId, issuerDid, proverDid).get();

		//8. Prover store Credential Offer
		Anoncreds.proverStoreCredentialOffer(proverWallet, credentialOffer).get();

		//9. Prover get Credential Offers
		String credentialOfferFilter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);
		String credentialOffersJson = Anoncreds.proverGetCredentialOffers(proverWallet, credentialOfferFilter).get();

		JSONArray credentialOffersObject = new JSONArray(credentialOffersJson);
		assertEquals(credentialOffersObject.length(), 1);

		JSONObject credentialOfferObject = credentialOffersObject.getJSONObject(0);
		String credentialOfferJson = credentialOfferObject.toString();

		//10. Prover create CredentialReq
		String credentialReq = Anoncreds.proverCreateAndStoreCredentialReq(proverWallet, proverDid, credentialOfferJson, credentialDef, masterSecret).get();

		//11. Issuer create Credential
		IssuerCreateCredentialResult createCredentialResult = Anoncreds.issuerCreateCredentail(issuerWallet, credentialReq, gvtCredentialValues, null, - 1, - 1).get();
		String credentialJson = createCredentialResult.getCredentialJson();

		//12. Prover store Credential
		Anoncreds.proverStoreCredential(proverWallet, credentialId1, credentialJson, null).get();

		//13. Prover gets Credentials for Proof Request
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

		String credentialsForProofJson = Anoncreds.proverGetCredentialsForProofReq(proverWallet, proofRequestJson).get();

		JSONObject credentialsForProof = new JSONObject(credentialsForProofJson);
		JSONArray credentialsForAttribute1 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray credentialsForAttribute2 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray credentialsForPredicate = credentialsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");

		assertEquals(credentialsForAttribute1.length(), 1);
		assertEquals(credentialsForAttribute2.length(), 1);
		assertEquals(credentialsForPredicate.length(), 1);

		String credentialUuid = credentialsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//13. Prover create Proof
		String selfAttestedValue = "8-800-300";
		String requestedCredentialsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"attr3_referent\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true},\n" +
				"                                                               \"attr2_referent\":{\"cred_id\":\"%s\", \"revealed\":false}},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}}\n" +
				"                                        }", selfAttestedValue, credentialUuid, credentialUuid, credentialUuid);

		String schemasJson = String.format("{\"%s\":%s}", credentialUuid, gvtSchemaJson);
		String credentialDefsJson = String.format("{\"%s\":%s}", credentialUuid, credentialDef);
		String revocInfosJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedCredentialsJson, schemasJson,
				masterSecret, credentialDefsJson, revocInfosJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//14. Verifier verify Proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		assertNotNull(proof.getJSONObject("requested_proof").getJSONObject("unrevealed_attrs").getString("attr2_referent"));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("attr3_referent"));

		String id = revealedAttr1.getString("referent");
		schemasJson = String.format("{\"%s\":%s}", id, gvtSchemaJson);
		credentialDefsJson = String.format("{\"%s\":%s}", id, credentialDef);
		String revocRegDefsJson = "{}";
		String revocRegsJson = "{}";

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, credentialDefsJson, revocRegDefsJson, revocRegsJson).get();
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

		//3. Issuer create CredentialDef
		IssuerCreateAndStoreCredentialDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(issuerGvtWallet, issuerDid, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String gvtCredentialDefId = createCredDefResult.getCredDefId();
		String gvtCredentialDef = createCredDefResult.getCredDefJson();

		//4. Issuer2 create XYZ Schema
		String issuerDid2 = "VsKV7grR1BUE29mG2Fm2kX";
		createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid2, XYZ_SCHEMA_NAME, SCHEMA_VERSION, XYZ_SCHEMA_ATTRIBUTES).get();
		String xyzSchemaJson = createSchemaResult.getSchemaJson();

		//5. Issuer create CredentialDef
		createCredDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(issuerXyzWallet, issuerDid2, xyzSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String xyzCredentialDefId = createCredDefResult.getCredDefId();
		String xyzCredentialDef = createCredDefResult.getCredDefJson();

		//6. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//7. Issuer1 create Credential Offer
		String gvtCredentialOffer = Anoncreds.issuerCreateCredentialOffer(issuerGvtWallet, gvtCredentialDefId, issuerDid, proverDid).get();

		//8. Prover store Credential Offer received from Issuer1
		Anoncreds.proverStoreCredentialOffer(proverWallet, gvtCredentialOffer).get();

		//9. Issuer2 create Credential Offer
		String xyzCredentialOffer = Anoncreds.issuerCreateCredentialOffer(issuerXyzWallet, xyzCredentialDefId, issuerDid2, proverDid).get();

		//10. Prover store Credential Offer received from Issuer2
		Anoncreds.proverStoreCredentialOffer(proverWallet, xyzCredentialOffer).get();

		//11. Prover create CredentialReq for GVT Credential Offer
		String gvtCredentialReq = Anoncreds.proverCreateAndStoreCredentialReq(proverWallet, proverDid, gvtCredentialOffer, gvtCredentialDef, masterSecret).get();

		//12. Issuer create Credential
		IssuerCreateCredentialResult gvtCreateCredentialResult = Anoncreds.issuerCreateCredentail(issuerGvtWallet, gvtCredentialReq, gvtCredentialValues, null, - 1, - 1).get();
		String gvtCredentialJson = gvtCreateCredentialResult.getCredentialJson();

		//13. Prover store Credential
		Anoncreds.proverStoreCredential(proverWallet, credentialId1, gvtCredentialJson, null).get();

		//14. Prover create CredentialReq for GVT Credential Offer
		String xyzCredentialReq = Anoncreds.proverCreateAndStoreCredentialReq(proverWallet, proverDid, xyzCredentialOffer, xyzCredentialDef, masterSecret).get();

		//15. Issuer create Credential
		IssuerCreateCredentialResult xyzCreateCredentialResult = Anoncreds.issuerCreateCredentail(issuerXyzWallet, xyzCredentialReq, xyzCredentialValues, null, - 1, - 1).get();
		String xyzCredentialJson = xyzCreateCredentialResult.getCredentialJson();

		//16. Prover store Credential
		Anoncreds.proverStoreCredential(proverWallet, credentialId2, xyzCredentialJson, null).get();

		//17. Prover gets Credentials for Proof Request
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

		String credentialsForProofJson = Anoncreds.proverGetCredentialsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(credentialsForProofJson);

		JSONObject credentialsForProof = new JSONObject(credentialsForProofJson);
		JSONArray credentialsForAttribute1 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray credentialsForAttribute2 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr2_referent");
		JSONArray credentialsForPredicate1 = credentialsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");
		JSONArray credentialsForPredicate2 = credentialsForProof.getJSONObject("predicates").getJSONArray("predicate2_referent");

		assertEquals(credentialsForAttribute1.length(), 1);
		assertEquals(credentialsForAttribute2.length(), 1);
		assertEquals(credentialsForPredicate1.length(), 1);
		assertEquals(credentialsForPredicate2.length(), 1);

		String credentialUuidForAttr1 = credentialsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String credentialUuidForAttr2 = credentialsForAttribute2.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String credentialUuidForPredicate1 = credentialsForPredicate1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String credentialUuidForPredicate2 = credentialsForPredicate2.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//18. Prover create Proof
		String requestedCredentialsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true},\n" +
				"                                                               \"attr2_referent\":{\"cred_id\":\"%s\", \"revealed\":true}},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}," +
				"                                                                    \"predicate2_referent\":{\"cred_id\":\"%s\"}}\n" +
				"                                        }", credentialUuidForAttr1, credentialUuidForAttr2, credentialUuidForPredicate1, credentialUuidForPredicate2);

		String schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", credentialUuidForAttr1, gvtSchemaJson, credentialUuidForAttr2, xyzSchemaJson);
		String credentialDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", credentialUuidForAttr1, gvtCredentialDef, credentialUuidForAttr2, xyzCredentialDef);
		String revocInfosJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedCredentialsJson, schemasJson,
				masterSecret, credentialDefsJson, revocInfosJson).get();
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
		credentialDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", subProofId1, gvtCredentialDef, subProofId2, xyzCredentialDef);
		String revocRegDefsJson = "{}";
		String revocRegsJson = "{}";

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, credentialDefsJson, revocRegDefsJson, revocRegsJson).get();
		assertTrue(valid);

		//20. Close and delete Issuer2 Wallet
		issuerXyzWallet.closeWallet().get();
		Wallet.deleteWallet("issuer2Wallet", null).get();
	}

	@Test
	public void testAnoncredsWorksForSingleIssuerSingleProverMultiplyCredentials() throws Exception {
		//1. Issuer create GVT Schema
		IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String gvtSchemaJson = createSchemaResult.getSchemaJson();

		//2. Issuer create CredentialDef
		IssuerCreateAndStoreCredentialDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(issuerWallet, issuerDid, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String gvtCredentialDefId = createCredDefResult.getCredDefId();
		String gvtCredentialDef = createCredDefResult.getCredDefJson();

		//3. Issuer create XYZ Schema
		createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, XYZ_SCHEMA_NAME, SCHEMA_VERSION, XYZ_SCHEMA_ATTRIBUTES).get();
		String xyzSchemaJson = createSchemaResult.getSchemaJson();

		//4. Issuer create CredentialDef
		createCredDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(issuerWallet, issuerDid, xyzSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String xyzCredentialDefId = createCredDefResult.getCredDefId();
		String xyzCredentialDef = createCredDefResult.getCredDefJson();

		//4. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//5. Issuer create GVT Credential Offer
		String gvtCredentialOffer = Anoncreds.issuerCreateCredentialOffer(issuerWallet, gvtCredentialDefId, issuerDid, proverDid).get();

		//6. Prover store GVT Credential Offer
		Anoncreds.proverStoreCredentialOffer(proverWallet, gvtCredentialOffer).get();

		//7. Issuer create XYZ Credential Offer
		String xyzCredentialOffer = Anoncreds.issuerCreateCredentialOffer(issuerWallet, xyzCredentialDefId, issuerDid, proverDid).get();

		//8. Prover store XYZ Credential Offer
		Anoncreds.proverStoreCredentialOffer(proverWallet, xyzCredentialOffer).get();

		//9. Prover create CredentialReq for GVT Credential Offer
		String gvtCredentialReq = Anoncreds.proverCreateAndStoreCredentialReq(proverWallet, proverDid, gvtCredentialOffer, gvtCredentialDef, masterSecret).get();

		//10. Issuer create GVT Credential
		IssuerCreateCredentialResult gvtCreateCredentialResult = Anoncreds.issuerCreateCredentail(issuerWallet, gvtCredentialReq, gvtCredentialValues, null, - 1, - 1).get();
		String gvtCredentialJson = gvtCreateCredentialResult.getCredentialJson();

		//11. Prover store GVT Credential
		Anoncreds.proverStoreCredential(proverWallet, credentialId1, gvtCredentialJson, null).get();

		//12. Prover create CredentialReq for XYZ Credential Offer
		String xyzCredentialReq = Anoncreds.proverCreateAndStoreCredentialReq(proverWallet, proverDid, xyzCredentialOffer, xyzCredentialDef, masterSecret).get();

		//13. Issuer create XYZ Credential
		IssuerCreateCredentialResult xyzCreateCredentialResult = Anoncreds.issuerCreateCredentail(issuerWallet, xyzCredentialReq, xyzCredentialValues, null, - 1, - 1).get();
		String xyzCredentialJson = xyzCreateCredentialResult.getCredentialJson();

		//14. Prover store XYZ Credential
		Anoncreds.proverStoreCredential(proverWallet, credentialId2, xyzCredentialJson, null).get();

		//15. Prover gets Credentials for Proof Request
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

		String credentialsForProofJson = Anoncreds.proverGetCredentialsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(credentialsForProofJson);

		JSONObject credentialsForProof = new JSONObject(credentialsForProofJson);
		JSONArray credentialsForAttribute1 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray credentialsForAttribute2 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr2_referent");
		JSONArray credentialsForPredicate1 = credentialsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");
		JSONArray credentialsForPredicate2 = credentialsForProof.getJSONObject("predicates").getJSONArray("predicate2_referent");

		assertEquals(credentialsForAttribute1.length(), 1);
		assertEquals(credentialsForAttribute2.length(), 1);
		assertEquals(credentialsForPredicate1.length(), 1);
		assertEquals(credentialsForPredicate2.length(), 1);

		String credentialUuidForAttr1 = credentialsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String credentialUuidForAttr2 = credentialsForAttribute2.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String credentialUuidForPredicate1 = credentialsForPredicate1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String credentialUuidForPredicate2 = credentialsForPredicate2.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//16. Prover create Proof
		String requestedCredentialsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true},\n" +
				"                                                               \"attr2_referent\":{\"cred_id\":\"%s\", \"revealed\":true}},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}," +
				"                                                                    \"predicate2_referent\":{\"cred_id\":\"%s\"}}\n" +
				"                                        }", credentialUuidForAttr1, credentialUuidForAttr2, credentialUuidForPredicate1, credentialUuidForPredicate2);

		String schemasJson = String.format("{\"%s\":%s, \"%s\":%s}", credentialUuidForAttr1, gvtSchemaJson, credentialUuidForAttr2, xyzSchemaJson);
		String credentialDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", credentialUuidForAttr1, gvtCredentialDef, credentialUuidForAttr2, xyzCredentialDef);
		String revocInfosJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedCredentialsJson, schemasJson,
				masterSecret, credentialDefsJson, revocInfosJson).get();
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
		credentialDefsJson = String.format("{\"%s\":%s, \"%s\":%s}", subProofId1, gvtCredentialDef, subProofId2, xyzCredentialDef);
		String revocRegDefsJson = "{}";
		String revocRegsJson = "{}";

		Boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, credentialDefsJson, revocRegDefsJson, revocRegsJson).get();
		assertTrue(valid);
	}

	@Test
	public void testAnoncredsWorksForRevocationProof() throws Exception {

		//1. Issuer create Schema
		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String schemaJson = createSchemaResult.getSchemaJson();

		//2. Issuer create credential definition
		String revocationCredentialDefConfig = "{\"support_revocation\":true}";
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredentialDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(issuerWallet, issuerDid, schemaJson, TAG, null, revocationCredentialDefConfig).get();
		String credentialDefId = createCredentialDefResult.getCredDefId();
		String credentialDefJson = createCredentialDefResult.getCredDefJson();

		//3. Issuer create revocation registry
		String revRegConfig = "{\"issuance_type\":null,\"max_cred_num\":5}";
		String tailsWriterConfig = String.format("{\"base_dir\":\"%s\", \"uri_pattern\":\"\"}", getIndyHomePath("tails"));

		AnoncredsResults.IssuerCreateAndStoreRevocRegResult createRevRegResult = Anoncreds.issuerCreateAndStoreRevocReg(issuerWallet, issuerDid, null, TAG, credentialDefId, revRegConfig, "default", tailsWriterConfig).get();
		String revRegId = createRevRegResult.getRevRegId();
		String revRegDef = createRevRegResult.getRevRegDefJson();

		//4. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//5. Issuer create Credential Offer
		String credentialOfferJson = Anoncreds.issuerCreateCredentialOffer(issuerWallet, credentialDefId, issuerDid, proverDid).get();

		//6. Prover store Credential Offer received from Issuer
		Anoncreds.proverStoreCredentialOffer(proverWallet, credentialOfferJson).get();

		//7. Prover create Credential Request
		String credentialReq = Anoncreds.proverCreateAndStoreCredentialReq(proverWallet, proverDid, credentialOfferJson, credentialDefJson, masterSecret).get();

		//8. Issuer open TailsReader
		JSONObject revRegDeg = new JSONObject(revRegDef);
		BlobStorage tailsReader = BlobStorage.openReader("default",
				tailsWriterConfig,
				revRegDeg.getJSONObject("value").getString("tails_location"),
				revRegDeg.getJSONObject("value").getString("tails_hash")).get();

		//9. Issuer create Credential
		int userRevocIndex = 1;
		IssuerCreateCredentialResult createCredentialResult = Anoncreds.issuerCreateCredentail(issuerWallet, credentialReq, gvtCredentialValues, revRegId, tailsReader.getTailsReaderHandle(), userRevocIndex).get();
		String credential = createCredentialResult.getCredentialJson();
		String revRegDelta = createCredentialResult.getRevocRegDeltaJson();

		//10. Prover create RevocationInfo
		int timestamp = 100;
		String revInfo = Anoncreds.createRevocationInfo(tailsReader.getTailsReaderHandle(), revRegDef, revRegDelta, timestamp, userRevocIndex).get();

		//11. Prover store RevocationInfo
		Anoncreds.storeRevocationInfo(proverWallet, credentialId1, revInfo).get();

		//12. Prover store received Credential
		Anoncreds.proverStoreCredential(proverWallet, credentialId1, credential, revRegDef).get();

		//13. Prover gets Credentials for Proof Request
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

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(proverWallet, proofRequest).get();
		JSONObject credentials = new JSONObject(credentialsJson);
		JSONArray credentialsForAttr1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");

		String credentialUuid = credentialsForAttr1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//15. Prover create Proof
		String requestedCredentialsJson = String.format("{" +
				"\"self_attested_attributes\":{}," +
				"\"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true, \"timestamp\":%d }}," +
				"\"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\", \"timestamp\":%d}}" +
				"}", credentialUuid, timestamp, credentialUuid, timestamp);

		String schemasJson = String.format("{\"%s\":%s}", credentialUuid, schemaJson);
		String credentialDefsJson = String.format("{\"%s\":%s}", credentialUuid, credentialDefJson);
		String revInfos = String.format("{\"%s\": { \"%s\":%s }}", credentialUuid, timestamp, revInfo);

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequest, requestedCredentialsJson, schemasJson, masterSecret,
				credentialDefsJson, revInfos).get();
		JSONObject proof = new JSONObject(proofJson);

		//15. Verifier verify proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		String id = revealedAttr1.getString("referent");

		schemasJson = String.format("{\"%s\":%s}", id, schemaJson);
		credentialDefsJson = String.format("{\"%s\":%s}", id, credentialDefJson);
		String revRegDefsJson = String.format("{\"%s\":%s}", id, revRegDef);
		String revRegs = String.format("{\"%s\": { \"%s\":%s }}", id, timestamp, revRegDelta);

		boolean valid = Anoncreds.verifierVerifyProof(proofRequest, proofJson, schemasJson, credentialDefsJson, revRegDefsJson, revRegs).get();
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

		//2. Issuer create CredentialDef
		IssuerCreateAndStoreCredentialDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(issuerWallet, issuerDid, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String credentialDefId = createCredDefResult.getCredDefId();
		String credentialDef = createCredDefResult.getCredDefJson();

		//3. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecret).get();

		//4. Issuer create Credential Offer
		String credentialOfferJson = Anoncreds.issuerCreateCredentialOffer(issuerWallet, credentialDefId, issuerDid, proverDid).get();

		//5. Prover store Credential Offer
		Anoncreds.proverStoreCredentialOffer(proverWallet, credentialOfferJson).get();

		//6. Prover create CredentialReq
		String credentialReq = Anoncreds.proverCreateAndStoreCredentialReq(proverWallet, proverDid, credentialOfferJson, credentialDef, masterSecret).get();

		//7. Issuer create Credential
		IssuerCreateCredentialResult createCredentialResult = Anoncreds.issuerCreateCredentail(issuerWallet, credentialReq, gvtCredentialValues, null, - 1, - 1).get();
		String credentialJson = createCredentialResult.getCredentialJson();

		//8. Prover store Credential
		Anoncreds.proverStoreCredential(proverWallet, credentialId1, credentialJson, null).get();

		//9. Prover gets Credentials for Proof Request
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

		String credentialsForProofJson = Anoncreds.proverGetCredentialsForProofReq(proverWallet, proofRequestJson).get();
		assertNotNull(credentialsForProofJson);

		JSONObject credentialsForProof = new JSONObject(credentialsForProofJson);
		JSONArray credentialsForAttribute1 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");

		assertEquals(credentialsForAttribute1.length(), 1);

		String credentialUuid = credentialsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//9. Prover create Proof
		String selfAttestedValue = "8-800-300";
		String requestedCredentialsJson = String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"attr2_referent\":\"%s\"},\n" +
				"                                          \"requested_attrs\":{\"attr1_referent\":[\"%s\", true]},\n" +
				"                                          \"requested_predicates\":{}\n" +
				"                                        }", selfAttestedValue, credentialUuid);

		String schemasJson = String.format("{\"%s\":%s}", credentialUuid, gvtSchemaJson);
		String credentialDefsJson = String.format("{\"%s\":%s}", credentialUuid, credentialDef);
		String revocInfosJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedCredentialsJson, schemasJson,
				masterSecret, credentialDefsJson, revocInfosJson).get();
		assertNotNull(proofJson);

		JSONObject proof = new JSONObject(proofJson);

		//10. Verifier verify Proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("attr3_referent"));

		String id = revealedAttr1.getString("referent");
		schemasJson = String.format("{\"%s\":%s}", id, gvtSchemaJson);
		credentialDefsJson = String.format("{\"%s\":%s}", id, credentialDef);
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



		Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemasJson, credentialDefsJson, revocRegDefsJson, revocRegsJson).get();
	}
}
