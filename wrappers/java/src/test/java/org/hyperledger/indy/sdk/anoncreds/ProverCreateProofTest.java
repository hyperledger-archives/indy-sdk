package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class ProverCreateProofTest extends AnoncredsIntegrationTest {

	private String requestedCredentialsJson = String.format("{" +
			"\"self_attested_attributes\":{}," +
			"\"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true}}," +
			"\"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}}" +
			"}", credentialId1, credentialId1);

	@Test
	public void testProverCreateProofWorks() throws Exception {

		String schemasJson = String.format("{\"%s\":%s}", gvtSchemaId, gvtSchema);
		String credentialDefsJson = String.format("{\"%s\":%s}", issuer1gvtCredDefId, issuer1gvtCredDef);
		String revocStatesJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson,
				masterSecretId, schemasJson, credentialDefsJson, revocStatesJson).get();
		assertNotNull(proofJson);
	}

	@Test
	public void testProverCreateProofWorksForUsingNotSatisfyCredential() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String requestedCredentialsJson = String.format("{\"self_attested_attributes\":{},\n" +
				"                                    \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true}},\n" +
				"                                    \"requested_predicates\":{}\n" +
				"                                   }", credentialId2);

		String schemasJson = String.format("{\"%s\":%s}", xyzSchemaId, xyzSchema);
		String credentialDefsJson = String.format("{\"%s\":%s}", issuer1xyzCredDef, issuer1xyzCredDef);
		String revocStatesJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, masterSecretId, schemasJson, credentialDefsJson, revocStatesJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidMasterSecret() throws Exception {

		String schemasJson = String.format("{\"%s\":%s}", gvtSchemaId, gvtSchema);
		String credentialDefsJson = String.format("{\"%s\":%s}", issuer1gvtCredDefId, issuer1gvtCredDef);
		String revocStatesJson = "{}";

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, "wrong_master_secret", schemasJson, credentialDefsJson, revocStatesJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidSchemas() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schemasJson = "{}";
		String credentialDefsJson = String.format("{\"%s\":%s}", issuer1gvtCredDefId, issuer1gvtCredDef);
		String revocStatesJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, masterSecretId, schemasJson, credentialDefsJson, revocStatesJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidRequestedCredentialsJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schemasJson = String.format("{\"%s\":%s}", gvtSchemaId, gvtSchema);
		String credentialDefsJson = String.format("{\"%s\":%s}", issuer1gvtCredDefId, issuer1gvtCredDef);
		String revocStatesJson = "{}";

		String requestedCredentialsJson = "{\"self_attested_attributes\":{},\n" +
				"                      \"requested_predicates\":{}\n" +
				"                    }";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, masterSecretId, schemasJson, credentialDefsJson, revocStatesJson).get();
	}
}
