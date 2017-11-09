package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.*;
import org.junit.rules.ExpectedException;
import org.junit.rules.Timeout;

import java.util.concurrent.TimeUnit;

public class AnoncredsIntegrationTest {

	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Rule
	public Timeout globalTimeout = new Timeout(2, TimeUnit.MINUTES);

	static Wallet wallet;
	static String claimDef;

	private static Boolean walletOpened = false;

	String masterSecretName = "master_secret_name";
	String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
	String issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	String proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	String claimOfferTemplate = "{\"issuer_did\":\"%s\",\"schema_seq_no\":%d}";
	String schema = "{\"seqNo\":1,\"data\": {\"name\":\"gvt\",\"version\":\"1.0\",\"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]}}";
	String claimRequestTemplate = "{\"blinded_ms\":" +
			"{\"prover_did\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"," +
			"\"u\":\"54172737564529332710724213139048941083013176891644677117322321823630308734620627329227591845094100636256829761959157314784293939045176621327154990908459072821826818718739696323299787928173535529024556540323709578850706993294234966440826690899266872682790228513973999212370574548239877108511283629423807338632435431097339875665075453785141722989098387895970395982432709011505864533727415552566715069675346220752584449560407261446567731711814188836703337365986725429656195275616846543535707364215498980750860746440672050640048215761507774996460985293327604627646056062013419674090094698841792968543317468164175921100038\"," +
			"\"ur\":null}," +
			"\"issuer_did\":\"%s\",\"schema_seq_no\":%d}";

	@BeforeClass
	public static void  setUp() throws Exception{
		InitHelper.init();
	}
	
	@AfterClass
	public static void cleanUp() throws Exception{
		
		if (walletOpened){
			wallet.closeWallet().get();			
			walletOpened = false;
		}
	}

	void initCommonWallet() throws Exception {

		if (walletOpened) {
			return;
		}

		StorageUtils.cleanupStorage();

		String walletName = "anoncredsCommonWallet";

		Wallet.createWallet("default", walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();

		claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, null, false).get();

		Anoncreds.proverStoreClaimOffer(wallet, String.format(claimOfferTemplate, issuerDid, 1)).get();
		Anoncreds.proverStoreClaimOffer(wallet, String.format(claimOfferTemplate, issuerDid, 2)).get();
		Anoncreds.proverStoreClaimOffer(wallet, String.format(claimOfferTemplate, issuerDid2, 2)).get();

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

		walletOpened = true;
	}
}
