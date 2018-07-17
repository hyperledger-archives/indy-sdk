package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

public class ProverGetCredentialsForProofRequestTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttribute() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{\"name\":\"name\"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttributeInUpperCase() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{\"name\":\"NAME\"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttributeContainsSpaces() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{\"name\":\" name \"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForNotFoundAttribute() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{\"name\":\"attribute\"}" +
				"              }," +
				"              \"requested_predicates\":{}" +
				"         }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(0, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForPredicate() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\"age\",\"p_type\":\">=\",\"p_value\":18" +
				"                   }" +
				"              }" +
				"          }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForPredicateAttrInUpperCase() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\"AGE\",\"p_type\":\">=\",\"p_value\":18" +
				"                   }" +
				"              }" +
				"          }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForPredicateAttrContainsSpaces() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\" age \",\"p_type\":\">=\",\"p_value\":18" +
				"                   }" +
				"              }" +
				"          }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForNotSatisfiedPredicate() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\"age\",\"p_type\":\">=\",\"p_value\":58" +
				"                   }" +
				"               }" +
				"         }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(0, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForMultiplyAttributesAndPredicates() throws Exception {

		String proofRequest = "{" +
				"               \"nonce\":\"123432421212\"," +
				"               \"name\":\"proof_req_1\"," +
				"               \"version\":\"0.1\"," +
				"               \"requested_attributes\":{" +
				"                     \"attr1_referent\":{ \"name\":\"name\"}," +
				"                     \"attr2_referent\":{\"name\":\"sex\"}" +
				"               }," +
				"               \"requested_predicates\":{" +
				"                     \"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}," +
				"                     \"predicate2_referent\":{\"name\":\"height\",\"p_type\":\">=\",\"p_value\":160}" +
				"               }" +
				"            }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, credentialsForAttribute1.length());

		JSONArray credentialsForAttribute2 = credentials.getJSONObject("attrs").getJSONArray("attr2_referent");
		assertEquals(2, credentialsForAttribute2.length());

		JSONArray credentialsForPredicate1 = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, credentialsForPredicate1.length());

		JSONArray credentialsForPredicate2 = credentials.getJSONObject("predicates").getJSONArray("predicate2_referent");
		assertEquals(2, credentialsForPredicate2.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForEmptyRequest() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{}" +
				"         }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		assertEquals(0, credentials.getJSONObject("attrs").length());
		assertEquals(0, credentials.getJSONObject("predicates").length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttributeBySpecificIssuer() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", issuerDid);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(1, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttributeBySchemaId() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaId);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttributeBySchemaName() throws Exception {

		String proofRequest = "{" +
				"               \"nonce\":\"123432421212\"," +
				"               \"name\":\"proof_req_1\"," +
				"               \"version\":\"0.1\"," +
				"               \"requested_attributes\":{" +
				"                    \"attr1_referent\":{" +
				"                        \"name\":\"name\"," +
				"                        \"restrictions\":[{\"schema_name\":\"gvt\"}]" +
				"                    }" +
				"                }," +
				"               \"requested_predicates\":{}" +
				"             }";

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttributeByMultipleSchemas() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}, {\"schema_id\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaId, xyzSchemaId);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttributeByCredDefId() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"cred_def_id\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", issuer1gvtCredDefId);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(1, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForRevealedAttributeBySpecificSchemaOrSpecificIssuer() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}, {\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaId, issuerDid);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForAttribute1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, credentialsForAttribute1.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForPredicateBySpecificIssuer() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\"age\",\"p_type\":\">=\",\"p_value\":18," +
				"                       \"restrictions\":[{\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", issuerDid);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(1, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForPredicateBySchemaId() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\"age\",\"p_type\":\">=\",\"p_value\":18," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaId);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForPredicateByMultipleSchemas() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\"age\",\"p_type\":\">=\",\"p_value\":18," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}, {\"schema_id\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaId, xyzSchemaId);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForPredicateBySpecificCredDefId() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\"age\",\"p_type\":\">=\",\"p_value\":18," +
				"                       \"restrictions\":[{\"cred_def_id\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", issuer1gvtCredDefId);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(1, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForPredicateBySpecificIssuerOrSpecificSchema() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"name\":\"age\",\"p_type\":\">=\",\"p_value\":18," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}, {\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaId, issuerDid);

		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();

		JSONObject credentials = new JSONObject(credentialsJson);

		JSONArray credentialsForPredicate = credentials.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, credentialsForPredicate.length());
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForInvalidProofRequest() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_predicates\":{}" +
				"          }";

		Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();
	}

	@Test
	public void testProverGetCredentialsForProofRequestWorksForInvalidPredicateType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attributes\":{}," +
				"              \"requested_predicates\":{" +
				"                    \"predicate1_referent\":{\"name\":\"age\",\"p_type\":\"LE\",\"p_value\":18}" +
				"              }" +
				"          }";

		Anoncreds.proverGetCredentialsForProofReq(wallet, new JSONObject(proofRequest).toString()).get();
	}
}
