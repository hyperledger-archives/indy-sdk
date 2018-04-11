package org.hyperledger.indy.sdk;

import org.hyperledger.indy.sdk.crypto.CryptoJSONParameters;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.did.DidJSONParameters;
import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.wallet.InMemWalletType;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.junit.After;
import org.junit.Before;
import org.junit.Rule;
import org.junit.rules.ExpectedException;
import org.junit.rules.Timeout;

import java.io.IOException;
import java.util.HashSet;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

import static org.hyperledger.indy.sdk.utils.EnvironmentUtils.getIndyHomePath;

public class IndyIntegrationTest {

	protected static final String TRUSTEE_SEED = "000000000000000000000000Trustee1";
	protected static final String MY1_SEED = "00000000000000000000000000000My1";
	protected static final String MY2_SEED = "00000000000000000000000000000My2";
	protected static final String VERKEY = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	protected static final String VERKEY_MY1 = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
	protected static final String VERKEY_MY2 = "kqa2HyagzfMAq42H5f9u3UMwnSBPQx2QfrSyXbUPxMn";
	protected static final String VERKEY_TRUSTEE = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
	protected static final String INVALID_VERKEY = "CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW";
	protected static final String DID = "8wZcEriaNLNKtteJvx7f8i";
	protected static final String DID_MY1 = "VsKV7grR1BUE29mG2Fm2kX";
	protected static final String DID_MY2 = "2PRyVHmkXQnQzJQKxHxnXC";
	protected static final String DID_TRUSTEE = "V4SGRU86Z58d6TV7PBUe6f";
	protected static final String INVALID_DID = "invalid_base58string";
	protected static final String IDENTITY_JSON_TEMPLATE = "{\"did\":\"%s\",\"verkey\":\"%s\"}";
	protected static final byte[] MESSAGE = "{\"reqId\":1496822211362017764}".getBytes();
	protected static final String SCHEMA_DATA = "{\"id\":\"id\", \"name\":\"gvt\",\"version\":\"1.0\",\"attrNames\":[\"name\"],\"ver\":\"1.0\"}";
	protected static final String POOL = "Pool1";
	protected static final String WALLET = "Wallet1";
	protected static final String TYPE = "default";
	protected static final String METADATA = "some metadata";
	protected static final String ENDPOINT = "127.0.0.1:9700";
	protected static final String CRYPTO_TYPE = "ed25519";
	protected byte[] SIGNATURE = {20, - 65, 100, - 43, 101, 12, - 59, - 58, - 53, 49, 89, - 36, - 51, - 64, - 32, - 35, 97, 77, - 36, - 66, 90, 60, - 114, 23, 16, - 16, - 67, - 127, 45, - 108, - 11, 8, 102, 95, 95, - 7, 100, 89, 41, - 29, - 43, 25, 100, 1, - 24, - 68, - 11, - 21, - 70, 21, 52, - 80, - 20, 11, 99, 70, - 101, - 97, 89, - 41, - 59, - 17, - 118, 5};
	protected byte[] ENCRYPTED_MESSAGE = {- 105, 30, 89, 75, 76, 28, - 59, - 45, 105, - 46, 20, 124, - 85, - 13, 109, 29, - 88, - 82, - 8, - 6, - 50, - 84, - 53, - 48, - 49, 56, 124, 114, 82, 126, 74, 99, - 72, - 78, - 117, 96, 60, 119, 50, - 40, 121, 21, 57, - 68, 89};
	protected byte[] NONCE = {- 14, 102, - 41, - 57, 1, 4, 75, - 46, - 91, 87, 14, 41, - 39, 48, 42, - 126, - 121, 84, - 58, 59, - 27, 51, - 32, - 23};
	protected String DEFAULT_CRED_DEF_CONFIG = "{\"support_revocation\":false}";
	protected String TAG = "tag1";
	protected String GVT_SCHEMA_NAME = "gvt";
	protected String XYZ_SCHEMA_NAME = "xyz";
	protected String SCHEMA_VERSION = "1.0";
	protected String GVT_SCHEMA_ATTRIBUTES = "[\"name\", \"age\", \"sex\", \"height\"]";
	protected String XYZ_SCHEMA_ATTRIBUTES = "[\"status\", \"period\"]";
	protected String REVOC_REG_TYPE = "CL_ACCUM";
	protected String SIGNATURE_TYPE = "CL";
	protected String TAILS_WRITER_CONFIG = new JSONObject(String.format("{\"base_dir\":\"%s\", \"uri_pattern\":\"\"}", getIndyHomePath("tails")).replace('\\', '/')).toString();
	protected String REV_CRED_DEF_CONFIG = "{\"support_revocation\":true}";
	protected String GVT_CRED_VALUES = "{\n" +
			"        \"sex\": {\"raw\": \"male\", \"encoded\": \"5944657099558967239210949258394887428692050081607692519917050\"},\n" +
			"        \"name\": {\"raw\": \"Alex\", \"encoded\": \"1139481716457488690172217916278103335\"},\n" +
			"        \"height\": {\"raw\": \"175\", \"encoded\": \"175\"},\n" +
			"        \"age\": {\"raw\": \"28\", \"encoded\": \"28\"}\n" +
			"    }";


	protected static final String TRUSTEE_IDENTITY_JSON =
			new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null).toJson();

	protected static final String MY1_IDENTITY_JSON =
			new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, null).toJson();

	protected static final String MY1_IDENTITY_KEY_JSON =
			new CryptoJSONParameters.CreateKeyJSONParameter(MY1_SEED, null).toJson();


	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

	private static Boolean isWalletRegistered = false;

	@Before
	public void setUp() throws IOException, InterruptedException, ExecutionException, IndyException, Exception {
		InitHelper.init();
		StorageUtils.cleanupStorage();
		if (! isWalletRegistered) {
			Wallet.registerWalletType("inmem", new InMemWalletType()).get();
		}
		isWalletRegistered = true;
	}

	protected HashSet<Pool> openedPools = new HashSet<>();

	@After
	public void tearDown() throws IOException {
		openedPools.forEach(pool -> {
			try {
				pool.closePoolLedger().get();
			} catch (IndyException | InterruptedException | ExecutionException ignore) {
			}
		});
		openedPools.clear();
		StorageUtils.cleanupStorage();
	}
}
