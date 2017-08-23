package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.*;

import java.util.concurrent.ExecutionException;

public class ProverStoreClaimTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverStoreClaimWorks() throws Exception {
		initCommonWallet();

		String proverWalletName = "proverWallet";
		Wallet.createWallet("default", proverWalletName, "default", null, null).get();
		Wallet proverWallet = Wallet.openWallet(proverWalletName, null, null).get();

		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecretName).get();

		String claimOffer = String.format(claimOfferTemplate, issuerDid, 1);

		String claimRequest = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, claimOffer, claimDef, masterSecretName).get();

		String claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"                 \"height\":[\"175\",\"175\"],\n" +
				"                 \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(proverWallet, claimJson).get();

		proverWallet.closeWallet().get();
		Wallet.deleteWallet(proverWalletName, null).get();
	}

	@Test
	public void testProverStoreClaimWorksWithoutClaimReq() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		String claimJson = String.format("{\"claim\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
				"                          \"issuer_did\":\"%s\",\n" +
				"                          \"revoc_reg_seq_no\":null,\n" +
				"                          \"schema_seq_no\":2,\n" +
				"                          \"signature\":{\"primary_claim\":{\"m2\":\"1\",\"a\":\"1\",\"e\":\"2\",\"v\":\"3\"}," +
				"                          \"non_revocation_claim\":null}}", issuerDid2);

		Anoncreds.proverStoreClaim(wallet, claimJson).get();
	}

	@Test
	public void testProverStoreClaimWorksForInvalidClaimJson() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String claimOffer = String.format(claimOfferTemplate, issuerDid, 1);

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOffer, claimDef, masterSecretName).get();

		String claimJson = "{\"claim\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
				"            \"issuer_did\":1,\"\n" +
				"            \"revoc_reg_seq_no\":null,\n" +
				"            \"schema_seq_no\":1}";

		Anoncreds.proverStoreClaim(wallet, claimJson).get();
	}
}
