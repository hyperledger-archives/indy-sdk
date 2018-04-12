import org.hyperledger.indy.sdk.anoncreds.*;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.blob_storage.BlobStorageReader;
import org.hyperledger.indy.sdk.blob_storage.BlobStorageWriter;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import utils.PoolUtils;

import static org.hyperledger.indy.sdk.anoncreds.Anoncreds.*;
import static org.junit.Assert.*;
import static utils.EnvironmentUtils.getIndyHomePath;


class AnoncredsRevocation {

	static void demo() throws Exception {
		System.out.println("Anoncreds Revocation sample -> started");

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
		String credDefConfigJson = "{\"support_revocation\":true}";
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredDefResult =
				issuerCreateAndStoreCredentialDef(issuerWallet, issuerDid, schemaJson, credDefTag, null, credDefConfigJson).get();
		String credDefId = createCredDefResult.getCredDefId();
		String credDefJson = createCredDefResult.getCredDefJson();

		//6. Issuer create Revocation Registry
		String revRegDefConfig = new JSONObject("{\"issuance_type\":\"ISSUANCE_ON_DEMAND\",\"max_cred_num\":5}").toString();
		String tailsWriterConfig = new JSONObject(String.format("{\"base_dir\":\"%s\", \"uri_pattern\":\"\"}", getIndyHomePath("tails")).replace('\\', '/')).toString();
		BlobStorageWriter tailsWriter = BlobStorageWriter.openWriter("default", tailsWriterConfig).get();

		String revRegDefTag = "Tag2";
		AnoncredsResults.IssuerCreateAndStoreRevocRegResult createRevRegResult =
				issuerCreateAndStoreRevocReg(issuerWallet, issuerDid, null, revRegDefTag, credDefId, revRegDefConfig, tailsWriter).get();
		String revRegId = createRevRegResult.getRevRegId();
		String revRegDefJson = createRevRegResult.getRevRegDefJson();

		//7. Prover create Master Secret
		String masterSecretId = proverCreateMasterSecret(proverWallet, null).get();

		//8. Issuer Creates Credential Offer
		String credOffer = issuerCreateCredentialOffer(issuerWallet, credDefId).get();

		//9. Prover Creates Credential Request
		AnoncredsResults.ProverCreateCredentialRequestResult createCredReqResult =
				proverCreateCredentialReq(proverWallet, proverDid, credOffer, credDefJson, masterSecretId).get();
		String credReqJson = createCredReqResult.getCredentialRequestJson();
		String credReqMetadataJson = createCredReqResult.getCredentialRequestMetadataJson();

		//10. Issuer open Tails Reader
		BlobStorageReader blobStorageReaderCfg = BlobStorageReader.openReader("default", tailsWriterConfig).get();
		int blobStorageReaderHandle = blobStorageReaderCfg.getBlobStorageReaderHandle();

		//11. Issuer create Credential
		String credValuesJson = new JSONObject("{\n" +
				"        \"sex\": {\"raw\": \"male\", \"encoded\": \"594465709955896723921094925839488742869205008160769251991705001\"},\n" +
				"        \"name\": {\"raw\": \"Alex\", \"encoded\": \"1139481716457488690172217916278103335\"},\n" +
				"        \"height\": {\"raw\": \"175\", \"encoded\": \"175\"},\n" +
				"        \"age\": {\"raw\": \"28\", \"encoded\": \"28\"}\n" +
				"    }").toString();

		AnoncredsResults.IssuerCreateCredentialResult createCredentialResult =
				Anoncreds.issuerCreateCredential(issuerWallet, credOffer, credReqJson, credValuesJson, revRegId, blobStorageReaderHandle).get();
		String credentialJson = createCredentialResult.getCredentialJson();
		String revRegDeltaJson = createCredentialResult.getRevocRegDeltaJson();
		String credRevId = createCredentialResult.getRevocId();

		//12. Prover Stores Credential
		Anoncreds.proverStoreCredential(proverWallet, null, credReqJson, credReqMetadataJson, credentialJson, credDefJson, revRegDefJson).get();

		//13. Prover Gets Credentials for Proof Request
		String proofRequestJson = new JSONObject("{\n" +
				"                   \"nonce\":\"123432421212\",\n" +
				"                   \"name\":\"proof_req_1\",\n" +
				"                   \"version\":\"0.1\", " +
				"                   \"requested_attributes\":{" +
				"                          \"attr1_referent\":{\"name\":\"name\"}" +
				"                    },\n" +
				"                    \"requested_predicates\":{" +
				"                          \"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}" +
				"                    }" +
				"               }").toString();

		String credentialsForProofJson = proverGetCredentialsForProofReq(proverWallet, proofRequestJson).get();

		JSONObject credentials = new JSONObject(credentialsForProofJson);
		JSONArray credentialsForAttr1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		JSONArray credentialsForPredicate1 = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");

		String credIdForAttr1 = credentialsForAttr1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		String credIdForPred1 = credentialsForPredicate1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//14. Prover create RevocationState
		int timestamp = 100;
		String revStateJson = Anoncreds.createRevocationState(blobStorageReaderHandle, revRegDefJson, revRegDeltaJson, timestamp, credRevId).get();

		//15. Prover Creates Proof
		String requestedCredentialsJson = new JSONObject(String.format("{" +
				"\"self_attested_attributes\":{}," +
				"\"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true, \"timestamp\":%d }}," +
				"\"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\", \"timestamp\":%d}}" +
				"}", credIdForAttr1, timestamp, credIdForPred1, timestamp)).toString();

		String schemas = new JSONObject(String.format("{\"%s\":%s}", schemaId, schemaJson)).toString();
		String credentialDefs = new JSONObject(String.format("{\"%s\":%s}", credDefId, credDefJson)).toString();
		String revStates = new JSONObject(String.format("{\"%s\": { \"%s\":%s }}", revRegId, timestamp, revStateJson)).toString();

		String proofJson = Anoncreds.proverCreateProof(proverWallet, proofRequestJson, requestedCredentialsJson, masterSecretId, schemas,
				credentialDefs, revStates).get();
		JSONObject proof = new JSONObject(proofJson);

		//16. Verifier verify Proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		String revRegDefs = new JSONObject(String.format("{\"%s\":%s}", revRegId, revRegDefJson)).toString();
		String revRegs = new JSONObject(String.format("{\"%s\": { \"%s\":%s }}", revRegId, timestamp, revRegDeltaJson)).toString();

		boolean valid = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemas, credentialDefs, revRegDefs, revRegs).get();
		assertTrue(valid);

		//17. Close and Delete issuer wallet
		issuerWallet.closeWallet().get();
		Wallet.deleteWallet(issuerWalletName, null).get();

		//18. Close and Delete prover wallet
		proverWallet.closeWallet().get();
		Wallet.deleteWallet(proverWalletName, null).get();

		//19. Close pool
		pool.closePoolLedger().get();

		//20. Delete Pool ledger config
		Pool.deletePoolLedgerConfig(poolName).get();

		System.out.println("Anoncreds Revocation sample -> completed");
	}
}
