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
	int gvtSchemaSeqNo = 1;
	int xyzSchemaSeqNo = 2;
	String claimOfferTemplate = "{\"issuer_did\":\"%s\",\"schema_seq_no\":%d}";
	String schema = "{\"seqNo\":1,\"data\": {\"name\":\"gvt\",\"version\":\"1.0\",\"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]}}";
	String claimRequestTemplate = "{\n" +
			"            \"blinded_ms\":{\n" +
			"                \"u\":\"72052674960029442327236458752017934128206007798774128392572211954456711136771871346204637748253860917837147111221378456345006764308173447177933384497678611527908801900335623480700015849806575534757455484512742315652166882850683721692964547448843598104385874050447011820051099399087175505815748958014671544911179795524159951193233504921329404534187047046492036161628814022862661479869322137573048331473599346645871295570237032991261433025344456232326409789544299441933427561947291495434188942844516539974096858281005872862193803356400358925349350554630231733687344283622639185011395343616612151755685912869590344206893\",\n" +
			"                \"ur\":null\n" +
			"            },\n" +
			"            \"prover_did\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\",\n" +
			"            \"issuer_did\":\"%s\",\n" +
			"            \"schema_seq_no\":%d\n" +
			"        }";

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

		String claimOffer1 = String.format(claimOfferTemplate, issuerDid, gvtSchemaSeqNo);
		String claimOffer2 = String.format(claimOfferTemplate, issuerDid, xyzSchemaSeqNo);
		String claimOffer3 = String.format(claimOfferTemplate, issuerDid2, gvtSchemaSeqNo);

		Anoncreds.proverStoreClaimOffer(wallet, claimOffer1).get();
		Anoncreds.proverStoreClaimOffer(wallet, claimOffer2).get();
		Anoncreds.proverStoreClaimOffer(wallet, claimOffer3).get();

		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		//Issue GVT claim bu Issuer1
		claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, null, false).get();

		String claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", claimOffer1, claimDef, masterSecretName).get();

		String claim = "{" +
				"           \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"           \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"           \"height\":[\"175\",\"175\"],\n" +
				"           \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimJson).get();

		//Issue XYZ claim bu Issuer1
		String xyz_schema = "{\"seqNo\":2,\"data\": {\"name\":\"gvt\",\"version\":\"1.0\",\"attr_names\":[\"status\",\"period\"]}}";
		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, xyz_schema, null, false).get();

		claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", claimOffer2, claimDef, masterSecretName).get();

		claim = "{" +
				"   \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
				"   \"period\":[\"8\",\"8\"]\n" +
				"}";

		createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
		claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimJson).get();


		//Issue GVT claim bu Issuer2
		claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid2, schema, null, false).get();

		claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", claimOffer3, claimDef, masterSecretName).get();

		claim = "{" +
			"           \"sex\":[\"male\",\"2142657394558967239210949258394838228692050081607692519917028371144233115103\"],\n" +
			"           \"name\":[\"Alexander\",\"21332817548165488690172217217278169335\"],\n" +
			"           \"height\":[\"170\",\"170\"],\n" +
			"           \"age\":[\"28\",\"28\"]\n" +
			"   }";

		createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
		claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimJson).get();

		walletOpened = true;
	}
}
