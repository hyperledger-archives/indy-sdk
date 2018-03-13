package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class ProverCreateProofTest extends AnoncredsIntegrationTest {

	private String schemasJson = String.format("{\"%s\":%s}", claimId1, gvtSchemaJson);
	private String claimDefsJson = String.format("{\"%s\":%s}", claimId1, issuer1gvtClaimDef);
	private String revocInfosJson = "{}";
	private String requestedClaimsJson = String.format(requestedClaimsJsonTemplate, claimId1, claimId1);

	@Test
	public void testProverCreateProofWorks() throws Exception {
		String proofJson = Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson,
				masterSecretName, claimDefsJson, revocInfosJson).get();
		assertNotNull(proofJson);
	}

	@Test
	public void testProverCreateProofWorksForUsingNotSatisfyClaim() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String requestedClaimsJson = String.format("{\"self_attested_attributes\":{},\n" +
				"                                    \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true}},\n" +
				"                                    \"requested_predicates\":{}\n" +
				"                                   }", claimId2);

		String schemasJson = String.format("{\"%s\":%s}", claimId2, xyzSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimId2, issuer1xyzClaimDef);
		String revocInfosJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson, masterSecretName, claimDefsJson, revocInfosJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidMasterSecret() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson, "wrong_master_secret", claimDefsJson, revocInfosJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidSchemas() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schemasJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson, masterSecretName, claimDefsJson, revocInfosJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidRequestedClaimsJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String requestedClaimsJson = "{\"self_attested_attributes\":{},\n" +
				"                      \"requested_predicates\":{}\n" +
				"                    }";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson, masterSecretName, claimDefsJson, revocInfosJson).get();
	}
}
