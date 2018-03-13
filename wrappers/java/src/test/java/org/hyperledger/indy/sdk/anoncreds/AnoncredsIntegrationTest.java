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

	private static Boolean walletOpened = false;

	static Wallet wallet;
	static String issuer1gvtClaimDefId;
	static String issuer2gvtClaimDefId;
	static String issuer1xyzClaimDef;
	static String issuer1gvtClaimDef;
	static String issuer1GvtClaimOffer;
	static String issuer1XyzClaimOffer;
	static String issuer2GvtClaimOffer;
	static String claimRequest;
	static String claim;
	static String gvtSchemaId;
	static String gvtSchemaJson;
	static String xyzSchemaId;
	static String xyzSchemaJson;
	String masterSecretName = "master_secret_name";
	String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
	String proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	String defaultCredentialDefConfig = "{\"support_revocation\":false}";
	String tag = "tag1";
	String gvtSchemaName = "gvt";
	String schemaVersion = "1.0";
	String gvtSchemaAttributes = "[\"name\", \"age\", \"sex\", \"height\"]";
	String claimId1 = "id1";
	String claimId2 = "id2";
	String gvtClaimValuesJson = "{\n" +
			"               \"sex\":{\"raw\":\"male\",\"encoded\":\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"},\n" +
			"               \"name\":{\"raw\":\"Alex\",\"encoded\":\"1139481716457488690172217916278103335\"},\n" +
			"               \"height\":{\"raw\":\"175\",\"encoded\":\"175\"},\n" +
			"               \"age\":{\"raw\":\"28\",\"encoded\":\"28\"}\n" +
			"        }";
	String xyzClaimValuesJson = "{\n" +
			"               \"status\":{\"raw\":\"partial\",\"encoded\":\"51792877103171595686471452153480627530895\"},\n" +
			"               \"period\":{\"raw\":\"8\",\"encoded\":\"8\"}\n" +
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
			"\"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true}}," +
			"\"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\"}}" +
			"}";

	@Before
	public void setUp() throws Exception {
		InitHelper.init();
		initCommonWallet();
	}

	void initCommonWallet() throws Exception {

		if (walletOpened) {
			return;
		}

		StorageUtils.cleanupStorage();

		String walletName = "anoncredsCommonWallet";

		Wallet.createWallet("default", walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();

		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, gvtSchemaName, schemaVersion, gvtSchemaAttributes).get();
		gvtSchemaId = createSchemaResult.getSchemaId();
		gvtSchemaJson = createSchemaResult.getSchemaJson();

		String xyzSchemaAttributes = "[\"status\", \"period\"]";
		String xyzSchemaName = "xyz";
		createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, xyzSchemaName, schemaVersion, xyzSchemaAttributes).get();
		xyzSchemaId = createSchemaResult.getSchemaId();
		xyzSchemaJson = createSchemaResult.getSchemaJson();

		//Issue GVT claim by Issuer1
		AnoncredsResults.IssuerCreateAndStoreClaimDefResult issuerCreateAndStoreClaimDefResult = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, tag, null, defaultCredentialDefConfig).get();
		issuer1gvtClaimDefId = issuerCreateAndStoreClaimDefResult.getClaimDefId();
		issuer1gvtClaimDef = issuerCreateAndStoreClaimDefResult.getClaimDefJson();

		//Issue XYZ claim by Issuer1
		issuerCreateAndStoreClaimDefResult = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, xyzSchemaJson, tag, null, defaultCredentialDefConfig).get();
		String issuer1xyzClaimDefId = issuerCreateAndStoreClaimDefResult.getClaimDefId();
		issuer1xyzClaimDef = issuerCreateAndStoreClaimDefResult.getClaimDefJson();

		//Issue GVT claim by Issuer2
		String issuerDid2 = "VsKV7grR1BUE29mG2Fm2kX";
		issuerCreateAndStoreClaimDefResult = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid2, gvtSchemaJson, tag, null, defaultCredentialDefConfig).get();
		issuer2gvtClaimDefId = issuerCreateAndStoreClaimDefResult.getClaimDefId();
		String issuer2gvtClaimDef = issuerCreateAndStoreClaimDefResult.getClaimDefJson();

		issuer1GvtClaimOffer = Anoncreds.issuerCreateClaimOffer(wallet, issuer1gvtClaimDefId, issuerDid, proverDid).get();
		issuer1XyzClaimOffer = Anoncreds.issuerCreateClaimOffer(wallet, issuer1xyzClaimDefId, issuerDid, proverDid).get();
		issuer2GvtClaimOffer = Anoncreds.issuerCreateClaimOffer(wallet, issuer2gvtClaimDefId, issuerDid2, proverDid).get();

		Anoncreds.proverStoreClaimOffer(wallet, issuer1GvtClaimOffer).get();
		Anoncreds.proverStoreClaimOffer(wallet, issuer1XyzClaimOffer).get();
		Anoncreds.proverStoreClaimOffer(wallet, issuer2GvtClaimOffer).get();

		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		claimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1GvtClaimOffer, issuer1gvtClaimDef, masterSecretName).get();

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, gvtClaimValuesJson, null, - 1, - 1).get();
		claim = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimId1, claim, null).get();

		String xyzClaimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer1XyzClaimOffer, issuer1xyzClaimDef, masterSecretName).get();

		createClaimResult = Anoncreds.issuerCreateClaim(wallet, xyzClaimRequest, xyzClaimValuesJson, null, - 1, - 1).get();
		String claimJson = createClaimResult.getClaimJson();

		Anoncreds.proverStoreClaim(wallet, claimId2, claimJson, null).get();

		String gvtClaimRequest = Anoncreds.proverCreateAndStoreClaimReq(wallet, proverDid, issuer2GvtClaimOffer, issuer2gvtClaimDef, masterSecretName).get();

		String claim = "{" +
				"           \"sex\":{\"raw\":\"male\",\"encoded\":\"2142657394558967239210949258394838228692050081607692519917028371144233115103\"},\n" +
				"           \"name\":{\"raw\":\"Alexander\",\"encoded\":\"21332817548165488690172217217278169335\"},\n" +
				"           \"height\":{\"raw\":\"170\",\"encoded\":\"170\"},\n" +
				"           \"age\":{\"raw\":\"28\",\"encoded\":\"28\"}\n" +
				"   }";

		createClaimResult = Anoncreds.issuerCreateClaim(wallet, gvtClaimRequest, claim, null, - 1, - 1).get();
		claimJson = createClaimResult.getClaimJson();

		String claimId3 = "id3";
		Anoncreds.proverStoreClaim(wallet, claimId3, claimJson, null).get();

		walletOpened = true;
	}
}
