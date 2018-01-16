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

		initCommonWallet();

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{\"name\":\"name\"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeInUpperCase() throws Exception {

		initCommonWallet();

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{\"name\":\"NAME\"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeContainsSpaces() throws Exception {

		initCommonWallet();

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{\"name\":\" name \"}" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForNotFoundAttribute() throws Exception {

		initCommonWallet();

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{\"name\":\"attribute\"}" +
				"              }," +
				"              \"requested_predicates\":{}" +
				"         }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(0, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicate() throws Exception {

		initCommonWallet();

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

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateAttrInUpperCase() throws Exception {

		initCommonWallet();

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

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateAttrContainsSpaces() throws Exception {

		initCommonWallet();

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

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForNotSatisfiedPredicate() throws Exception {

		initCommonWallet();

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

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(0, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForMultiplyAttributesAndPredicates() throws Exception {

		initCommonWallet();

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

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

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

		initCommonWallet();

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{}" +
				"         }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		assertEquals(0, claims.getJSONObject("attrs").length());
		assertEquals(0, claims.getJSONObject("predicates").length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificIssuer() throws Exception {

		initCommonWallet();

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

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(1, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificSchema() throws Exception {

		initCommonWallet();

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_key\":%s}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaKey);

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeByPartOfSchema() throws Exception {

		initCommonWallet();

		String proofRequest = "{" +
				"               \"nonce\":\"123432421212\"," +
				"               \"name\":\"proof_req_1\"," +
				"               \"version\":\"0.1\"," +
				"               \"requested_attrs\":{" +
				"                    \"attr1_referent\":{" +
				"                        \"name\":\"name\"," +
				"                        \"restrictions\":[{\"schema_key\":{\"name\":\"gvt\"}}]" +
				"                    }" +
				"                }," +
				"               \"requested_predicates\":{}" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeByMultipleSchemas() throws Exception {

		initCommonWallet();

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_key\":%s}, {\"schema_key\":%s}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaKey, xyzSchemaKey);

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificSchemaIssuerPair() throws Exception {

		initCommonWallet();

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_key\":%s, \"issuer_did\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaKey, issuerDid);

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(1, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificSchemaOrSpecificIssuer() throws Exception {

		initCommonWallet();

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{" +
				"                   \"attr1_referent\":{" +
				"                       \"name\":\"name\"," +
				"                       \"restrictions\":[{\"schema_key\":%s}, {\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"               }," +
				"              \"requested_predicates\":{}" +
				"          }", gvtSchemaKey, issuerDid);

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_referent");
		assertEquals(2, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateBySpecificIssuer() throws Exception {

		initCommonWallet();

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

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(1, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateBySpecificSchema() throws Exception {

		initCommonWallet();

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"schema_key\":%s}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaKey);

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateByMultipleSchemas() throws Exception {

		initCommonWallet();

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"schema_key\":%s}, {\"schema_key\":%s}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaKey, xyzSchemaKey);

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateBySpecificIssuerSchemaPair() throws Exception {

		initCommonWallet();

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"schema_key\":%s, \"issuer_did\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaKey, issuerDid);

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(1, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForPredicateBySpecificIssuerOrSpecificSchema() throws Exception {

		initCommonWallet();

		String proofRequest = String.format("{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_attrs\":{}," +
				"              \"requested_predicates\":{" +
				"                   \"predicate1_referent\":{" +
				"                       \"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18," +
				"                       \"restrictions\":[{\"schema_key\":%s}, {\"issuer_did\":\"%s\"}]" +
				"                   }" +
				"              }" +
				"          }", gvtSchemaKey, issuerDid);

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_referent");
		assertEquals(2, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForInvalidProofRequest() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String proofRequest = "{" +
				"              \"nonce\":\"123432421212\"," +
				"              \"name\":\"proof_req_1\"," +
				"              \"version\":\"0.1\"," +
				"              \"requested_predicates\":{}" +
				"          }";

		Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForInvalidPredicateType() throws Exception {

		initCommonWallet();

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

		Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();
	}
}
