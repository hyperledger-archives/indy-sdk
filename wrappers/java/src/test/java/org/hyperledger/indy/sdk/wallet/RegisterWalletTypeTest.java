package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.json.JSONArray;
import org.junit.Ignore;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.*;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

public class RegisterWalletTypeTest extends IndyIntegrationTest {

	private String type = "inmem";

	@Test
	@Ignore //The wallet is already registered by the base class!
	public void testRegisterWalletTypeWorks() throws Exception {
		Wallet.registerWalletType(type, new InMemWalletType()).get();
	}

	@Test
	public void testRegisterWalletTypeDoesNotWorkForTwiceWithSameName() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(DuplicateWalletTypeException.class));

		Wallet.registerWalletType(type, new InMemWalletType()).get();
	}

	@Rule
	public Timeout globalTimeout = new Timeout(2, TimeUnit.MINUTES);

	@Test
	public void customWalletWorkoutTest() throws Exception {

		StorageUtils.cleanupStorage();

		String walletName = "inmemWorkoutWallet";

		// 1. Creates and opens wallet
		Wallet.createWallet(POOL, walletName, type, null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		// 2. Issuer creates Schema
		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(DID, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES).get();
		String gvtSchemaJson = createSchemaResult.getSchemaJson();

		// 3. Issuer creates Credential Definition
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredentialDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(wallet, DID, gvtSchemaJson, TAG, null, DEFAULT_CRED_DEF_CONFIG).get();
		String сredentialDefId = createCredentialDefResult.getCredDefId();
		String сredentialDef = createCredentialDefResult.getCredDefJson();

		// 4. Issuer creates Credential Offer
		String сredentialOffer = Anoncreds.issuerCreateCredentialOffer(wallet, сredentialDefId, DID, DID_MY1).get();

		// 5. Issuer stores Credential Offer
		Anoncreds.proverStoreCredentialOffer(wallet, сredentialOffer).get();

		// 6. Issuer creates Master Secret
		String masterSecretName = "master_secret_name";
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		// 7. Prover creates Credential Request
		String credentialRequest = Anoncreds.proverCreateAndStoreCredentialReq(wallet, DID_MY1, сredentialOffer, сredentialDef, masterSecretName).get();

		// 8. Issuer creates Credential
		String gvtCredentialValues = "{\n" +
				"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";
		AnoncredsResults.IssuerCreateCredentialResult createCredentialResult = Anoncreds.issuerCreateCredentail(wallet, credentialRequest, gvtCredentialValues, null, - 1, - 1).get();
		String сredentialJson = createCredentialResult.getCredentialJson();

		// 9. Prover stores Credential
		Anoncreds.proverStoreCredential(wallet, "id1", сredentialJson, null).get();

		// 10. Prover gets Credential
		String сredentials = Anoncreds.proverGetCredentials(wallet, String.format("{\"issuer_did\":\"%s\"}", DID)).get();

		JSONArray сredentialsArray = new JSONArray(сredentials);

		assertEquals(1, сredentialsArray.length());
	}
}