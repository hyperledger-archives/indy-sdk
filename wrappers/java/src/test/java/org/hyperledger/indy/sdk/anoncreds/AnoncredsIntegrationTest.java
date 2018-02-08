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
	static String issuer1GvtClaimOffer;
	static String issuer1XyzClaimOffer;
	static String issuer2GvtClaimOffer;
	static String claimRequest;
	static String claim;

	private static Boolean walletOpened = false;

	String masterSecretName = "master_secret_name";
	String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
	String proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrT";
	private String issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	private String schemaTemplate = "{\n" +
			"                    \"seqNo\":%d,\n" +
			"                    \"dest\":\"%s\",\n" +
			"                    \"data\": {\n" +
			"                        \"name\":\"%s\",\n" +
			"                        \"version\":\"1.0\",\n" +
			"                        \"attr_names\":%s\n" +
			"                    }\n" +
			"                }";
	String gvtSchemaJson = String.format(schemaTemplate, 1, issuerDid, "gvt", "[\"age\",\"sex\",\"height\",\"name\"]");
	private String xyzSchemaJson = String.format(schemaTemplate, 2, issuerDid2, "xyz", "[\"status\",\"period\"]");
	private String schemaKeyTemplate = "{\"name\":\"%s\",\"version\":\"1.0\",\"did\":\"%s\"}";
	String gvtSchemaKey = String.format(schemaKeyTemplate, "gvt", issuerDid);
	String xyzSchemaKey = String.format(schemaKeyTemplate, "xyz", issuerDid2);
	String gvtClaimValuesJson = "{\n" +
			"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
			"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
			"               \"height\":[\"175\",\"175\"],\n" +
			"               \"age\":[\"28\",\"28\"]\n" +
			"        }";
	String xyzClaimValuesJson = "{\n" +
			"               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
			"               \"period\":[\"8\",\"8\"]\n" +
			"        }";
	String proofRequest = "{\n" +
			"                   \"nonce\":\"123432421212\",\n" +
			"                   \"name\":\"proof_req_1\",\n" +
			"                   \"version\":\"0.1\", " +
			"                   \"requested_attrs\":{" +
			"                          \"attr1_referent\":{\"name\":\"name\"}" +
			"                    },\n" +
			"                    \"requested_predicates\":{" +
			"                          \"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}" +
			"                    }" +
			"               }";
	String requestedClaimsJsonTemplate = "{" +
			"\"self_attested_attributes\":{}," +
			"\"requested_attrs\":{\"attr1_referent\":[\"%s\", true]}," +
			"\"requested_predicates\":{\"predicate1_referent\":\"%s\"}" +
			"}";

	@BeforeClass
	public static void setUp() throws Exception {
		InitHelper.init();
	}

	@AfterClass
	public static void cleanUp() throws Exception {

		if (walletOpened) {
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

		//Issue GVT claim by Issuer1
		claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, null, false).get();

		//Issue XYZ claim bu Issuer1
		String xyzClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, xyzSchemaJson, null, false).get();

		//Issue GVT claim bu Issuer2
		String gvtClaimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid2, gvtSchemaJson, null, false).get();

		issuer1GvtClaimOffer = Anoncreds.issuerCreateClaimOffer(wallet, gvtSchemaJson, issuerDid, proverDid).get();
		issuer1XyzClaimOffer = Anoncreds.issuerCreateClaimOffer(wallet, xyzSchemaJson, issuerDid, proverDid).get();
		issuer2GvtClaimOffer = Anoncreds.issuerCreateClaimOffer(wallet, gvtSchemaJson, issuerDid2, proverDid).get();

		Anoncreds.proverStoreClaimOffer(wallet, issuer1GvtClaimOffer).get();
		Anoncreds.proverStoreClaimOffer(wallet, issuer1XyzClaimOffer).get();
		Anoncreds.proverStoreClaimOffer(wallet, issuer2GvtClaimOffer).get();

		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, claimDef, masterSecretName).get();

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, gvtClaimValuesJson, - 1).get();
		claim = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claim, null).get();

		String xyzClaimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1XyzClaimOffer, xyzClaimDef, masterSecretName).get();

		createClaimResult = Anoncreds.issuerCreateClaim(wallet, xyzClaimRequest, xyzClaimValuesJson, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimJson, null).get();

		String gvtClaimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer2GvtClaimOffer, gvtClaimDef, masterSecretName).get();

		String claim = "{" +
				"           \"sex\":[\"male\",\"2142657394558967239210949258394838228692050081607692519917028371144233115103\"],\n" +
				"           \"name\":[\"Alexander\",\"21332817548165488690172217217278169335\"],\n" +
				"           \"height\":[\"170\",\"170\"],\n" +
				"           \"age\":[\"28\",\"28\"]\n" +
				"   }";

		createClaimResult = Anoncreds.issuerCreateClaim(wallet, gvtClaimRequest, claim, - 1).get();
		claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimJson, null).get();

		walletOpened = true;
	}
}
