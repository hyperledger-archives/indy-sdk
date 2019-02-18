package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletItemNotFoundException;
import org.junit.Test;
import org.json.JSONObject;
import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class ProverCreateProofTest extends AnoncredsIntegrationTest {

	private String requestedCredentialsJson = String.format("{" +
			"\"self_attested_attributes\":{}," +
			"\"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true}}," +
			"\"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}}" +
			"}", credentialId1, credentialId1);

	@Test
	public void testProverCreateProofWorks() throws Exception {

		String schemasJson = new JSONObject().put(gvtSchemaId, new JSONObject(gvtSchema)).toString();
		String credentialDefsJson = new JSONObject().put(issuer1gvtCredDefId, new JSONObject(issuer1gvtCredDef)).toString();
		String revocStatesJson = new JSONObject().toString();

		String proofJson = Anoncreds.proverCreateProof(wallet, proofRequest, new JSONObject(requestedCredentialsJson).toString(),
				masterSecretId, schemasJson, credentialDefsJson, revocStatesJson).get();
		assertNotNull(proofJson);
	}

	@Test
	public void testProverCreateProofWorksForUsingNotSatisfyCredential() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String requestedCredentialsJson = String.format("{\"self_attested_attributes\":{},\n" +
				"                                    \"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true}},\n" +
				"                                    \"requested_predicates\":{}\n" +
				"                                   }", credentialId2);

		String schemasJson = new JSONObject().put(xyzSchemaId, xyzSchema).toString();
		String credentialDefsJson = new JSONObject().put(issuer1xyzCredDef, issuer1xyzCredDef).toString();
		String revocStatesJson = new JSONObject().toString();

		Anoncreds.proverCreateProof(wallet, proofRequest, new JSONObject(requestedCredentialsJson).toString(),
				masterSecretId, schemasJson, credentialDefsJson, revocStatesJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidMasterSecret() throws Exception {

		String schemasJson = new JSONObject().put(gvtSchemaId, new JSONObject(gvtSchema)).toString();
		String credentialDefsJson = new JSONObject().put(issuer1gvtCredDefId, new JSONObject(issuer1gvtCredDef)).toString();
		String revocStatesJson = new JSONObject().toString();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Anoncreds.proverCreateProof(wallet, proofRequest, new JSONObject(requestedCredentialsJson).toString(),
				"wrong_master_secret", schemasJson, credentialDefsJson, revocStatesJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidSchemas() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schemasJson = new JSONObject().toString();
		String credentialDefsJson = new JSONObject().put(issuer1gvtCredDefId, issuer1gvtCredDef).toString();
		String revocStatesJson = new JSONObject().toString();

		Anoncreds.proverCreateProof(wallet, proofRequest, new JSONObject(requestedCredentialsJson).toString(),
				masterSecretId, schemasJson, credentialDefsJson, revocStatesJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidRequestedCredentialsJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schemasJson = new JSONObject().put(gvtSchemaId, gvtSchema).toString();
		String credentialDefsJson = new JSONObject().put(issuer1gvtCredDefId, issuer1gvtCredDef).toString();
		String revocStatesJson = new JSONObject().toString();
		String requestedCredentialsJson = new JSONObject().put("self_attested_attributes", new JSONObject()).
														   put("requested_predicates", new JSONObject()).toString();


		/*String requestedCredentialsJson = "{\"self_attested_attributes\":{},\n" +
				"                      \"requested_predicates\":{}\n" +
				"                    }";*/

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, masterSecretId, schemasJson, credentialDefsJson, revocStatesJson).get();
	}
}
