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

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
				"              \"requested_predicates\":{}\n" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_uuid");
		assertEquals(1, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForNotFoundAttribute() throws Exception {

		initCommonWallet();

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"attribute\"}},\n" +
				"              \"requested_predicates\":{}\n" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_uuid");
		assertEquals(0, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForSatisfyPredicate() throws Exception {

		initCommonWallet();

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{},\n" +
				"              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_uuid");
		assertEquals(1, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForNotSatisfyPredicate() throws Exception {

		initCommonWallet();

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{},\n" +
				"              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":58}}\n" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_uuid");
		assertEquals(0, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForMultiplyAttributesAndPredicates() throws Exception {

		initCommonWallet();

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"               \"name\":\"proof_req_1\",\n" +
				"               \"version\":\"0.1\",\n" +
				"               \"requested_attrs\":{\n" +
				"                     \"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"},\n" +
				"                     \"attr2_uuid\":{\"schema_seq_no\":1, \"name\":\"sex\"}\n" +
				"               },\n" +
				"               \"requested_predicates\":{\n" +
				"                     \"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18},\n" +
				"                     \"predicate2_uuid\":{\"attr_name\":\"height\",\"p_type\":\"GE\",\"value\":160}\n" +
				"               }}";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_uuid");
		assertEquals(1, claimsForAttribute1.length());

		JSONArray claimsForAttribute2 = claims.getJSONObject("attrs").getJSONArray("attr2_uuid");
		assertEquals(1, claimsForAttribute2.length());

		JSONArray claimsForPredicate1 = claims.getJSONObject("predicates").getJSONArray("predicate1_uuid");
		assertEquals(1, claimsForPredicate1.length());

		JSONArray claimsForPredicate2 = claims.getJSONObject("predicates").getJSONArray("predicate2_uuid");
		assertEquals(1, claimsForPredicate2.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForEmptyRequest() throws Exception {

		initCommonWallet();

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{},\n" +
				"              \"requested_predicates\":{}\n" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		assertEquals(0, claims.getJSONObject("attrs").length());
		assertEquals(0, claims.getJSONObject("predicates").length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeWithOtherSchema() throws Exception {

		initCommonWallet();

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":2, \"name\":\"name\"}},\n" +
				"              \"requested_predicates\":{}\n" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_uuid");
		assertEquals(0, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificIssuer() throws Exception {

		initCommonWallet();

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{\"attr1_uuid\":{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"name\":\"name\"}},\n" +
				"              \"requested_predicates\":{}\n" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForAttribute1 = claims.getJSONObject("attrs").getJSONArray("attr1_uuid");
		assertEquals(1, claimsForAttribute1.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForSatisfyPredicateByIssuerAndSchema() throws Exception {

		initCommonWallet();

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{},\n" +
				"              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}}\n" +
				"             }";

		String claimsJson = Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();

		JSONObject claims = new JSONObject(claimsJson);

		JSONArray claimsForPredicate = claims.getJSONObject("predicates").getJSONArray("predicate1_uuid");
		assertEquals(1, claimsForPredicate.length());
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForInvalidProofRequest() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_predicates\":{}\n" +
				"             }";

		Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();
	}

	@Test
	public void testProverGetClaimsForProofRequestWorksForInvalidPredicateType() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String proofRequest = "{\"nonce\":\"123432421212\",\n" +
				"              \"name\":\"proof_req_1\",\n" +
				"              \"version\":\"0.1\",\n" +
				"              \"requested_attrs\":{},\n" +
				"              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"LE\",\"value\":18}}\n" +
				"             }";

		Anoncreds.proverGetClaimsForProofReq(wallet, proofRequest).get();
	}
}
