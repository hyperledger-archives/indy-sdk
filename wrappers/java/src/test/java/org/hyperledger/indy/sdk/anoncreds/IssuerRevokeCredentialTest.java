package org.hyperledger.indy.sdk.anoncreds;

import static org.hyperledger.indy.sdk.utils.EnvironmentUtils.getIndyHomePath;
import static org.junit.Assert.*;


import blob_storage.BlobStorage;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.*;


public class IssuerRevokeCredentialTest extends AnoncredsIntegrationTest {

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

		//3. Issuer create issuer1GvtCredential definition
		String revocationCredentialDefConfig = "{\"support_revocation\":true}";
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredentialDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, schemaJson, tag, null, revocationCredentialDefConfig).get();
		String credDefId = createCredentialDefResult.getCredDefId();
		String credDefJson = createCredentialDefResult.getCredDefJson();

		//4. Issuer create revocation registry
		String revRegConfig = "{\"issuance_type\":null,\"max_cred_num\":5}";
		AnoncredsResults.IssuerCreateAndStoreRevocRegResult createRevRegResult = Anoncreds.issuerCreateAndStoreRevocReg(wallet, issuerDid, null, tag, credDefId, revRegConfig, "default", tailsWriterConfig).get();
		String revRegId = createRevRegResult.getRevRegId();
		String revRegDef = createRevRegResult.getRevRegDefJson();

		//5. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretId).get();

		//6. Issuer create Credential Offer
		String credOfferJson = Anoncreds.issuerCreateCredentialOffer(wallet, credDefId).get();

		//7. Prover create Credential Request
		AnoncredsResults.ProverCreateCredentialRequestResult createCredReqResult =
				Anoncreds.proverCreateCredentialReq(wallet, proverDid, credOfferJson, credDefJson, masterSecretId).get();
		String credentialReqJson = createCredReqResult.getCredentialRequestJson();
		String credentialReqMetadataJson = createCredReqResult.getCredentialRequestMetadataJson();

		//8. Issuer open TailsReader
		JSONObject revRegDeg = new JSONObject(revRegDef);
		BlobStorage blobReader = BlobStorage.openReader("default",
				tailsWriterConfig,
				revRegDeg.getJSONObject("value").getString("tails_location"),
				revRegDeg.getJSONObject("value").getString("tails_hash")).get();
		int blobStorageReaderHandle = blobReader.getBlobStorageReaderHandle();

		//9. Issuer create Credential
		AnoncredsResults.IssuerCreateCredentialResult createCredentialResult =
				Anoncreds.issuerCreateCredential(wallet, credOfferJson, credentialReqJson, gvtCredentialValuesJson, revRegId, blobStorageReaderHandle).get();
		String credJson = createCredentialResult.getCredentialJson();
		String credRevocId = createCredentialResult.getRevocId();
		String revRegDelta = createCredentialResult.getRevocRegDeltaJson();

		//10. Prover create RevocationState
		int timestamp = 100;
		String revStateJson = Anoncreds.createRevocationState(blobStorageReaderHandle, revRegDef, revRegDelta, timestamp, credRevocId).get();

		//11. Prover store received Credential
		Anoncreds.proverStoreCredential(wallet, credentialId1, credentialReqJson, credentialReqMetadataJson, credJson, credDefJson, revRegDef).get();

		//12. Prover gets Credentials for Proof Request
		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();
		JSONObject credentials = new JSONObject(credentialsJson);
		JSONArray credentialsForAttr1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");

		String credentialUuid = credentialsForAttr1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//13. Prover create Proof
		String requestedCredentialsJson = String.format("{" +
				"\"self_attested_attributes\":{}," +
				"\"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true, \"timestamp\":%d }}," +
				"\"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\", \"timestamp\":%d}}" +
				"}", credentialUuid, timestamp, credentialUuid, timestamp);

		String schemasJson = String.format("{\"%s\":%s}", gvtSchemaId, schemaJson);
		String credentialDefsJson = String.format("{\"%s\":%s}", credDefId, credDefJson);
		String revStatesJson = String.format("{\"%s\": { \"%s\":%s }}", revRegId, timestamp, revStateJson);

		String proofJson = Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, masterSecretId, schemasJson,
				credentialDefsJson, revStatesJson).get();
		JSONObject proof = new JSONObject(proofJson);

		//14. Issuer revoke Credential
		revRegDelta = Anoncreds.issuerRevokeCredential(wallet, blobStorageReaderHandle, revRegId, credRevocId).get();

		//15. Verifier verify proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		String revRegDefsJson = String.format("{\"%s\":%s}", revRegId, revRegDef);
		String revRegs = String.format("{\"%s\": { \"%s\":%s }}", revRegId, timestamp, revRegDelta);

		boolean valid = Anoncreds.verifierVerifyProof(proofRequest, proofJson, schemasJson, credentialDefsJson, revRegDefsJson, revRegs).get();
		assertFalse(valid);

		// 17. Close and Delete Wallet
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}
}
