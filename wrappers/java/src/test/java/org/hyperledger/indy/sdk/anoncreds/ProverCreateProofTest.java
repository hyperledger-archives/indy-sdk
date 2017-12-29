package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class ProverCreateProofTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverCreateProofWorks() throws Exception {

		initCommonWallet();

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONObject claimForAttribute = claims.getJSONObject("attrs").getJSONArray("attr1_referent").getJSONObject(0);

		String claimUuid = claimForAttribute.getString("referent");

		String requestedClaimsJson = String.format(requestedClaimsJsonTemplate, claimUuid, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";

		String proofJson = Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson,
				masterSecretName, claimDefsJson, revocRegsJson).get();
		assertNotNull(proofJson);
	}

	@Test
	public void testProverCreateProofWorksForUsingNotSatisfyClaim() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		initCommonWallet();

		String claimsJson = Anoncreds.proverGetClaims(wallet, "{}").get();

		JSONArray claims = new JSONArray(claimsJson);

		String claimUuid = claims.getJSONObject(0).getString("referent");

		String proofRequest = "{\n" +
		"                           \"nonce\":\"123432421212\",\n" +
		"                           \"name\":\"proof_req_1\",\n" +
		"                           \"version\":\"0.1\", " +
		"                           \"requested_attrs\":{" +
		"                               \"attr1_referent\":{\"name\":\"some_attr\"}" +
		"                           },\n" +
		"                           \"requested_predicates\":{}" +
		"                      }";

		String requestedClaimsJson = String.format("{\"self_attested_attributes\":{},\n" +
				"                                    \"requested_attrs\":{\"attr1_referent\":[\"%s\", true]},\n" +
				"                                    \"requested_predicates\":{}\n" +
				"                                   }", claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson,
				masterSecretName, claimDefsJson, revocRegsJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidMasterSecret() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		initCommonWallet();

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONObject claimForAttribute = claims.getJSONObject("attrs").getJSONArray("attr1_referent").getJSONObject(0);

		String claimUuid = claimForAttribute.getString("referent");

		String requestedClaimsJson = String.format(requestedClaimsJsonTemplate, claimUuid, claimUuid);

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson, "wrong_master_secret", claimDefsJson, revocRegsJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidSchemas() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		initCommonWallet();

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONObject claimForAttribute = claims.getJSONObject("attrs").getJSONArray("attr1_referent").getJSONObject(0);

		String claimUuid = claimForAttribute.getString("referent");

		String requestedClaimsJson = String.format(requestedClaimsJsonTemplate, claimUuid, claimUuid);
		String schemasJson = "{}";
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson, masterSecretName, claimDefsJson, revocRegsJson).get();
	}

	@Test
	public void testProverCreateProofWorksForInvalidRequestedClaimsJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		initCommonWallet();

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONObject claimForAttribute = claims.getJSONObject("attrs").getJSONArray("attr1_referent").getJSONObject(0);

		String claimUuid = claimForAttribute.getString("referent");

		String requestedClaimsJson = "{\"self_attested_attributes\":{},\n" +
				"                      \"requested_predicates\":{}\n" +
				"                    }";

		String schemasJson = String.format("{\"%s\":%s}", claimUuid, gvtSchemaJson);
		String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDef);
		String revocRegsJson = "{}";

		Anoncreds.proverCreateProof(wallet, proofRequest, requestedClaimsJson, schemasJson, masterSecretName, claimDefsJson, revocRegsJson).get();
	}
}
