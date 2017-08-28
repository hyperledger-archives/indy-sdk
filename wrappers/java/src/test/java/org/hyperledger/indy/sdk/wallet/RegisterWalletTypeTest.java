package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.json.JSONArray;
import org.junit.Ignore;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import static org.junit.Assert.*;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

public class RegisterWalletTypeTest extends IndyIntegrationTest {

	@Test
	@Ignore //The wallet is already registered by the base class!
	public void testRegisterWalletTypeWorks() throws Exception {

		Wallet.registerWalletType("inmem", new InMemWalletType()).get();
	}

	@Test
	public void testRegisterWalletTypeDoesNotWorkForTwiceWithSameName() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletTypeAlreadyRegisteredError));

		Wallet.registerWalletType("inmem", new InMemWalletType()).get();
	}

	@Rule
	public Timeout globalTimeout = new Timeout(2, TimeUnit.MINUTES);

	@Test
	public void customWalletWorkoutTest() throws Exception {

		StorageUtils.cleanupStorage();

		String walletName = "inmemWorkoutWallet";

		Wallet.createWallet("default", walletName, "inmem", null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
		String schema = "{\"seqNo\":1,\"data\": {\"name\":\"gvt\",\"version\":\"1.0\",\"keys\":[\"age\",\"sex\",\"height\",\"name\"]}}";
		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, null, false).get();

		String claimOfferTemplate = "{\"issuer_did\":\"%s\",\"schema_seq_no\":%d}";
		Anoncreds.proverStoreClaimOffer(wallet, String.format(claimOfferTemplate, issuerDid, 1)).get();
		Anoncreds.proverStoreClaimOffer(wallet, String.format(claimOfferTemplate, issuerDid, 2)).get();
		String issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
		Anoncreds.proverStoreClaimOffer(wallet, String.format(claimOfferTemplate, issuerDid2, 2)).get();

		String masterSecretName = "master_secret_name";
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		String claimOffer = String.format("{\"issuer_did\":\"%s\",\"schema_seq_no\":%d}", issuerDid, 1);

		String claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", claimOffer, claimDef, masterSecretName).get();

		String claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"                 \"height\":[\"175\",\"175\"],\n" +
				"                 \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimJson).get();

		String filter = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		String claims = Anoncreds.proverGetClaims(wallet, filter).get();

		JSONArray claimsArray = new JSONArray(claims);

		assertEquals(1, claimsArray.length());
	}
}