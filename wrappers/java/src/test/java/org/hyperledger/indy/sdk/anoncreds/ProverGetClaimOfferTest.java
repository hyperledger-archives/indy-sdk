package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.InMemWalletType;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

public class ProverGetClaimOfferTest extends AnoncredsIntegrationTest {

	@Test
	public void testsProverGetClaimOffersWorksForEmptyFilter() throws Exception {

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, "{}").get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(3, claimOffersArray.length());
	}

	@Test
	public void testsProverGetClaimOffersWorksForFilterByIssuer() throws Exception {

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(2, claimOffersArray.length());
		assertEquals(claimOffersArray.getJSONObject(0).getString("issuer_did"), issuerDid);
		assertEquals(claimOffersArray.getJSONObject(1).getString("issuer_did"), issuerDid);
	}

	@Test
	public void testsProverGetClaimOffersWorksForFilterBySchemaId() throws Exception {

		String filter = String.format("{\"schema_id\":\"%s\"}", gvtSchemaId);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(2, claimOffersArray.length());
		assertEquals(claimOffersArray.getJSONObject(0).getString("cred_def_id"), issuer1gvtClaimDefId);
		assertEquals(claimOffersArray.getJSONObject(1).getString("cred_def_id"), issuer2gvtClaimDefId);
	}

	@Test
	public void testsProverGetClaimOffersWorksForFilterBySchemaName() throws Exception {

		String filter = "{\"schema_name\":\"gvt\"}";

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(2, claimOffersArray.length());
		assertEquals(claimOffersArray.getJSONObject(0).getString("cred_def_id"), issuer1gvtClaimDefId);
		assertEquals(claimOffersArray.getJSONObject(1).getString("cred_def_id"), issuer2gvtClaimDefId);
	}

	@Test
	public void testsProverGetClaimOffersWorksForFilterByCredDefId() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"cred_def_id\":\"%s\"}", issuer2gvtClaimDefId);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(1, claimOffersArray.length());
		assertEquals(claimOffersArray.getJSONObject(0).getString("cred_def_id"), issuer2gvtClaimDefId);
	}

	@Test
	public void testsProverGetClaimOffersWorksForNoResult() throws Exception {

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid + "a");

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(0, claimOffersArray.length());
	}

	@Test
	public void testsProverGetClaimOffersWorksForInvalidFilterJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String filter = "gvt";

		Anoncreds.proverGetClaimOffers(wallet, filter).get();
	}

	@Test
	public void testGetClaimOffersForPlugged() throws Exception {
		String walletName = "proverCustomWallet";

		Wallet.registerWalletType("proverInmem", new InMemWalletType()).get();

		Wallet.createWallet("default", walletName, "proverInmem", null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		Anoncreds.proverStoreClaimOffer(wallet, issuer1GvtClaimOffer).get();
		Anoncreds.proverStoreClaimOffer(wallet, issuer1XyzClaimOffer).get();
		Anoncreds.proverStoreClaimOffer(wallet, issuer2GvtClaimOffer).get();

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(2, claimOffersArray.length());
		assertEquals(claimOffersArray.getJSONObject(0).getString("issuer_did"), issuerDid);
		assertEquals(claimOffersArray.getJSONObject(1).getString("issuer_did"), issuerDid);
	}
}
