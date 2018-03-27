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

		// 2. Issuer creates Claim Definition
		String gvtSchemaJson = String.format(SCHEMA_TEMPLATE, 1, DID, "gvt", "[\"age\",\"sex\",\"height\",\"name\"]");
		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, DID, gvtSchemaJson, null, false).get();

		// 3. Issuer creates Claim Offer
		String claimOffer = Anoncreds.issuerCreateClaimOffer(wallet, gvtSchemaJson, DID, DID_MY1).get();

		// 4. Issuer stores Claim Offer
		Anoncreds.proverStoreClaimOffer(wallet, claimOffer).get();

		// 5. Issuer creates Master Secret
		String masterSecretName = "master_secret_name";
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		// 6. Prover creates Claim Request
		String claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, DID_MY1, claimOffer, claimDef, masterSecretName).get();

		// 7. Issuer creates Claim
		String claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"                 \"height\":[\"175\",\"175\"],\n" +
				"                 \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		// 8. Prover stores Claim
		Anoncreds.proverStoreClaim(wallet, claimJson, null).get();

		// 9. Prover gets Claim
		String claims = Anoncreds.proverGetClaims(wallet, String.format("{\"issuer_did\":\"%s\"}", DID)).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(1, claimsArray.length());
	}
}