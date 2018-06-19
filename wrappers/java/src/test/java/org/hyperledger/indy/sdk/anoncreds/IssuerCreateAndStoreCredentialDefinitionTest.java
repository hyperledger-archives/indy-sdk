package org.hyperledger.indy.sdk.anoncreds;

import static org.hamcrest.CoreMatchers.*;

import org.hyperledger.indy.sdk.InvalidStructureException;


import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class IssuerCreateAndStoreCredentialDefinitionTest extends AnoncredsIntegrationTest {

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorks() throws Exception {
	}

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorksForInvalidSchemaJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schema = "{" +
				"           \"name\":\"name\"," +
				"           \"version\":1.0," +
				"           \"attr_names\":[\"name\"]" +
				"        }";

		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, schema, "InvalidSchema", null, defaultCredentialDefinitionConfig).get();
	}

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorksForEmptyKeys() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schema = "{\n" +
				"           \"id\":\"1\",\n" +
				"           \"name\":\"gvt\",\n" +
				"           \"version\":\"1.0\",\n" +
				"           \"attr_names\":[]\n" +
				"       }";

		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, schema, "EmptyKeys", null, defaultCredentialDefinitionConfig).get();
	}

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorksForDuplicate() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(CredDefAlreadyExistsException.class));

		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, gvtSchema, tag, null, defaultCredentialDefinitionConfig).get();
	}
}
