package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

public class ProverGetClaimsForProofRequestTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttribute() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{\"name\":\"name\"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeInUpperCase() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{\"name\":\"NAME\"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeContainsSpaces() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{\"name\":\" name \"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForNotFoundAttribute() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{\"name\":\"attribute\"}" +
				"              }," +
				"              \"requested_predicates\":{}" +
				"         }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(0, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicate() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18" +
				"                   }" +
				"              }" +
				"          }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateAttrInUpperCase() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"AGE\",\"p_type\":\">=\",\"value\":18" +
				"                   }" +
				"              }" +
				"          }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateAttrContainsSpaces() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\" age \",\"p_type\":\">=\",\"value\":18" +
				"                   }" +
				"              }" +
				"          }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForNotSatisfiedPredicate() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":58" +
				"                   }" +
				"               }" +
				"         }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(0, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForMultiplyAttributesAndPredicates() throws Exception {

		String proofRequest = "{" +
				"               \"nonce\":\"123432421212\"," +
				"               \"name\":\"proof_req_1\"," +
				"               \"version\":\"0.1\"," +
				"               \"requested_attrs\":{" +
				"                     \"attr1_referent\":{ \"name\":\"name\"}," +
				"                     \"attr2_referent\":{\"name\":\"sex\"}" +
				"               }," +
				"               \"requested_predicates\":{" +
				"                     \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}," +
				"                     \"predicate2_referent\":{\"attr_name\":\"height\",\"p_type\":\">=\",\"value\":160}" +
				"               }" +
				"            }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());

		JSONArray claimsForAttribute2 = claims.getJSONObject("attrs").getJSONArray("attr2_referent");
		assertEquals(2, claimsForAttribute2.length());

		JSONArray claimsForPredicate1 = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate1.length());

		JSONArray claimsForPredicate2 = claims.getJSONObject("predicates").getJSONArray("predicate2_referent");
		assertEquals(2, claimsForPredicate2.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForEmptyRequest() throws Exception {

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{}" +
				"         }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		assertEquals(0, claims.getJSONObject("attrs").length());
		assertEquals(0, claims.getJSONObject("predicates").length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificIssuer() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", issuerDid);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(1, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySchemaId() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaId);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySchemaName() throws Exception {

		String proofRequest = "{" +
				"               \"nonce\":\"123432421212\"," +
				"               \"name\":\"proof_req_1\"," +
				"               \"version\":\"0.1\"," +
				"               \"requested_attrs\":{" +
				"                    \"attr1_referent\":{" +
				"                        \"name\":\"name\"," +
				"                        \"restrictions\":[{\"schema_name\":\"gvt\"}]" +
				"                    }" +
				"                }," +
				"               \"requested_predicates\":{}" +
				"             }";

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeByMultipleSchemas() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}, {\"schema_id\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaId, xyzSchemaId);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeByCredDefId() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"cred_def_id\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", issuer1gvtCredDefId);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(1, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificSchemaOrSpecificIssuer() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}, {\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaId, issuerDid);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateBySpecificIssuer() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", issuerDid);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(1, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateBySchemaId() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaId);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateByMultipleSchemas() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}, {\"schema_id\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaId, xyzSchemaId);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateBySpecificCredDefId() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"cred_def_id\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", issuer1gvtCredDefId);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(1, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateBySpecificIssuerOrSpecificSchema() throws Exception {

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"schema_id\":\"%s\"}, {\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaId, issuerDid);

		String claimsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForInvalidProofRequest() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_predicates\":{}" +
				"          }";

		Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForInvalidPredicateType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                    \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\"LE\",\"value\":18}" +
				"              }" +
				"          }";

		Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();
	}
}
