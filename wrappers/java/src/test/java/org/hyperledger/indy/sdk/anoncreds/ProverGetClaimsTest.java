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

		initCommonWallet();

		String claims = Anoncreds.proverGetClaims(wallet, "{}").get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(1, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForFilterByIssuer() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(1, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForFilterByIssuerAndSchema() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"issuer_did\":\"%s\", \"schema_seq_no\":%d}", issuerDid, 1);

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(1, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForEmptyResult() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"schema_seq_no\":%d}", 10);

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(0, claimsArray.length());
	}

	@Test
	public void testProverGetClaimsWorksForInvalidFilterJson() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String filter = String.format("{\"schema_seq_no\":\"%d\"}", 1);

		Anoncreds.proverGetClaims(wallet, filter).get();
	}
}
