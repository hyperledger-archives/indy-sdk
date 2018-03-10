package org.hyperledger.indy.sdk.anoncreds;

import static org.hamcrest.CoreMatchers.*;

import org.hyperledger.indy.sdk.InvalidStructureException;

import static org.junit.Assert.assertNotNull;

import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class IssuerCreateAndStoreClaimDefinitionTest extends AnoncredsIntegrationTest {

	@Test
	public void testIssuerCreateAndStoreClaimDefWorks() throws Exception {
		AnoncredsResults.IssuerCreateAndStoreClaimDefResult createClaimDefResult =
				Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "Works", null, defaultCredentialDefConfig).get();
		assertNotNull(createClaimDefResult);
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForInvalidSchemaJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schema = "{" +
				"           \"name\":\"name\"," +
				"           \"version\":1.0," +
				"           \"attr_names\":[\"name\"]" +
				"        }";

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, "InvalidSchema", null, defaultCredentialDefConfig).get();
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForEmptyKeys() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schema = "{\n" +
				"           \"id\":\"1\",\n" +
				"           \"name\":\"gvt\",\n" +
				"           \"version\":\"1.0\",\n" +
				"           \"attr_names\":[]\n" +
				"       }";

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, "EmptyKeys", null, defaultCredentialDefConfig).get();
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForCorrectCryptoType() throws Exception {
		AnoncredsResults.IssuerCreateAndStoreClaimDefResult createClaimDefResult =
				Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "CorrectCryptoType", "CL", defaultCredentialDefConfig).get();
		assertNotNull(createClaimDefResult);
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForInvalidCryptoType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "InvalidCryptoType", "type", defaultCredentialDefConfig).get();
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForDuplicate() throws Exception {
		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "Duplicate", null, defaultCredentialDefConfig).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(ClaimDefAlreadyExistsException.class));

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "Duplicate", null, defaultCredentialDefConfig).get();
	}
}
