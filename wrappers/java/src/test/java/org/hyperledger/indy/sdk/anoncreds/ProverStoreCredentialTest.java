package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.*;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverStoreCredentialTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverStoreCredentialWorks() throws Exception {
	}

	@Test
	public void testProverStoreCredentialWorksForInvalidCredentialJson() throws Exception {

		Anoncreds.proverCreateCredentialReq(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretId).get();

		String credentialJson = "{\"issuer1GvtCredential\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
				"            \"issuer_did\":1}";

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.proverStoreCredential(wallet, credentialId1, issuer1GvtCredReqMetadata, credentialJson, issuer1gvtCredDef, null).get();
	}
}
