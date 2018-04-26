package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverCreateMasterSecretTest extends AnoncredsIntegrationTest {

	private Wallet wallet;
	private String walletName = "createMasterSecretWallet";

	@Before
	public void createWallet() throws Exception {
		Wallet.createWallet("default", walletName, "default", null, null).get();
		this.wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void deleteWallet() throws Exception {
		this.wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testProverCreateMasterSecretWorks() throws Exception {

		Anoncreds.proverCreateMasterSecret(wallet, masterSecretId).get();
	}

	@Test
	public void testProverCreateMasterSecretWorksForDuplicate() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(DuplicateMasterSecretNameException.class));

		String masterSecretId = "master_secret_name_duplicate";
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretId).get();
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretId).get();
	}

	@Test
	public void testProverCreateMasterSecretWorksForEmptyName() throws Exception {
		Anoncreds.proverCreateMasterSecret(wallet, null).get();
	}
}
