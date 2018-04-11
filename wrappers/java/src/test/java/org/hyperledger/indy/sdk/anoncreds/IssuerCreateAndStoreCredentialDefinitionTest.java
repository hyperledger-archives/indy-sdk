package org.hyperledger.indy.sdk.anoncreds;

import static org.hamcrest.CoreMatchers.*;

import org.hyperledger.indy.sdk.InvalidStructureException;

import static org.junit.Assert.assertNotNull;

import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class IssuerCreateAndStoreCredentialDefinitionTest extends AnoncredsIntegrationTest {

	private Wallet wallet;
	private String walletName = "createAndStoreCredDefWallet";

	@Before
	public void createWallet() throws Exception {
		Wallet.createWallet("default", walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorks() throws Exception {
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredentialDefResult =
				Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, gvtSchema, "Works", null, defaultCredentialDefitionConfig).get();
		assertNotNull(createCredentialDefResult);
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

		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, schema, "InvalidSchema", null, defaultCredentialDefitionConfig).get();
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

		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, schema, "EmptyKeys", null, defaultCredentialDefitionConfig).get();
	}

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorksForCorrectCryptoType() throws Exception {
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredentialDefResult =
				Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, gvtSchema, "CorrectCryptoType", "CL", defaultCredentialDefitionConfig).get();
		assertNotNull(createCredentialDefResult);
	}

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorksForInvalidCryptoType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, gvtSchema, "InvalidCryptoType", "type", defaultCredentialDefitionConfig).get();
	}

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorksForDuplicate() throws Exception {
		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, gvtSchema, "Duplicate", null, defaultCredentialDefitionConfig).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(CredDefAlreadyExistsException.class));

		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, gvtSchema, "Duplicate", null, defaultCredentialDefitionConfig).get();
	}
}
