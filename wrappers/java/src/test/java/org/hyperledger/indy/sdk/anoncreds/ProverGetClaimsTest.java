package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.json.JSONArray;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

public class ProverGetClaimsTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverGetClaimsWorksForEmptyFilter() throws Exception {

		String claims = Anoncreds.proverGetClaims(wallet, "{}").get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(3, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForFilterByIssuer() throws Exception {

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(2, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForFilterBySchema() throws Exception {

		String filter = String.format("{\"schema_id\":\"%s\"}", gvtSchemaId);

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(2, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForFilterBySchemaName() throws Exception {

		String filter = "{\"schema_name\": \"gvt\"}";

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(2, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForFilterByCredDefId() throws Exception {

		String filter = String.format("{\"cred_def_id\":\"%s\"}", issuer1gvtClaimDefId);

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(1, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForEmptyResult() throws Exception {

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid + "a");

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(0, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForInvalidFilterJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String filter = "{\"schema_name\":gvt}";

		Anoncreds.proverGetClaims(wallet, filter).get();
	}
}
