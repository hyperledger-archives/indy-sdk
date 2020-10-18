package org.hyperledger.indy.sdk.anoncreds;

import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverCreateMasterSecretTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverCreateMasterSecretWorks() throws Exception {
	}

	@Test
	public void testProverCreateMasterSecretWorksForDuplicate() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(DuplicateMasterSecretNameException.class));

		Anoncreds.proverCreateMasterSecret(wallet, masterSecretId).get();
	}
}
