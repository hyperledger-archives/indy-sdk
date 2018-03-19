package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.InMemWalletType;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

public class ProverGetCredentialOffersTest extends AnoncredsIntegrationTest {

	@Test
	public void testsProverGetCredentialOffersWorksForEmptyFilter() throws Exception {

		String credentialOffers = Anoncreds.proverGetCredentialOffers(wallet, "{}").get();
		JSONArray credentialOffersArray = new JSONArray(credentialOffers);

		assertEquals(3, credentialOffersArray.length());
	}

	@Test
	public void testsProverGetCredentialOffersWorksForFilterByIssuer() throws Exception {

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String credentialOffers = Anoncreds.proverGetCredentialOffers(wallet, filter).get();
		JSONArray credentialOffersArray = new JSONArray(credentialOffers);

		assertEquals(2, credentialOffersArray.length());
		assertEquals(credentialOffersArray.getJSONObject(0).getString("issuer_did"), issuerDid);
		assertEquals(credentialOffersArray.getJSONObject(1).getString("issuer_did"), issuerDid);
	}

	@Test
	public void testsProverGetCredentialOffersWorksForFilterBySchemaId() throws Exception {

		String filter = String.format("{\"schema_id\":\"%s\"}", gvtSchemaId);

		String credentialOffers = Anoncreds.proverGetCredentialOffers(wallet, filter).get();
		JSONArray credentialOffersArray = new JSONArray(credentialOffers);

		assertEquals(2, credentialOffersArray.length());
		assertEquals(credentialOffersArray.getJSONObject(0).getString("cred_def_id"), issuer1gvtCredDefId);
		assertEquals(credentialOffersArray.getJSONObject(1).getString("cred_def_id"), issuer2gvtCredDefId);
	}

	@Test
	public void testsProverGetCredentialOffersWorksForFilterBySchemaName() throws Exception {

		String filter = "{\"schema_name\":\"gvt\"}";

		String credentialOffers = Anoncreds.proverGetCredentialOffers(wallet, filter).get();
		JSONArray credentialOffersArray = new JSONArray(credentialOffers);

		assertEquals(2, credentialOffersArray.length());
		assertEquals(credentialOffersArray.getJSONObject(0).getString("cred_def_id"), issuer1gvtCredDefId);
		assertEquals(credentialOffersArray.getJSONObject(1).getString("cred_def_id"), issuer2gvtCredDefId);
	}

	@Test
	public void testsProverGetCredentialOffersWorksForFilterByCredDefId() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"cred_def_id\":\"%s\"}", issuer2gvtCredDefId);

		String credentialOffers = Anoncreds.proverGetCredentialOffers(wallet, filter).get();
		JSONArray credentialOffersArray = new JSONArray(credentialOffers);

		assertEquals(1, credentialOffersArray.length());
		assertEquals(credentialOffersArray.getJSONObject(0).getString("cred_def_id"), issuer2gvtCredDefId);
	}

	@Test
	public void testsProverGetCredentialOffersWorksForNoResult() throws Exception {

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid + "a");

		String credentialOffers = Anoncreds.proverGetCredentialOffers(wallet, filter).get();
		JSONArray credentialOffersArray = new JSONArray(credentialOffers);

		assertEquals(0, credentialOffersArray.length());
	}

	@Test
	public void testsProverGetCredentialOffersWorksForInvalidFilterJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String filter = "gvt";

		Anoncreds.proverGetCredentialOffers(wallet, filter).get();
	}

	@Test
	public void testGetCredentialOffersForPlugged() throws Exception {
		String walletName = "proverCustomWallet";

		Wallet.registerWalletType("proverInmem", new InMemWalletType()).get();

		Wallet.createWallet("default", walletName, "proverInmem", null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		Anoncreds.proverStoreCredentialOffer(wallet, issuer1GvtCredOffer).get();
		Anoncreds.proverStoreCredentialOffer(wallet, issuer1XyzCredOffer).get();
		Anoncreds.proverStoreCredentialOffer(wallet, issuer2GvtCredOffer).get();

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String credentialOffers = Anoncreds.proverGetCredentialOffers(wallet, filter).get();
		JSONArray credentialOffersArray = new JSONArray(credentialOffers);

		assertEquals(2, credentialOffersArray.length());
		assertEquals(credentialOffersArray.getJSONObject(0).getString("issuer_did"), issuerDid);
		assertEquals(credentialOffersArray.getJSONObject(1).getString("issuer_did"), issuerDid);
	}
}
