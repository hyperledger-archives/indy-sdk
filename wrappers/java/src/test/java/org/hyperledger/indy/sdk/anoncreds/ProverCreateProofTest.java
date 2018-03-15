package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class ProverCreateProofTest extends AnoncredsIntegrationTest {

	private String schemasJson = String.format("{\"%s\":%s}", credentialId1, gvtSchemaJson);
	private String credentialDefsJson = String.format("{\"%s\":%s}", credentialId1, issuer1gvtCredDef);
	private String revocInfosJson = "{}";
	private String requestedCredentialsJson = String.format(requestedCredentialsJsonTemplate, credentialId1, credentialId1);

	@Test
	public void testProverCreateProofWorks() throws Exception {
		String proofJson = Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, schemasJson,
				masterSecretName, credentialDefsJson, revocInfosJson).get();
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

		String schemasJson = String.format("{\"%s\":%s}", credentialId2, xyzSchemaJson);
		String credentialDefsJson = String.format("{\"%s\":%s}", credentialId2, issuer1xyzCredDef);
		String revocInfosJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, schemasJson, masterSecretName, credentialDefsJson, revocInfosJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidMasterSecret() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, schemasJson, "wrong_master_secret", credentialDefsJson, revocInfosJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidSchemas() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schemasJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, schemasJson, masterSecretName, credentialDefsJson, revocInfosJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidRequestedCredentialsJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String requestedCredentialsJson = "{\"self_attested_attributes\":{},\n" +
				"                      \"requested_predicates\":{}\n" +
				"                    }";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, schemasJson, masterSecretName, credentialDefsJson, revocInfosJson).get();
	}
}
