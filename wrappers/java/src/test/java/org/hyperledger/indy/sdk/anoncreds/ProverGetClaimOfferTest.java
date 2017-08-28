package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.json.JSONArray;
import org.junit.*;

import java.util.concurrent.ExecutionException;

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

		String filter = String.format("{\"schema_seq_no\":%d}", 2);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(2, claimOffersArray.length());

		assertTrue(claimOffersArray.toString().contains(String.format(claimOfferTemplate, issuerDid, 2)));
		assertTrue(claimOffersArray.toString().contains(String.format(claimOfferTemplate, issuerDid2, 2)));
	}

	@Test
	public void testsProverGetClaimOffersWorksForFilterByIssuerAndSchema() throws Exception {

		initCommonWallet();

		String filter = String.format("{\"issuer_did\":\"%s\",\"schema_seq_no\":%d}", issuerDid, 1);

		String claimOffers = Anoncreds.proverGetClaimOffers(wallet, filter).get();
		JSONArray claimOffersArray = new JSONArray(claimOffers);

		assertEquals(1, claimOffersArray.length());

		assertTrue(claimOffersArray.toString().contains(String.format(claimOfferTemplate, issuerDid, 1)));
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
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String filter = String.format("{\"schema_seq_no\":\"%d\"}", 1);

		Anoncreds.proverGetClaimOffers(wallet, filter).get();
	}
}
