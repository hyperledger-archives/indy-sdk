package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.InMemWalletType;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class ProverGetClaimOfferTest extends AnoncredsIntegrationTest {

	@Test
	public void testsProverGetClaimOffersWorksForEmptyFilter() throws Exception {

		initCommonWallet();

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, "{}").get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(3, claimOffersArray.length());
	}

	@Test
	public void testsProverGetClaimOffersWorksForFilterByIssuer() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(2, claimOffersArray.length());

		assertTrue(claimOffersArray.toString().contains(String.format(claimOfferTemplate, issuerDid, 1)));
		assertTrue(claimOffersArray.toString().contains(String.format(claimOfferTemplate, issuerDid, 2)));
	}

	@Test
	public void testsProverGetClaimOffersWorksForFilterBySchema() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"schema_seq_no\":%d}", gvtSchemaSeqNo);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(2, claimOffersArray.length());

		assertTrue(claimOffersArray.toString().contains(String.format(claimOfferTemplate, issuerDid, gvtSchemaSeqNo)));
		assertTrue(claimOffersArray.toString().contains(String.format(claimOfferTemplate, issuerDid2, gvtSchemaSeqNo)));
	}

	@Test
	public void testsProverGetClaimOffersWorksForFilterByIssuerAndSchema() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"issuer_did\":\"%s\",\"schema_seq_no\":%d}", issuerDid, gvtSchemaSeqNo);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(1, claimOffersArray.length());

		assertTrue(claimOffersArray.toString().contains(String.format(claimOfferTemplate, issuerDid, gvtSchemaSeqNo)));
	}

	@Test
	public void testsProverGetClaimOffersWorksForNoResult() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"schema_seq_no\":%d}", 3);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(0, claimOffersArray.length());
	}

	@Test
	public void testsProverGetClaimOffersWorksForInvalidFilterJson() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String filter = String.format("{\"schema_seq_no\":\"%d\"}", gvtSchemaSeqNo);

		Anoncreds.proverGetClaimOffers(wallet, filter).get();
	}

	@Test
	public void testGetClaimOffersForPlugged() throws Exception {
		String type = "proverInmem";
		String poolName = "default";
		String walletName = "proverCustomWallet";

		Wallet.registerWalletType(type, new InMemWalletType()).get();

		Wallet.createWallet(poolName, walletName, type, null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		String claimOffer = String.format(claimOfferTemplate, issuerDid, gvtSchemaSeqNo);
		String claimOffer2 = String.format(claimOfferTemplate, issuerDid, xyzSchemaSeqNo);
		String claimOffer3 = String.format(claimOfferTemplate, issuerDid2, gvtSchemaSeqNo);

		Anoncreds.proverStoreClaimOffer(wallet, claimOffer).get();
		Anoncreds.proverStoreClaimOffer(wallet, claimOffer2).get();
		Anoncreds.proverStoreClaimOffer(wallet, claimOffer3).get();

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);
		System.out.println(claimOffersArray);
		assertEquals(2, claimOffersArray.length());

		assertTrue(claimOffersArray.toString().contains(claimOffer));
		assertTrue(claimOffersArray.toString().contains(claimOffer2));
	}
}
