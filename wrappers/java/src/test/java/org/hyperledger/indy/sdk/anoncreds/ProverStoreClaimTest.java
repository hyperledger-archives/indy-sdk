package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.*;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverStoreClaimTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverStoreClaimWorks() throws Exception {
		initCommonWallet();

		String proverWalletName = "proverWallet";
		Wallet.createWallet("default", proverWalletName, "default", null, null).get();
		Wallet proverWallet = Wallet.openWallet(proverWalletName, null, null).get();

		Anoncreds.proverCreateMasterSecret(proverWallet, masterSecretName).get();

		String claimRequest = Anoncreds.proverCreateAndStoreClaimReq(proverWallet, proverDid, gvtClaimOffer, claimDef, masterSecretName).get();

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, gvtClaimValuesJson, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(proverWallet, claimJson, null).get();

		proverWallet.closeWallet().get();
		Wallet.deleteWallet(proverWalletName, null).get();
	}

	@Test
	public void testProverStoreClaimWorksWithoutClaimReq() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		String claimJson = String.format("{\"values\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
				"                          \"issuer_did\":\"%s\",\n" +
				"                          \"rev_reg_seq_no\":null,\n" +
				"                          \"schema_key\":%s,\n" +
				"                          \"signature\":{\"p_claim\":{\"m_2\":\"1\",\"a\":\"1\",\"e\":\"2\",\"v\":\"3\"}," +
				"                          \"r_claim\":null}}", issuerDid2, xyzSchemaKey);

		Anoncreds.proverStoreClaim(wallet, claimJson, null).get();
	}

	@Test
	public void testProverStoreClaimWorksForInvalidClaimJson() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimOffer = String.format(claimOfferTemplate, issuerDid, 1);

		Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, claimOffer, claimDef, masterSecretName).get();

		String claimJson = "{\"claim\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
				"            \"issuer_did\":1,\"\n" +
				"            \"revoc_reg_seq_no\":null,\n" +
				"            \"schema_key\":1}";

		Anoncreds.proverStoreClaim(wallet, claimJson, null).get();
	}
}
