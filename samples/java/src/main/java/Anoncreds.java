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

		//4. Issuer Creates Credential Schema
		String schemaName = "gvt";
		String schemaVersion = "1.0";
		String schemaAttributes = "[\"name\", \"age\", \"sex\", \"height\"]";
		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult =
				issuerCreateSchema(issuerDid, schemaName, schemaVersion, schemaAttributes).get();
		String schemaId = createSchemaResult.getSchemaId();
		String schemaJson = createSchemaResult.getSchemaJson();

		//5. Issuer create Credential Definition
		String credDefTag = "Tag1";
		String credDefConfigJson = "{\"support_revocation\":false}";
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredDefResult =
				issuerCreateAndStoreCredentialDef(issuerWallet, issuerDid, schemaJson, credDefTag, null, credDefConfigJson).get();
		String credDefId = createCredDefResult.getCredDefId();
		String credDefJson = createCredDefResult.getCredDefJson();

		//6. Prover create Master Secret
		String masterSecretId = proverCreateMasterSecret(proverWallet, null).get();

		//7. Issuer Creates Credential Offer
		String credOffer = issuerCreateCredentialOffer(issuerWallet, credDefId).get();

		//8. Prover Creates Credential Request
		AnoncredsResults.ProverCreateCredentialRequestResult createCredReqResult =
				proverCreateCredentialReq(proverWallet, proverDid, credOffer, credDefJson, masterSecretId).get();
		String credReqJson = createCredReqResult.getCredentialRequestJson();
		String credReqMetadataJson = createCredReqResult.getCredentialRequestMetadataJson();

		//9. Issuer create Credential
		String credValuesJson = new JSONObject("{\n" +
				"        \"sex\": {\"raw\": \"male\", \"encoded\": \"594465709955896723921094925839488742869205008160769251991705001\"},\n" +
				"        \"name\": {\"raw\": \"Alex\", \"encoded\": \"1139481716457488690172217916278103335\"},\n" +
				"        \"height\": {\"raw\": \"175\", \"encoded\": \"175\"},\n" +
				"        \"age\": {\"raw\": \"28\", \"encoded\": \"28\"}\n" +
				"    }").toString();

		AnoncredsResults.IssuerCreateCredentialResult createCredentialResult =
				issuerCreateCredential(issuerWallet, credOffer, credReqJson, credValuesJson, null, - 1).get();
		String credential = createCredentialResult.getCredentialJson();

		//10. Prover Stores Credential
		proverStoreCredential(proverWallet, null, credReqJson, credReqMetadataJson, credential, credDefJson, null).get();

		//11. Prover Gets Credentials for Proof Request
		String proofRequestJson = new JSONObject("{" +
				"                    \"nonce\":\"123432421212\",\n" +
				"                    \"name\":\"proof_req_1\",\n" +
				"                    \"version\":\"0.1\", " +
				"                    \"requested_attributes\": {" +
				"                          \"attr1_referent\":{\"name\":\"name\"}," +
				"                          \"attr2_referent\":{\"name\":\"sex\"}," +
				"                          \"attr3_referent\":{\"name\":\"phone\"}" +
				"                     }," +
				"                    \"requested_predicates\":{" +
				"                         \"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}" +
				"                    }" +
				"                  }").toString();

		String credentialsForProofJson = proverGetCredentialsForProofReq(proverWallet, proofRequestJson).get();

		JSONObject credentialsForProof = new JSONObject(credentialsForProofJson);
		JSONArray credentialsForAttribute1 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray credentialsForAttribute2 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr2_referent");
		JSONArray credentialsForAttribute3 = credentialsForProof.getJSONObject("attrs").getJSONArray("attr3_referent");
		JSONArray credentialsForPredicate = credentialsForProof.getJSONObject("predicates").getJSONArray("predicate1_referent");

		assertEquals(credentialsForAttribute1.length(), 1);
		assertEquals(credentialsForAttribute2.length(), 1);
		assertEquals(credentialsForAttribute3.length(), 0);
		assertEquals(credentialsForPredicate.length(), 1);

		String credentialId = credentialsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//12. Prover Creates Proof
		String selfAttestedValue = "8-800-300";
		String requestedCredentialsJson = new JSONObject(String.format("{\n" +
				"                                          \"self_attested_attributes\":{\"attr3_referent\":\"%s\"},\n" +
				"                                          \"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true},\n" +
				"                                                                    \"attr2_referent\":{\"cred_id\":\"%s\", \"revealed\":false}},\n" +
				"                                          \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}}\n" +
				"                                        }", selfAttestedValue, credentialId, credentialId, credentialId)).toString();

		String schemas = new JSONObject(String.format("{\"%s\":%s}", schemaId, schemaJson)).toString();
		String credentialDefs = new JSONObject(String.format("{\"%s\":%s}", credDefId, credDefJson)).toString();
		String revocStates = new JSONObject("{}").toString();

		String proofJson = proverCreateProof(proverWallet, proofRequestJson, requestedCredentialsJson,
				masterSecretId, schemas, credentialDefs, revocStates).get();
		JSONObject proof = new JSONObject(proofJson);

		//13. Verifier verify Proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		assertNotNull(proof.getJSONObject("requested_proof").getJSONObject("unrevealed_attrs").getJSONObject("attr2_referent").getInt("sub_proof_index"));

		assertEquals(selfAttestedValue, proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs").getString("attr3_referent"));

		String revocRegDefs = new JSONObject("{}").toString();
		String revocRegs = new JSONObject("{}").toString();

		Boolean valid = verifierVerifyProof(proofRequestJson, proofJson, schemas, credentialDefs, revocRegDefs, revocRegs).get();
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
