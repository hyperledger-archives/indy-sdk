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
	static String gvtSchemaId;
	static String gvtSchemaJson;
	static String xyzSchemaId;
	static String xyzSchemaJson;
	static String issuer1gvtCredDefId;
	static String issuer2gvtCredDefId;
	static String issuer1xyzCredDef;
	static String issuer1gvtCredDef;
	static String issuer1GvtCredOffer;
	static String issuer1XyzCredOffer;
	static String issuer2GvtCredOffer;
	static String credentialRequest;
	static String credential;
	String masterSecretName = "master_secret_name";
	String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
	String proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	String defaultCredentialDefConfig = "{\"support_revocation\":false}";
	String tag = "tag1";
	String gvtSchemaName = "gvt";
	String schemaVersion = "1.0";
	String gvtSchemaAttributes = "[\"name\", \"age\", \"sex\", \"height\"]";
	String credentialId1 = "id1";
	String credentialId2 = "id2";
	String gvtCredentialValuesJson = "{\n" +
			"               \"sex\":{\"raw\":\"male\",\"encoded\":\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"},\n" +
			"               \"name\":{\"raw\":\"Alex\",\"encoded\":\"1139481716457488690172217916278103335\"},\n" +
			"               \"height\":{\"raw\":\"175\",\"encoded\":\"175\"},\n" +
			"               \"age\":{\"raw\":\"28\",\"encoded\":\"28\"}\n" +
			"        }";
	String xyzCredentialValuesJson = "{\n" +
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
	String requestedCredentialsJsonTemplate = "{" +
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

		//Issue GVT credential by Issuer1
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult IssuerCreateAndStoreCredentialDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, gvtSchemaJson, tag, null, defaultCredentialDefConfig).get();
		issuer1gvtCredDefId = IssuerCreateAndStoreCredentialDefResult.getCredDefId();
		issuer1gvtCredDef = IssuerCreateAndStoreCredentialDefResult.getCredDefJson();

		//Issue XYZ credential by Issuer1
		IssuerCreateAndStoreCredentialDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, xyzSchemaJson, tag, null, defaultCredentialDefConfig).get();
		String issuer1xyzCredDefId = IssuerCreateAndStoreCredentialDefResult.getCredDefId();
		issuer1xyzCredDef = IssuerCreateAndStoreCredentialDefResult.getCredDefJson();

		//Issue GVT credential by Issuer2
		String issuerDid2 = "VsKV7grR1BUE29mG2Fm2kX";
		IssuerCreateAndStoreCredentialDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid2, gvtSchemaJson, tag, null, defaultCredentialDefConfig).get();
		issuer2gvtCredDefId = IssuerCreateAndStoreCredentialDefResult.getCredDefId();
		String issuer2gvtCredDef = IssuerCreateAndStoreCredentialDefResult.getCredDefJson();

		issuer1GvtCredOffer = Anoncreds.issuerCreateCredentialOffer(wallet, issuer1gvtCredDefId, issuerDid, proverDid).get();
		issuer1XyzCredOffer = Anoncreds.issuerCreateCredentialOffer(wallet, issuer1xyzCredDefId, issuerDid, proverDid).get();
		issuer2GvtCredOffer = Anoncreds.issuerCreateCredentialOffer(wallet, issuer2gvtCredDefId, issuerDid2, proverDid).get();

		Anoncreds.proverStoreCredentialOffer(wallet, issuer1GvtCredOffer).get();
		Anoncreds.proverStoreCredentialOffer(wallet, issuer1XyzCredOffer).get();
		Anoncreds.proverStoreCredentialOffer(wallet, issuer2GvtCredOffer).get();

		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		credentialRequest = Anoncreds.proverCreateAndStoreCredentialReq(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretName).get();

		AnoncredsResults.IssuerCreateCredentialResult createCredentialResult = Anoncreds.issuerCreateCredentail(wallet, credentialRequest, gvtCredentialValuesJson, null, - 1, - 1).get();
		credential = createCredentialResult.getCredentialJson();

		Anoncreds.proverStoreCredential(wallet, credentialId1, credential, null).get();

		String xyzCredentialRequest = Anoncreds.proverCreateAndStoreCredentialReq(wallet, proverDid, issuer1XyzCredOffer, issuer1xyzCredDef, masterSecretName).get();

		createCredentialResult = Anoncreds.issuerCreateCredentail(wallet, xyzCredentialRequest, xyzCredentialValuesJson, null, - 1, - 1).get();
		String issuer1XyzCredentialJson = createCredentialResult.getCredentialJson();

		Anoncreds.proverStoreCredential(wallet, credentialId2, issuer1XyzCredentialJson, null).get();

		String issuer2GvtCredentialRequest = Anoncreds.proverCreateAndStoreCredentialReq(wallet, proverDid, issuer2GvtCredOffer, issuer2gvtCredDef, masterSecretName).get();

		String gvt2CredValues = "{" +
				"           \"sex\":{\"raw\":\"male\",\"encoded\":\"2142657394558967239210949258394838228692050081607692519917028371144233115103\"},\n" +
				"           \"name\":{\"raw\":\"Alexander\",\"encoded\":\"21332817548165488690172217217278169335\"},\n" +
				"           \"height\":{\"raw\":\"170\",\"encoded\":\"170\"},\n" +
				"           \"age\":{\"raw\":\"28\",\"encoded\":\"28\"}\n" +
				"   }";

		createCredentialResult = Anoncreds.issuerCreateCredentail(wallet, issuer2GvtCredentialRequest, gvt2CredValues, null, - 1, - 1).get();
		issuer1XyzCredentialJson = createCredentialResult.getCredentialJson();

		String credentialId3 = "id3";
		Anoncreds.proverStoreCredential(wallet, credentialId3, issuer1XyzCredentialJson, null).get();

		walletOpened = true;
	}
}
