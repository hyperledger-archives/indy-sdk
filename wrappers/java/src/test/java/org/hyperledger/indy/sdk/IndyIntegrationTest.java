package org.hyperledger.indy.sdk;

import org.hyperledger.indy.sdk.crypto.CryptoJSONParameters;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.wallet.InMemWalletType;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Rule;
import org.junit.rules.ExpectedException;
import org.junit.rules.Timeout;

import java.io.IOException;
import java.util.HashSet;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

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
	protected static final String INVALID_DID = "invalid_base58string";
	protected static final String IDENTITY_JSON_TEMPLATE = "{\"did\":\"%s\",\"verkey\":\"%s\"}";
	protected static final byte[] MESSAGE = "{\"reqId\":1496822211362017764}".getBytes();
	protected static final String SCHEMA_DATA = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"attr_names\": [\"name\", \"male\"]}";
	protected static final String POOL = "Pool1";
	protected static final String WALLET = "Wallet1";
	protected static final String TYPE = "default";
	protected static final String METADATA = "some metadata";
	protected static final String ENDPOINT = "127.0.0.1:9700";
	protected static final String CRYPTO_TYPE = "ed25519";
	protected byte[] SIGNATURE = {- 87, - 41, 8, - 31, 7, 107, 110, 9, - 63, - 94, - 54, - 42, - 94, 66, - 18, - 45, 63, - 47, 12, - 60, 8, - 45, 55, 27, 120, 94, - 52, - 109, 53, 104,
			103, 61, 60, - 7, - 19, 127, 103, 46, - 36, - 33, 10, 95, 75, 53, - 11, - 46, - 15, - 105, - 65, 41, 48, 30, 9, 16, 78, - 4, - 99, - 50, - 46, - 111, 125, - 123, 109, 11};
	protected byte[] ENCRYPTED_MESSAGE = {- 105, 30, 89, 75, 76, 28, - 59, - 45, 105, - 46, 20, 124, - 85, - 13, 109, 29, - 88, - 82, - 8, - 6, - 50, - 84, - 53, - 48, - 49, 56, 124, 114, 82, 126, 74, 99, - 72, - 78, - 117, 96, 60, 119, 50, - 40, 121, 21, 57, - 68, 89};
	protected byte[] NONCE = {- 14, 102, - 41, - 57, 1, 4, 75, - 46, - 91, 87, 14, 41, - 39, 48, 42, - 126, - 121, 84, - 58, 59, - 27, 51, - 32, - 23};


	protected static final String TRUSTEE_IDENTITY_JSON =
			new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null).toJson();

	protected static final String MY1_IDENTITY_JSON =
			new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, null).toJson();

	protected static final String MY1_IDENTITY_KEY_JSON =
			new CryptoJSONParameters.CreateKeyJSONParameter(MY1_SEED, null).toJson();


	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

	private static Boolean isWalletRegistered = false;

	@Before
	public void setUp() throws IOException, InterruptedException, ExecutionException, IndyException {
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
