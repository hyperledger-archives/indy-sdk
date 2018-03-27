package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.json.JSONArray;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

public class ProverGetCredentialsTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverGetCredentialsWorksForEmptyFilter() throws Exception {

		String credentials = Anoncreds.proverGetCredentials(wallet, "{}").get();

		JSONArray credentialsArray = new JSONArray(credentials);

		assertEquals(3, credentialsArray.length());
	}

	@Test
	public void testProverGetCredentialsWorksForFilterByIssuer() throws Exception {

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String credentials = Anoncreds.proverGetCredentials(wallet, filter).get();

		JSONArray credentialsArray = new JSONArray(credentials);

		assertEquals(2, credentialsArray.length());
	}

	@Test
	public void testProverGetCredentialsWorksForFilterBySchema() throws Exception {

		String filter = String.format("{\"schema_id\":\"%s\"}", gvtSchemaId);

		String credentials = Anoncreds.proverGetCredentials(wallet, filter).get();

		JSONArray credentialsArray = new JSONArray(credentials);

		assertEquals(2, credentialsArray.length());
	}

	@Test
	public void testProverGetCredentialsWorksForFilterBySchemaName() throws Exception {

		String filter = "{\"schema_name\": \"gvt\"}";

		String credentials = Anoncreds.proverGetCredentials(wallet, filter).get();

		JSONArray credentialsArray = new JSONArray(credentials);

		assertEquals(2, credentialsArray.length());
	}

	@Test
	public void testProverGetCredentialsWorksForFilterByCredDefId() throws Exception {

		String filter = String.format("{\"cred_def_id\":\"%s\"}", issuer1gvtCredDefId);

		String credentials = Anoncreds.proverGetCredentials(wallet, filter).get();

		JSONArray credentialsArray = new JSONArray(credentials);

		assertEquals(1, credentialsArray.length());
	}

	@Test
	public void testProverGetCredentialsWorksForEmptyResult() throws Exception {

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid + "a");

		String credentials = Anoncreds.proverGetCredentials(wallet, filter).get();

		JSONArray credentialsArray = new JSONArray(credentials);

		assertEquals(0, credentialsArray.length());
	}

	@Test
	public void testProverGetCredentialsWorksForInvalidFilterJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String filter = "{\"schema_name\":gvt}";

		Anoncreds.proverGetCredentials(wallet, filter).get();
	}
}
