package org.hyperledger.indy.sdk;

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
	protected static final String DID1 = "8wZcEriaNLNKtteJvx7f8i";
	protected static final String IDENTITY_JSON_TEMPLATE = "{\"did\":\"%s\",\"verkey\":\"%s\"}";
	protected static final byte[] MESSAGE = "{\"reqId\":1496822211362017764}".getBytes();
	protected static final String SCHEMA_DATA = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"attr_names\": [\"name\", \"male\"]}";
	protected static final String POOL = "Pool1";
	protected static final String WALLET = "Wallet1";
	protected static final String TYPE = "default";
	protected static final String TRUSTEE_IDENTITY_JSON =
			new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null).toJson();
	protected static final String VERKEY = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	protected static final String VERKEY_FOR_MY1_SEED = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
	protected static final String VERKEY_FOR_MY2_SEED = "kqa2HyagzfMAq42H5f9u3UMwnSBPQx2QfrSyXbUPxMn";
	protected static final String DID_FOR_MY1_SEED = "VsKV7grR1BUE29mG2Fm2kX";
	protected static final String DID_FOR_MY2_SEED = "2PRyVHmkXQnQzJQKxHxnXC";
	protected static final String METADATA = "some metadata";
	protected static final String ENDPOINT = "127.0.0.1:9700";
	protected static final String INVALID_VERKEY = "CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW";
	protected static final String CRYPTO_TYPE = "ed25519";


	protected static final String MY1_IDENTITY_JSON =
			new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, null).toJson();

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
