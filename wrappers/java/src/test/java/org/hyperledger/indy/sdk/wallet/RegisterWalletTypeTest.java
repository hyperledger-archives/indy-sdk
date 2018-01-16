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

		Wallet.createWallet(POOL, walletName, type, null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		String gvtSchemaKey = String.format(SCHEMA_KEY_TEMPLATE, "gvt", DID);
		String xyzSchemaKey = String.format(SCHEMA_KEY_TEMPLATE, "xyz", DID_TRUSTEE);

		String gvtSchemaJson = String.format(SCHEMA_TEMPLATE, 1, DID, "gvt", "[\"age\",\"sex\",\"height\",\"name\"]");
		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, DID, gvtSchemaJson, null, false).get();

		Anoncreds.proverStoreClaimOffer(wallet, String.format(CLAIM_OFFER_TEMPLATE, DID, gvtSchemaKey)).get();
		Anoncreds.proverStoreClaimOffer(wallet, String.format(CLAIM_OFFER_TEMPLATE, DID, xyzSchemaKey)).get();
		Anoncreds.proverStoreClaimOffer(wallet, String.format(CLAIM_OFFER_TEMPLATE, DID_TRUSTEE, gvtSchemaKey)).get();

		String masterSecretName = "master_secret_name";
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		String claimOffer = String.format(CLAIM_OFFER_TEMPLATE, DID, gvtSchemaKey);

		String claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, DID_MY1, claimOffer, claimDef, masterSecretName).get();

		String claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"                 \"height\":[\"175\",\"175\"],\n" +
				"                 \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimJson, null).get();

		String claims = Anoncreds.proverGetClaims(wallet, String.format("{\"issuer_did\":\"%s\"}", DID)).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(1, claimsArray.length());
	}
}