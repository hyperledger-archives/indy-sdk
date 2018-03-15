package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class IssuerCreateCredentialTest extends AnoncredsIntegrationTest {

	@Test
	public void testIssuerCreateCredentialWorks() throws Exception {

		AnoncredsResults.IssuerCreateCredentialResult createCredentialResult =
				Anoncreds.issuerCreateCredentail(wallet, credentialRequest, gvtCredentialValuesJson, null, - 1, - 1).get();
		assertNotNull(createCredentialResult);
	}

	@Test
	public void testIssuerCreateCredentialWorksForCredentialValuesDoesNotCorrespondToCredentialRequest() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.issuerCreateCredentail(wallet, credentialRequest, xyzCredentialValuesJson, null, - 1, - 1).get();
	}

	@Test
	public void testIssuerCreateCredentialWorksForInvalidCredentialValues() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String credential = "{" +
				"        \"sex\":\"male\",\n" +
				"        \"age\":\"28\"" +
				"       }";

		Anoncreds.issuerCreateCredentail(wallet, credentialRequest, credential, null, - 1, - 1).get();
	}
}
